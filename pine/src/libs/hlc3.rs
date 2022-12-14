use super::VarResult;
use crate::ast::stat_expr_types::VarIndex;
use crate::ast::syntax_type::SyntaxType;
use crate::helper::ensure_srcs;
use crate::helper::{
    pine_ref_to_f64,
};
use crate::runtime::context::{Ctx};

use crate::types::{
    Arithmetic, Evaluate, EvaluateVal, Float, PineRef, RuntimeErr,
    Series,
};

#[derive(Debug, Clone, PartialEq)]
struct AccDistVal {
    low_index: VarIndex,
    high_index: VarIndex,
    close_index: VarIndex,
    ad_history: Vec<Float>,
    is_init: bool,
}

impl AccDistVal {
    pub fn new() -> AccDistVal {
        AccDistVal {
            low_index: VarIndex::new(0, 0),
            high_index: VarIndex::new(0, 0),
            close_index: VarIndex::new(0, 0),
            ad_history: vec![],
            is_init: false,
        }
    }
}

impl<'a> EvaluateVal<'a> for AccDistVal {
    fn custom_name(&self) -> &str {
        "hlc3"
    }

    fn call(&mut self, ctx: &mut dyn Ctx<'a>) -> Result<PineRef<'a>, RuntimeErr> {
        ensure_srcs(ctx, vec!["low", "high", "close"], |indexs| {
            self.low_index = indexs[0];
            self.high_index = indexs[1];
            self.close_index = indexs[2];
        });

        let low = pine_ref_to_f64(ctx.get_var(self.low_index).clone());
        let high = pine_ref_to_f64(ctx.get_var(self.high_index).clone());
        let close = pine_ref_to_f64(ctx.get_var(self.close_index).clone());
        let res = high.add(low).add(close).div(Some(3f64));
        self.ad_history.push(res);
        Ok(PineRef::new_rc(Series::from(res)))
    }

    fn back(&mut self, _ctx: &mut dyn Ctx<'a>) -> Result<(), RuntimeErr> {
        self.ad_history.pop();
        Ok(())
    }

    fn copy(&self) -> Box<dyn EvaluateVal<'a>> {
        Box::new(self.clone())
    }
}
pub const VAR_NAME: &'static str = "hlc3";

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(Evaluate::new(Box::new(AccDistVal::new())));

    let syntax_type = SyntaxType::float_series();
    VarResult::new(value, syntax_type, VAR_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::syntax_type::SyntaxType;
    use crate::runtime::VarOperate;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::types::Series;
    use crate::{LibInfo, PineParser, PineRunner};

    #[test]
    fn accdist_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![
                ("high", SyntaxType::float_series()),
                ("low", SyntaxType::float_series()),
                ("close", SyntaxType::float_series()),
            ],
        );
        let src = "m = hlc3";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![
                    (
                        "high",
                        AnySeries::from_float_vec(vec![Some(15f64), Some(22f64)]),
                    ),
                    (
                        "low",
                        AnySeries::from_float_vec(vec![Some(2f64), Some(2f64)]),
                    ),
                    (
                        "close",
                        AnySeries::from_float_vec(vec![Some(1f64), Some(0f64)]),
                    ),
                ],
                None,
            )
            .unwrap();

        assert_eq!(
            runner.get_context().get_var(VarIndex::new(0, 0)),
            &Some(PineRef::new(Series::from_vec(vec![Some(6f64), Some(8f64)])))
        );
    }
}
