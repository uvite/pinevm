extern crate pine;
use pine::ast::syntax_type::{SimpleSyntaxType, SyntaxType};
use pine::libs::declare_vars;
use pine::libs::plot;
use pine::libs::print;
use pine::runtime::data_src::{Callback, DataSrc, NoneCallback};
use pine::runtime::{AnySeries, OutputDataCollect, PineFormatError};




pub fn runcode( code: &str,closes: &Vec<Option<f64>>,highs: &Vec<Option<f64>>,lows: &Vec<Option<f64>>)-> Result<OutputDataCollect, PineFormatError>{
    let lib_info = pine::LibInfo::new(
        declare_vars(),
        vec![
            ("close", SyntaxType::Series(SimpleSyntaxType::Float)),
            ("open", SyntaxType::Series(SimpleSyntaxType::Float)),
            ("high", SyntaxType::Series(SimpleSyntaxType::Float)),
            ("low", SyntaxType::Series(SimpleSyntaxType::Float)),
            ("volume", SyntaxType::Series(SimpleSyntaxType::Int)),
            ("_time", SyntaxType::Series(SimpleSyntaxType::Int)),
            ("bar_index", SyntaxType::Series(SimpleSyntaxType::Int)),
        ],
    );
    let mut parser = pine::PineScript::new_with_libinfo(lib_info, Some(&NoneCallback()));

    // let mut close_data: Vec<Option<f64>> = vec![];
    // close_data.resize(800, data_set);

    let mut low_data: Vec<Option<f64>> = vec![];
    low_data.resize(800, Some(1f64));


    let data = vec![
        ("close", AnySeries::from_float_vec(closes.to_vec())),
        ("high", AnySeries::from_float_vec(highs.to_vec())),
        ("low", AnySeries::from_float_vec(lows.to_vec())),
       // ("low", AnySeries::from_float_vec(low_data)),
    ];
    parser.parse_src(String::from(code)).unwrap();
    let out_data=parser.run_with_data(data.clone(), None);

    //println!("Out data {:?}", out_data.as_ref().unwrap().data_list);
    //Some(out_data)
    out_data
}


