use super::sma::{sma_func, stdev_func};
use super::VarResult;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};
use crate::helper::err_msgs::*;
use crate::helper::str_replace;
use crate::helper::{
    move_element, pine_ref_to_f64, pine_ref_to_f64_series, pine_ref_to_i64,
    require_param,
};
use crate::runtime::context::{Ctx};

use crate::types::{
    Arithmetic, Callable, CallableFactory, ParamCollectCall, PineRef, RefData, RuntimeErr, Series, SeriesCall,
};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
struct BbVal;

impl<'a> SeriesCall<'a> for BbVal {
    fn step(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        mut param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        move_tuplet!((series, length, mult) = param);

        let series = require_param("series", pine_ref_to_f64_series(series))?;
        let length = require_param("length", pine_ref_to_i64(length))?;
        let mult = require_param("mult", pine_ref_to_f64(mult))?;

        if length < 1i64 {
            return Err(RuntimeErr::InvalidParameters(str_replace(
                GE_1,
                vec![String::from("length")],
            )));
        }

        let basis = sma_func(RefData::clone(&series), length)?;
        let dev = Some(mult).mul(stdev_func(series, length)?);

        // ((basis + dev) - (basis - dev)) / basis
        let result = basis.add(dev).minus(basis.minus(dev)).div(basis);
        Ok(PineRef::new(Series::from(result)))
    }

    fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
        Box::new(self.clone())
    }
}

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| {
        Callable::new(
            None,
            Some(Box::new(ParamCollectCall::new_with_caller(Box::new(BbVal)))),
        )
    }));

    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![
            ("series", SyntaxType::float_series()),
            ("length", SyntaxType::int()),
            ("mult", SyntaxType::float()),
        ],
        SyntaxType::float_series(),
    ))]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, "bbw")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::stat_expr_types::VarIndex;
    use crate::ast::syntax_type::SyntaxType;
    use crate::runtime::VarOperate;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::types::Series;
    use crate::{LibInfo, PineParser, PineRunner};
    // use crate::libs::{floor, exp, };

    #[test]
    fn bbw_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = "m1 = bbw(close, 2, 1.2)";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![(
                    "close",
                    AnySeries::from_float_vec(vec![Some(6f64), Some(12f64)]),
                )],
                None,
            )
            .unwrap();
        // 9 +   None, 3 * sqrt(2) * 1.2 * 2 / 9
        let res = 3f64 * 1.2f64 * 2f64 / 9f64;
        let series = pine_ref_to_f64_series(runner.get_context().move_var(VarIndex::new(0, 0)));
        let res_val = series.unwrap().index_value(1).unwrap().unwrap();
        println!("get res {:?} {:?}", res_val, res);
        assert!((res_val - res).abs() < 0.1f64);
        // assert_eq!(
        //     ,
        //     Some(PineRef::new(Series::from_vec(vec![None, Some(res)])))
        // );
    }
}
