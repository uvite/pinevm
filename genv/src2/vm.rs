
extern crate pine;
use pine::ast::syntax_type::{SimpleSyntaxType, SyntaxType};
use pine::{LibInfo, PineParser, PineRunner};
use pine::ast::stat_expr_types::VarIndex;
use pine::libs::declare_vars;

use pine::libs::plot;
use pine::libs::print;
use pine::runtime::data_src::{Callback, DataSrc, NoneCallback};
use pine::runtime::{AnySeries, InputVal};
use pine::types::PineRef;
use binance::websockets::*;
use std::sync::atomic::{AtomicBool};
use binance::api::Binance;
use binance::config::Config;
const VI_SCRIPTS: &'static str = r#"
study(title = "Vortex Indicator", shorttitle="VI", format=format.price, precision=4)
period_ = input(14, title="Period", minval=2)

VMP = sum( abs( high - low[1]), period_ )
VMM = sum( abs( low - high[1]), period_ )
STR = sum( atr(1), period_ )
VIP = VMP / STR
VIM = VMM / STR

plot(VIP, title="VI +", color=#3BB3E4)
plot(VIM, title="VI -", color=#FF006E)
"#;

fn main() {
    // let lib_info = pine::LibInfo::new(
    //     declare_vars(),
    //     vec![
    //         ("close", SyntaxType::Series(SimpleSyntaxType::Float)),
    //         ("open", SyntaxType::Series(SimpleSyntaxType::Float)),
    //         ("high", SyntaxType::Series(SimpleSyntaxType::Float)),
    //         ("low", SyntaxType::Series(SimpleSyntaxType::Float)),
    //         ("volume", SyntaxType::Series(SimpleSyntaxType::Int)),
    //         ("_time", SyntaxType::Series(SimpleSyntaxType::Int)),
    //         ("bar_index", SyntaxType::Series(SimpleSyntaxType::Int)),
    //     ],
    // );
    // let mut parser = pine::PineScript::new_with_libinfo(lib_info, Some(&NoneCallback()));
    //
    // let mut high_data: Vec<Option<f64>> = vec![];
    // high_data.resize(800, Some(10f64));
    //
    // let mut low_data: Vec<Option<f64>> = vec![];
    // low_data.resize(800, Some(1f64));
    //
    // let data = vec![
    //     ("high", AnySeries::from_float_vec(high_data)),
    //     ("low", AnySeries::from_float_vec(low_data)),
    // ];
    // parser.parse_src(String::from(VI_SCRIPTS)).unwrap();
    // let out_data=parser.run_with_data(data, None);
    //
    // println!("Out data {:?}", out_data.as_ref().unwrap().data_list);


    let config = Config::default()
        .set_ws_endpoint("wss://stream.binance.com/ws");

    //wss://stream.binance.com/ws/${formattedPair.toLowerCase()}@kline_${formattedInterval}
    // .set_futures_rest_api_endpoint("https://testnet.binancefuture.com/api")
    // .set_futures_ws_endpoint("https://testnet.binancefuture.com/ws")

    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let kline = format!("{}", "ethusdt@kline_5m");
    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                println!("Symbol: {}, high: {}, low: {}", kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high);
            },
            _ => (),
        };
        Ok(())
    });

    web_socket.connect_with_config(&kline,&config).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
            err => {
                println!("Error: {:?}", err);
            }
        }
    }
    web_socket.disconnect().unwrap();
    //
    // println!("Out data {:?}", 333);
    //
    //
    // let lib_info = LibInfo::new(
    //     declare_vars(),
    //     vec![("close", SyntaxType::float_series())],
    // );
    // let src = r#"
    // band = input(1.0, title="Multiplier")
    // band3 = input(3.0, title="Multiplier")
    // "#;
    // let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
    // let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());
    //
    // runner.change_inputs(vec![Some(InputVal::Float(2f64))]);
    // runner
    //     .run(
    //         &vec![("close", AnySeries::from_float_vec(vec![Some(1f64)]))],
    //         None,
    //     )
    //     .unwrap();
    // // assert_eq!(
    // //     runner.get_context().move_var(VarIndex::new(0, 0)),
    // //     Some(PineRef::new_box(Some(2f64)))
    // // );
    // let out1=runner.get_context().move_var(VarIndex::new(1, 0));
    // println!("Out data {:?}", out1);


    // let out_data=parser.run_with_data(data, None);
    //
    // println!("Out data {:?}", out_data.as_ref().unwrap().data_list);


}
