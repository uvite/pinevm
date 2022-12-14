

use super::rsi::calc_rsi_series;

use super::sum::series_sum;

use super::VarResult;
use crate::ast::stat_expr_types::VarIndex;
use crate::ast::syntax_type::{FunctionType, FunctionTypes, SyntaxType};


use crate::helper::{
    ensure_srcs, ge1_param_i64, move_element,
    pine_ref_to_f64, pine_ref_to_f64_series, pine_ref_to_i64, require_param,
};
use crate::runtime::context::{Ctx};

use crate::types::{
    Arithmetic, Callable, CallableFactory, Float, ParamCollectCall, PineRef, RuntimeErr, Series,
    SeriesCall,
};
use std::rc::Rc;
#[derive(Debug, Clone, PartialEq)]
pub struct KcVal<'a> {
    volume_index: VarIndex,
    upper_history: Series<'a, Float>,
    lower_history: Series<'a, Float>,
}

impl<'a> KcVal<'a> {
    pub fn new() -> KcVal<'a> {
        KcVal {
            volume_index: VarIndex::new(0, 0),
            upper_history: Series::new(),
            lower_history: Series::new(),
        }
    }

    fn handle_index(&mut self, ctx: &mut dyn Ctx<'a>) {
        ensure_srcs(ctx, vec!["volume"], |indexs| {
            self.volume_index = indexs[0];
        });
    }

    fn process_rsi(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        mut param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<Float, RuntimeErr> {
        self.handle_index(_ctx);
        move_tuplet!((series, length) = param);

        let series = require_param("series", pine_ref_to_f64_series(series))?;
        let length = ge1_param_i64("length", pine_ref_to_i64(length))?;

        let s0 = series.index_value(0).unwrap();
        let s1 = series.index_value(1).unwrap();
        let volume = pine_ref_to_f64(_ctx.get_var(self.volume_index).clone());

        let upper = if s0.minus(s1) <= Some(0f64) {
            Some(0f64)
        } else {
            s0.mul(volume)
        };
        self.upper_history.update(upper);

        let lower = if s0.minus(s1) >= Some(0f64) {
            Some(0f64)
        } else {
            s0.mul(volume)
        };
        self.lower_history.update(lower);

        // self.upper_history.push(upper);
        // self.lower_history.push(lower);

        let upper_sum = series_sum(&self.upper_history, length)?;
        let lower_sum = series_sum(&self.lower_history, length)?;

        self.upper_history.commit();
        self.lower_history.commit();

        let res = if lower_sum.is_none() {
            None
        } else {
            let res = calc_rsi_series(upper_sum, lower_sum)?;
            res
        };
        Ok(res)
    }
}

impl<'a> SeriesCall<'a> for KcVal<'a> {
    fn step(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        param: Vec<Option<PineRef<'a>>>,
        _func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        let res = self.process_rsi(_ctx, param, _func_type)?;
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
            ("series", SyntaxType::float_series()),
            ("length", SyntaxType::int()),
        ],
        SyntaxType::float_series(),
    ))]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, "mfi")
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
    fn mfi_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![
                ("close", SyntaxType::float_series()),
                ("volume", SyntaxType::float_series()),
            ],
        );
        let src = "m = mfi(close, 2)\n";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![
                    (
                        "close",
                        AnySeries::from_float_vec(vec![Some(20f64), Some(10f64), Some(40f64)]),
                    ),
                    (
                        "volume",
                        AnySeries::from_int_vec(vec![Some(1i64), Some(1i64), Some(1i64)]),
                    ),
                ],
                None,
            )
            .unwrap();

        assert_eq!(
            runner.get_context().move_var(VarIndex::new(0, 0)),
            Some(PineRef::new(Series::from_vec(vec![
                None,
                Some(0f64),
                Some(80f64),
            ])))
        );
    }
}

// 20 10 20, na 0 40, na 10 0  40 10  rs 2  res 100 - 100 / 5
