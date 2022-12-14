use super::VarResult;
use crate::ast::syntax_type::{SyntaxType};

use crate::runtime::context::Ctx;
use crate::types::{Evaluate, EvaluateVal, PineRef, RuntimeErr};
use chrono::Utc;

#[derive(Debug, Clone, PartialEq)]
struct TimenowVal {
    now_time: i64,
}

impl<'a> TimenowVal {
    fn new() -> TimenowVal {
        TimenowVal { now_time: 0 }
    }
}

impl<'a> EvaluateVal<'a> for TimenowVal {
    fn custom_name(&self) -> &str {
        "timenow"
    }

    fn call(&mut self, _ctx: &mut dyn Ctx<'a>) -> Result<PineRef<'a>, RuntimeErr> {
        if self.now_time == 0 {
            self.now_time = Utc::now().timestamp_millis();
        }
        Ok(PineRef::new_box(Some(self.now_time)))
    }

    fn copy(&self) -> Box<dyn EvaluateVal<'a>> {
        Box::new(TimenowVal::new())
    }
}

pub const VAR_NAME: &'static str = "timenow";

pub fn declare_var<'a>() -> VarResult<'a> {
    let value = PineRef::new(Evaluate::new(Box::new(TimenowVal::new())));
    let syntax_type = SyntaxType::int();
    VarResult::new(value, syntax_type, VAR_NAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::stat_expr_types::VarIndex;
    use crate::runtime::{AnySeries, NoneCallback, VarOperate};
    use crate::types::{Int, PineFrom};
    use crate::{LibInfo, PineParser, PineRunner};

    #[test]
    fn timenow_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = "m = timenow";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(2f64)]))],
                None,
            )
            .unwrap();

        let val = Int::implicity_from(runner.get_context().move_var(VarIndex::new(0, 0)).unwrap())
            .unwrap()
            .into_inner();
        assert!(val.unwrap() > 0);
    }
}
