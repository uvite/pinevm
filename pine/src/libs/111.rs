use super::VarResult;
use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};


use crate::helper::{
    check_ge1_i64, move_element, pine_ref_to_f64_series, pine_ref_to_i64,
    require_param, series_index2,
};
use crate::runtime::context::{Ctx};
use crate::types::{Arithmetic, Bool, Callable, CallableFactory, Comparator, Float, ParamCollectCall, PineRef, RefData, RuntimeErr, Series, StepHandleFunc};

use std::rc::Rc;

pub fn series_change(series: &Series<Option<f64>>, length: usize) -> Float {
    series_index2(series, 0).minus(series_index2(series, length))
}

fn cross_func<'a>(
    _context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
    _func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {
    move_tuplet!((s1, s2) = param);
    let series1 = require_param("series", pine_ref_to_f64_series(s1))?;
    let series2 = require_param("series", pine_ref_to_f64_series(s2))?;

    Ok(PineRef::new_rc(Series::from(true)))

}
pub fn series_minus(s1:  &Series<Option<f64>>, s2: &Series<Option<f64>>, length: usize) -> Float {
    series_index2(&*s1,length).minus(series_index2(&*s2,length))
}
fn cross_over_func<'a>(
    _context: &mut dyn Ctx<'a>,
    mut param: Vec<Option<PineRef<'a>>>,
    _func_type: FunctionType<'a>,
) -> Result<PineRef<'a>, RuntimeErr> {
    move_tuplet!((s1, s2) = param);
    let series1 = require_param("series", pine_ref_to_f64_series(s1))?;
    let series2 = require_param("series", pine_ref_to_f64_series(s2))?;
    let up =series_minus(&*series1, &*series2, 0) ;
    let down =series_minus(&*series1, &*series2, 1) ;

    let result= up.gt(Some(0f64))&&down.lt(Some(0f64));


    Ok(PineRef::new_rc(Series::from(result)))

}
// fn cross_under_func<'a>(
//     _context: &mut dyn Ctx<'a>,
//     mut param: Vec<Option<PineRef<'a>>>,
//     _func_type: FunctionType<'a>,
// ) -> Result<PineRef<'a>, RuntimeErr> {
//     move_tuplet!((s1, s2) = param);
//     let up =series_minus(&*series1, &*series2, 0) ;
//     let down =series_minus(&*series1, &*series2, 1) ;
//
//     let result= up.lt(Some(0f64))&&down.gt(Some(0f64));
//
//     Ok(PineRef::new_rc(Series::from(result)))
//
// }



#[derive(Debug, Clone, PartialEq)]
struct CrossVal;

impl<'a> SeriesCall<'a> for CrossVal {
    fn step(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        mut param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        move_tuplet!((series, length, offset, sigma) = param);

        let series = require_param("series", pine_ref_to_f64_series(series))?;
        let length = require_param("length", pine_ref_to_f64(length))?;
        let offset = require_param("offset", pine_ref_to_f64(offset))?;
        let sigma = require_param("sigma", pine_ref_to_f64(sigma))?;

        if length < 1f64 {
            return Err(RuntimeErr::InvalidParameters(str_replace(
                GE_1,
                vec![String::from("length")],
            )));
        }
        // m = floor(offset * (windowsize - 1))
        let m = (offset * (length - 1f64)).floor();

        // s = windowsize / sigma
        let s = length / sigma;

        // norm = 0.0
        // sum = 0.0
        // for i = 0 to windowsize - 1
        //     weight = exp(-1 * pow(i - m, 2) / (2 * pow(s, 2)))
        //     norm := norm + weight
        //     sum := sum + series[windowsize - i - 1] * weight
        // sum / norm
        let mut norm = 0f64;
        let mut sum = 0f64;
        for i in 0..length as usize {
            let weight = ((-1f64 * (i as f64 - m).powi(2)) / (2f64 * s.powi(2))).exp();
            norm += weight;
            match series.index_value(length as usize - i - 1)? {
                Some(val) => {
                    sum += val * weight;
                }
                None => {
                    return Ok(PineRef::new_rc(Series::from(Float::from(None))));
                }
            }
        }
        Ok(PineRef::new(Series::from(Some(sum / norm))))
    }

    fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
        Box::new(self.clone())
    }
}

fn declare_var<'a>(name: &'static str, handle: StepHandleFunc) -> VarResult<'a> {

    let value = PineRef::new(CallableFactory::new(|| {
        Callable::new(None, Some(Box::new(CrossVal::new())))
    }));


    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![
            ("s1", SyntaxType::float_series()),
            ("s2", SyntaxType::float_series()),
        ],
        SyntaxType::bool(),
    ))]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, name)
}
// pub fn declare_cross_var<'a>() -> VarResult<'a> {
//     declare_var("cross", cross_func)
// }
pub fn declare_crossover_var<'a>() -> VarResult<'a> {
    declare_var("crossover", cross_over_func)
}
// pub fn declare_crossunder_var<'a>() -> VarResult<'a> {
//     declare_var("crossunder", cross_under_func)
// }

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::ast::stat_expr_types::VarIndex;
//     use crate::ast::syntax_type::SimpleSyntaxType;
//     use crate::runtime::{AnySeries, NoneCallback, VarOperate};
//     use crate::{LibInfo, PineParser, PineRunner,VarIndex};
//
//     #[test]
//     fn change_test() {
//         let lib_info = LibInfo::new(
//             vec![declare_change_var(), declare_mom_var()],
//             vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
//         );
//         let src = "m1 = change(close)\nm2 = mom(close, 1)";
//         let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
//         let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());
//
//         runner
//             .run(
//                 &vec![(
//                     "close",
//                     AnySeries::from_float_vec(vec![Some(10f64), Some(20f64)]),
//                 )],
//                 None,
//             )
//             .unwrap();
//         assert_eq!(
//             runner.get_context().move_var(VarIndex::new(0, 0)),
//             Some(PineRef::new(Series::from_vec(vec![None, Some(10f64)])))
//         );
//         assert_eq!(
//             runner.get_context().move_var(VarIndex::new(1, 0)),
//             Some(PineRef::new(Series::from_vec(vec![None, Some(10f64)])))
//         );
//     }
//
//     #[test]
//     fn change_color_test() {
//         use crate::libs::color;
//         use crate::libs::plot;
//         use crate::runtime::output::{OutputData, StrOptionsData};
//
//         let lib_info = LibInfo::new(
//             vec![
//                 declare_change_var(),
//                 plot::declare_var(),
//                 color::declare_var(),
//             ],
//             vec![
//                 ("high", SyntaxType::Series(SimpleSyntaxType::Float)),
//                 ("low", SyntaxType::Series(SimpleSyntaxType::Float)),
//             ],
//         );
//         // 5 10 -10
//         let src = "ao = high - low\nplot(ao, color = change(ao) <= 0 ? color.red : color.green, style=plot.style_histogram)";
//         let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
//         let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());
//
//         runner
//             .run(
//                 &vec![
//                     (
//                         "high",
//                         AnySeries::from_float_vec(vec![Some(10f64), Some(20f64), Some(10f64)]),
//                     ),
//                     (
//                         "low",
//                         AnySeries::from_float_vec(vec![Some(5f64), Some(10f64), Some(20f64)]),
//                     ),
//                 ],
//                 None,
//             )
//             .unwrap();
//         assert_eq!(
//             runner.move_output_data(),
//             vec![Some(OutputData::new_with_sc(
//                 vec![vec![Some(5f64), Some(10f64), Some(-10f64)]],
//                 vec![StrOptionsData {
//                     options: vec![String::from("#FF5252"), String::from("#4CAF50")],
//                     values: vec![Some(0), Some(1), Some(0)]
//                 }]
//             )),]
//         );
//     }
// }
