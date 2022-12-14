use super::VarResult;

use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};


use crate::helper::{
    pine_ref_to_f64_series,
    require_param,
};
use crate::runtime::context::{Ctx};

use crate::types::{
    Arithmetic, Callable, CallableFactory, ParamCollectCall, PineRef, RuntimeErr, Series, SeriesCall,
};
use std::mem;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
struct EmaVal;

impl<'a> SeriesCall<'a> for EmaVal {
    fn step(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        mut param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        let source = mem::replace(&mut param[0], None);

        let source = require_param("x", pine_ref_to_f64_series(source))?;
        // x[3] * 1 / 6 + x[2] * 2 / 6 + x[1] * 2 / 6 + x[0] * 1 / 6
        let val = (source
            .index_value(3)
            .unwrap()
            .add(source.index_value(0).unwrap()))
        .div(Some(6f64))
        .add(
            source
                .index_value(2)
                .unwrap()
                .add(source.index_value(1).unwrap())
                .mul(Some(2f64).div(Some(6f64))),
        );
        Ok(PineRef::new(Series::from(val)))
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
                EmaVal,
            )))),
        )
    }));

    let func_type = FunctionTypes(vec![FunctionType::new((
        vec![("source", SyntaxType::float_series())],
        SyntaxType::float_series(),
    ))]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, "swma")
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
    fn alma_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = "m1 = swma(close)";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![(
                    "close",
                    AnySeries::from_float_vec(vec![
                        Some(6f64),
                        Some(12f64),
                        Some(12f64),
                        Some(6f64),
                    ]),
                )],
                None,
            )
            .unwrap();

        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new(Series::from_vec(vec![
                None,
                None,
                None,
                Some(10f64)
            ])))
        );
    }
}
