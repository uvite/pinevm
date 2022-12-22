use std::rc::Rc;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SimpleSyntaxType, SyntaxType};
use crate::helper::{float_max2, ge1_param_i64, move_element, pine_ref_to_bool, pine_ref_to_f64, pine_ref_to_f64_series, pine_ref_to_i64, require_param, series_index, series_index2};
use crate::runtime::context::Ctx;
use crate::SimpleSyntaxType::Bool as SBool;

use crate::types::{Arithmetic, Bool, Callable, CallableFactory, Comparator, DataType, Float, ParamCollectCall, PineRef, RefData, RuntimeErr, Series, SeriesCall};
use crate::types::DataType::String;

use super::ema::series_rma;
use super::VarResult;

pub fn calc_rsi(
    s0: Float,
    length: i64,
    s1: Float,
    upwards: &mut Series<Float>,
    downwards: &mut Series<Float>,
) -> Result<(Float, Float, Float), RuntimeErr> {
    let upward = float_max2(s0.minus(s1), Some(0f64));
    let downward = float_max2(s1.minus(s0), Some(0f64));

    let rma1 = series_rma(upward, length, upwards)?;
    let rma2 = series_rma(downward, length, downwards)?;
    let rs = rma1.div(rma2);

    let res = Some(100f64).minus(Some(100f64).div(rs.add(Some(1f64))));
    Ok((res, upward, downward))
}

pub fn calc_rsi_series(s0: Float, s1: Float) -> Result<Float, RuntimeErr> {
    // rs = x / y
    // res = 100 - 100 / (1 + rs)
    let rs = s0.div(s1);
    Ok(Some(100f64).minus(Some(100f64).div(Some(1f64).add(rs))))
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrossVal<'a> {
    rev: bool,
    upwards: Series<'a, Float>,
    downwards: Series<'a, Float>,
}
// pub fn series_minus(s1:  &Series<f64>, s2: &Series<Option<f64>>, length: usize) -> Float {
//     series_index2(&*s1,length).minus(series_index2(&*s2,length))
// }

fn series_minus(
    s1: &Option<RefData<Series<Float>>>,
    s2: &Option<RefData<Series<Float>>>,
    length: usize,
) -> Float {
    //  max(high - low, abs(high - close[1]), abs(low - close[1]))
    let result=series_index(s1, length).minus(series_index(s2, length));
    result
}

// pub fn float_max(f1: Float, f2: Float, f3: Float) -> Float {
//     vec![f1.unwrap_or(0f64), f2.unwrap_or(0f64), f3.unwrap_or(0f64)]
//         .into_iter()
//         .max_by(|a, b| a.partial_cmp(b).unwrap())
// }

impl<'a> CrossVal<'a> {
    pub unsafe fn new() -> CrossVal<'a> {
        CrossVal {
            rev: true,
            upwards: Series::new(),
            downwards: Series::new(),
        }
    }

    fn process_cross(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        mut param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<bool, RuntimeErr> {
        move_tuplet!((x, y) = param);

        match _func_type.get_type(1) {
            _ => {
                let s1 = pine_ref_to_f64_series(x.clone());
                let s2 = pine_ref_to_f64_series(y.clone());
                let up = series_minus(&s1, &s2, 0);
                let down = series_minus(&s1, &s2, 1);

                let result = up.gt(Some(0f64)) && down.lt(Some(0f64));

                Ok(result)

                //Ok(PineRef::new_box(pine_ref_to_bool(result)));


            }
        }
    }
}

impl<'a> SeriesCall<'a> for CrossVal<'a> {
    fn step(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        let res = self.process_cross(_ctx, param, _func_type)?;
       // let lookahead = pine_ref_to_bool(val).unwrap_or(false);

        Ok(PineRef::new_box(res))


    }

    fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
        Box::new(self.clone())
    }
}

fn declare_var<'a>(name: &'static str,factory: fn() -> Callable<'a>) -> VarResult<'a> {
    // let value = PineRef::new(CallableFactory::new(|| {
    //     Callable::new(
    //         None,
    //         Some(Box::new(ParamCollectCall::new_with_caller(Box::new(
    //             CrossVal::new(),
    //         )))),
    //     )
    // }));

    let value = PineRef::new(CallableFactory::new(factory));


    let func_type = FunctionTypes(vec![
        FunctionType::new((
            vec![
                ("x", SyntaxType::float_series()),
                ("y", SyntaxType::float_series()),
            ],
            SyntaxType::bool()
        )),
    ]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, name)
}


pub fn declare_crossover_var<'a>() -> VarResult<'a> {

    declare_var("crossover", || unsafe {
        Callable::new(None, Some(Box::new(CrossVal::new())))
    })
}

pub fn declare_crossunder_var<'a>() -> VarResult<'a> {
    declare_var("crossunder", || {
        Callable::new(None, Some(Box::new(CrossVal::new(true))))
    })

}

#[cfg(test)]
mod tests {
    use crate::{LibInfo, PineParser, PineRunner, VarIndex};
    use crate::ast::syntax_type::SyntaxType;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::runtime::VarOperate;
    use crate::types::Series;

    use super::*;

// use crate::libs::{floor, exp, };

    #[test]
    fn rsi_int_test() {
        let lib_info = LibInfo::new(
            vec![declare_crossover_var(),declare_crossunder_var()],
            vec![("close", SyntaxType::float_series()),("high", SyntaxType::float_series())],
        );
        let src = "m = crossover(close, high)\n";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![(
                          "close",
                          AnySeries::from_float_vec(vec![Some(10f64), Some(30f64)]),
                      ),(
                          "high",
                          AnySeries::from_float_vec(vec![Some(15f64), Some(25f64)]),
                      )],
                None,
            )
            .unwrap();

        //println!("{:?}",runner.get_context().move_var(VarIndex::new(0, 0)));

        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new_box(true))
        );
    }

}
