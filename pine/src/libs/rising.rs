
use super::falling::declare_s_var;
use super::VarResult;





use crate::types::{
    Float, PineRef,
    RefData, RuntimeErr, Series,
};

pub const VAR_NAME: &'static str = "rising";

fn check_greater<'a>(
    source: RefData<Series<Float>>,
    length: i64,
) -> Result<PineRef<'a>, RuntimeErr> {
    let cur_val = source.index_value(0).unwrap();
    for i in 1..=length as usize {
        if source.index_value(i).unwrap() > cur_val {
            return Ok(PineRef::new_rc(Series::from(false)));
        }
    }
    return Ok(PineRef::new_rc(Series::from(true)));
}

pub fn declare_var<'a>() -> VarResult<'a> {
    declare_s_var(VAR_NAME, check_greater)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::syntax_type::SyntaxType;
    use crate::runtime::VarOperate;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::types::Series;
    use crate::{LibInfo, PineParser, PineRunner, VarIndex};

    #[test]
    fn accdist_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = "m = rising(close, 2)";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![(
                    "close",
                    AnySeries::from_float_vec(vec![Some(10f64), Some(20f64), Some(5f64)]),
                )],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().get_var(VarIndex::new(0, 0)),
            &Some(PineRef::new(Series::from_vec(vec![true, true, false])))
        );
    }
}
