use super::sma::declare_ma_var;
use super::VarResult;


use crate::types::{
    Arithmetic, Float,
    RefData, RuntimeErr, Series,
};

pub fn series_sum<'a>(source: &Series<Float>, length: i64) -> Result<Float, RuntimeErr> {
    let mut sum_val = Some(0f64);
    for i in 0..length {
        let val = source.at(i as usize);
        sum_val = sum_val.add(val);
    }
    Ok(sum_val)
}

fn sum_func<'a>(source: RefData<Series<Float>>, length: i64) -> Result<Float, RuntimeErr> {
    let mut sum_val = Some(0f64);
    for i in 0..length {
        let val = source.index_value(i as usize).unwrap();
        sum_val = sum_val.add(val);
    }
    Ok(sum_val)
}

pub fn declare_var<'a>() -> VarResult<'a> {
    declare_ma_var("sum", sum_func)
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
    fn sum_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![("close", SyntaxType::float_series())],
        );
        let src = "m = sum(close, 2)";
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
            Some(PineRef::new(Series::from_vec(vec![None, Some(18f64),])))
        );
    }
}
