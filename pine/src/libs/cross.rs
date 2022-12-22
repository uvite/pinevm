use super::VarResult;
use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};


use crate::helper::{check_ge1_i64, move_element, pine_ref_to_f64_series, pine_ref_to_i64, require_param, series_index, series_index2};
use crate::runtime::context::{Ctx};
use crate::types::{Arithmetic, Callable, CallableFactory, Float, ParamCollectCall, PineRef, RefData, RuntimeErr, Series};

use std::rc::Rc;



fn series_minus(
    s1: &Option<RefData<Series<Float>>>,
    s2: &Option<RefData<Series<Float>>>,
    length: usize,
) -> Float {
    //  max(high - low, abs(high - close[1]), abs(low - close[1]))
    let result=series_index(s1, length).minus(series_index(s2, length));
    result
}


fn cross_func<'a>(
    _context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
    _func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {

    move_tuplet!((x, y) = param);

    match _func_type.get_type(1) {
        _ => {
            let s1 = pine_ref_to_f64_series(x.clone());
            let s2 = pine_ref_to_f64_series(y.clone());
            let zero = series_minus(&s1, &s2, 0);
            let one = series_minus(&s1, &s2, 1);


            let res = zero.gt(&Some(0f64)) && one.lt(&Some(0f64));

            Ok(PineRef::new_box(res))

            //Ok(PineRef::new_box(pine_ref_to_bool(result)));


        }
    }

}

fn crossover_func<'a>(
    _context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
    _func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {

    move_tuplet!((x, y) = param);

    match _func_type.get_type(1) {
        _ => {
            let s1 = pine_ref_to_f64_series(x.clone());
            let s2 = pine_ref_to_f64_series(y.clone());
            let zero = series_minus(&s1, &s2, 0);
            let one = series_minus(&s1, &s2, 1);


            let res = zero.gt(&Some(0f64)) && one.lt(&Some(0f64));

            Ok(PineRef::new_box(res))

            //Ok(PineRef::new_box(pine_ref_to_bool(result)));


        }
    }

}

fn crossunder_func<'a>(
    _context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
    _func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {
    move_tuplet!((x, y) = param);

    match _func_type.get_type(1) {
        _ => {
            let s1 = pine_ref_to_f64_series(x.clone());
            let s2 = pine_ref_to_f64_series(y.clone());
            let zero = series_minus(&s1, &s2, 0);
            let one = series_minus(&s1, &s2, 1);

            let res = zero.lt(&Some(0f64)) && one.gt(&Some(0f64));

            Ok(PineRef::new_box(res))

            //Ok(PineRef::new_box(pine_ref_to_bool(result)));


        }
    }
}

fn declare_crossover_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| {
        Callable::new(
            None,
            Some(Box::new(ParamCollectCall::new_with_fns(
                Some(crossover_func),
                None,
            ))),
        )
    }));


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
    VarResult::new(value, syntax_type, "crossover")
}
fn declare_crossunder_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| {
        Callable::new(
            None,
            Some(Box::new(ParamCollectCall::new_with_fns(
                Some(crossunder_func),
                None,
            ))),
        )
    }));


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
    VarResult::new(value, syntax_type, "crossunder")
}



fn declare_cross_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| {
        Callable::new(
            None,
            Some(Box::new(ParamCollectCall::new_with_fns(
                Some(cross_func),
                None,
            ))),
        )
    }));


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
    VarResult::new(value, syntax_type, "cross")
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
        let src = "m = crossover(close, high)\nm2 =crossunder(close,high)";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![(
                          "close",
                          AnySeries::from_float_vec(vec![Some(36f64), Some(10f64)]),
                      ),(
                          "high",
                          AnySeries::from_float_vec(vec![Some(35f64), Some(25f64)]),
                      )],
                None,
            )
            .unwrap();

        //println!("{:?}",runner.get_context().move_var(VarIndex::new(0, 0)));

        assert_eq!(
            runner.get_context().move_var(VarIndex::new(1, 0)),
            Some(PineRef::new_box(true))
        );

        // assert_eq!(
        //     runner.get_context().move_var(VarIndex::new(1, 0)),
        //     Some(PineRef::new_box(false))
        // );
    }

}
