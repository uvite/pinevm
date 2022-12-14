extern crate pine;
use pine::ast::syntax_type::{SimpleSyntaxType, SyntaxType};
use pine::libs::plot;
use pine::libs::print;
use pine::runtime::data_src::{Callback, DataSrc, NoneCallback};
use pine::runtime::output::OutputData;
use pine::runtime::AnySeries;

const MA_SCRIPT: &str = "
// N = 5
// ma = close
// // ma = (close + close[1] + close[2] + close[3] + close[4]) / 5
// for i = 1 to N - 1
//     ma := ma + close[i]
// ma := ma / N
plot(sma(close, 3))
//plot(ma)
";

fn main() {
    let lib_info = pine::LibInfo::new(
        vec![plot::declare_var()],
        vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
    );
    let mut parser = pine::PineScript::new_with_libinfo(lib_info, Some(&NoneCallback()));
    parser.parse_src(String::from(MA_SCRIPT)).unwrap();
    let data = vec![(
        "close",
        AnySeries::from_float_vec(vec![
            Some(1f64),
            Some(2f64),
            Some(3f64),
            Some(4f64),
            Some(5f64),
        ]),
    )];
    let out_data = parser.run_with_data(data, None);
    //assert!(out_data.is_ok());
    println!("Out data {:?}", out_data.as_ref().unwrap().data_list);
    // for i in 0..2 {
    //     assert!(out_data.as_ref().unwrap().data_list[i]
    //         .as_ref()
    //         .unwrap()
    //         .series[0][15]
    //         .is_some());
    // }


}
