use super::kc::KcVal;


use super::VarResult;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};





use crate::types::{
    Arithmetic, Callable, CallableFactory, Float, ParamCollectCall, PineRef, Series,
};
use std::rc::Rc;

fn gen_kcw<'a>(vals: (Float, Float, Float)) -> PineRef<'a> {
    let (basis, ema1, ema2) = vals;
    PineRef::new_rc(Series::from(ema1.minus(ema2).div(basis)))
}

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| {
        Callable::new(
            None,
            Some(Box::new(ParamCollectCall::new_with_caller(Box::new(
                KcVal::new(gen_kcw as *mut ()),
            )))),
        )
    }));

    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![
            ("series", SyntaxType::float_series()),
            ("length", SyntaxType::int()),
            ("mult", SyntaxType::float()),
            ("useTrueRange", SyntaxType::bool()),
        ],
        SyntaxType::float_series(),
    ))]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, "kcw")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::syntax_type::SyntaxType;
    use crate::runtime::VarOperate;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::types::Series;
    use crate::{LibInfo, PineParser, PineRunner};
    // use crate::libs::{floor, exp, };

    #[test]
    fn kcw_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![
                ("close", SyntaxType::float_series()),
                ("high", SyntaxType::float_series()),
                ("low", SyntaxType::float_series()),
            ],
        );
        let src = "m = kcw(close, 3, 1)\n";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![
                    (
                        "close",
                        AnySeries::from_float_vec(vec![Some(10f64), Some(20f64)]),
                    ),
                    (
                        "high",
                        AnySeries::from_float_vec(vec![Some(6f64), Some(20f64)]),
                    ),
                    (
                        "low",
                        AnySeries::from_float_vec(vec![Some(6f64), Some(20f64)]),
                    ),
                ],
                None,
            )
            .unwrap();

        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new(Series::from_vec(vec![
                Some(0f64),
                Some(0.8f64)
            ])))
        );
    }
}
