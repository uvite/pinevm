

use super::highest::get_max_val;
use super::lowest::get_min_val;


use super::VarResult;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};


use crate::helper::{
    ge1_param_i64, move_element,
    pine_ref_to_f64, pine_ref_to_f64_series, pine_ref_to_i64,
};
use crate::runtime::context::{Ctx};

use crate::types::{
    Arithmetic, Callable, CallableFactory, Float, ParamCollectCall, PineRef, RuntimeErr, Series, SeriesCall,
};

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct KcVal {}

impl KcVal {
    pub fn new() -> KcVal {
        KcVal {}
    }

    fn process_stoch<'a>(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        mut param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<Float, RuntimeErr> {
        move_tuplet!((source, high, low, length) = param);

        let length = ge1_param_i64("length", pine_ref_to_i64(length))?;
        // 100 * (close - lowest(low, length)) / (highest(high, length) - lowest(low, length))
        let source = pine_ref_to_f64(source);
        let high = pine_ref_to_f64_series(high);
        let low = pine_ref_to_f64_series(low);

        let low_val = get_min_val(&low, length);
        let high_val = get_max_val(&high, length);
        Ok(Some(100f64)
            .mul(source.minus(low_val))
            .div(high_val.minus(low_val)))
    }
}

impl<'a> SeriesCall<'a> for KcVal {
    fn step(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        let res = self.process_stoch(_ctx, param, _func_type)?;
        Ok(PineRef::new_rc(Series::from(res)))
    }

    fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
        Box::new(self.clone())
    }
}

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| {
        Callable::new(
            None,
            Some(Box::new(ParamCollectCall::new_with_caller(Box::new(
                KcVal::new(),
            )))),
        )
    }));

    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![
            ("source", SyntaxType::float_series()),
            ("high", SyntaxType::float_series()),
            ("low", SyntaxType::float_series()),
            ("length", SyntaxType::int()),
        ],
        SyntaxType::float_series(),
    ))]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, "stoch")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::syntax_type::SyntaxType;
    use crate::runtime::VarOperate;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::types::Series;
    use crate::{LibInfo, PineParser, PineRunner,VarIndex};
    // use crate::libs::{floor, exp, };

    #[test]
    fn rsi_int_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = "m = stoch(close, close, close, 2)\n";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![(
                    "close",
                    AnySeries::from_float_vec(vec![Some(20f64), Some(30f64)]),
                )],
                None,
            )
            .unwrap();

        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new(Series::from_vec(vec![None, Some(100.0)])))
        );
    }
}
