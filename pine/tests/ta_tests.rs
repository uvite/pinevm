extern crate pine;
use pine::ast::syntax_type::{SimpleSyntaxType, SyntaxType};
use pine::libs::{hma, plot, ta};
use pine::libs::print;
use pine::{LibInfo, PineParser, PineRunner};
use pine::ast::stat_expr_types::VarIndex;
use pine::runtime::data_src::{Callback, DataSrc, NoneCallback};
use pine::runtime::output::OutputData;
use pine::runtime::AnySeries;
use pine::types::{downcast_pf, PineRef, Tuple, Color, Series};




#[test]
fn sma_test() {

    let lib_info = LibInfo::new(
        vec![ta::declare_var()],

        vec![
            ("close", SyntaxType::float_series()),
            ("high", SyntaxType::float_series()),
            ("low", SyntaxType::float_series()),
        ],
    );
    let src = "m = ta.sma(close, 2)\n";
    let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
    let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

    runner
        .run(
            &vec![(
                "close",
                AnySeries::from_float_vec(vec![
                    Some(6f64),
                    Some(12f64),
                    Some(6f64),
                    Some(12f64),
                ]),
            )],
            None,
        )
        .unwrap();

    assert_eq!(
        runner.get_context().move_var(VarIndex::new(0, 0)),
        Some(PineRef::new(Series::from_vec(vec![
            None,
            Some(14f64),
            Some(4f64),
            Some(14f64)
        ])))
    );
}

#[test]
fn sma(){
    let lib_info = LibInfo::new(
        vec![ta::declare_var()],

        vec![
            ("close", SyntaxType::float_series()),
            ("high", SyntaxType::float_series()),
            ("low", SyntaxType::float_series()),
        ],
    );
    let src = "m = ta.sma(close, 2)\nm2 = ta.wma(close, 2)\n
        m3 = ta.dev(close, 2)\nm4 = ta.variance(close, 2)\n
        m5 = ta.stdev(close, 2)";
    let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
    let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

    runner
        .run(
            &vec![(
                "close",
                AnySeries::from_float_vec(vec![Some(6f64), Some(12f64)]),
            )],
            None,
        )
        .unwrap();
    let starti = 0;
    assert_eq!(
        runner.get_context().move_var(VarIndex::new(starti, 0)),
        Some(PineRef::new(Series::from_vec(vec![None, Some(9f64)])))
    );
    assert_eq!(
        runner.get_context().move_var(VarIndex::new(starti + 1, 0)),
        Some(PineRef::new(Series::from_vec(vec![None, Some(10f64)])))
    );
    assert_eq!(
        runner.get_context().move_var(VarIndex::new(starti + 2, 0)),
        Some(PineRef::new(Series::from_vec(vec![None, Some(3f64)])))
    );
    assert_eq!(
        runner.get_context().move_var(VarIndex::new(starti + 3, 0)),
        Some(PineRef::new(Series::from_vec(vec![None, Some(9f64)])))
    );
    assert_eq!(
        runner.get_context().move_var(VarIndex::new(starti + 4, 0)),
        Some(PineRef::new(Series::from_vec(vec![None, Some(3f64)])))
    );
}
