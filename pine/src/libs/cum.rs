
use super::VarResult;
use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};
use crate::helper::{
    pine_ref_to_f64,
};
use crate::runtime::context::{Ctx};
use crate::types::{
    Arithmetic, Callable, CallableFactory,
    Float, PineRef, RuntimeErr, Series, SeriesCall,
};
use std::mem;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
struct CumVal {
    prev_sum: Float,
}

impl CumVal {
    pub fn new() -> CumVal {
        CumVal {
            prev_sum: Some(0f64),
        }
    }
}

impl<'a> SeriesCall<'a> for CumVal {
    fn step(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        mut param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        let source = mem::replace(&mut param[0], None);

        let source = pine_ref_to_f64(source);
        if source.is_some() {
            self.prev_sum = self.prev_sum.add(source);
        }
        Ok(PineRef::new(Series::from(self.prev_sum)))
    }

    fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
        Box::new(self.clone())
    }
}

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(CallableFactory::new(|| {
        Callable::new(None, Some(Box::new(CumVal::new())))
    }));

    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![("x", SyntaxType::float_series())],
        SyntaxType::float_series(),
    ))]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, "cum")
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
    fn cum_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = "m = cum(close)";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![(
                    "close",
                    AnySeries::from_float_vec(vec![Some(12f64), Some(6f64)]),
                )],
                None,
            )
            .unwrap();

        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new(Series::from_vec(vec![
                Some(12f64),
                Some(18f64),
            ])))
        );
    }
}
