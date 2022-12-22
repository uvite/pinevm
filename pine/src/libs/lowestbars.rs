
use super::highestbars::declare_s_var;
use super::VarResult;


use crate::helper::{
    series_index,
};


use crate::types::{
    Float, Int, RefData, Series,
};



fn get_min_val<'a>(source: &Option<RefData<Series<Float>>>, length: i64) -> Int {
    let mut min_val = Some(std::f64::MAX);
    let mut min_i = Some(0);
    for i in 0..length {
        let cur_val = series_index(source, i as usize);
        if cur_val < min_val {
            min_i = Some(-i);
            min_val = cur_val;
        }
    }
    min_i
}

pub fn declare_var<'a>() -> VarResult<'a> {
    declare_s_var("lowestbars", "low", get_min_val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::syntax_type::SyntaxType;
    use crate::runtime::VarOperate;
    use crate::runtime::{AnySeries, NoneCallback};
    use crate::types::Series;
    use crate::{LibInfo, PineParser, PineRef, PineRunner, VarIndex};

    #[test]
    fn accdist_test() {
        let lib_info = LibInfo::new(
            vec![declare_var()],
            vec![
                ("close", SyntaxType::float_series()),
                ("low", SyntaxType::float_series()),
            ],
        );
        let src = "m1 = lowestbars(2)\nm2 = lowestbars(close, 2)";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![
                    (
                        "close",
                        AnySeries::from_float_vec(vec![Some(10f64), Some(20f64), Some(5f64)]),
                    ),
                    (
                        "low",
                        AnySeries::from_float_vec(vec![Some(19f64), Some(25f64), Some(10f64)]),
                    ),
                ],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().get_var(VarIndex::new(0, 0)),
            &Some(PineRef::new(Series::from_vec(vec![
                Some(-1i64),
                Some(-1i64),
                Some(0i64)
            ])))
        );
    }
}
