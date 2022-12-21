
extern crate pine;
use pine::ast::syntax_type::{SimpleSyntaxType, SyntaxType};
use pine::{LibInfo, PineParser, PineRunner};
use pine::ast::stat_expr_types::VarIndex;
use pine::libs::declare_vars;
use pine::libs::ema::{declare_ema_var, declare_rma_var, series_ema};
use pine::libs::plot;
use pine::libs::print;
use pine::runtime::data_src::{Callback, DataSrc, NoneCallback};
use pine::runtime::{AnySeries, OutputDataCollect, PineFormatError};
use pine::types::{PineRef, Series};

mod vm;

use yata::prelude::*;
use yata::methods::EMA;


extern crate csv;

const MACD_SCRIPT: &str = r#"
//@version=4
study(title="MACD", shorttitle="MACD")

// Getting inputs
fast_length = input(title="Fast Length", type=input.integer, defval=12)
slow_length = input(title="Slow Length", type=input.integer, defval=26)
src = input(title="Source", type=input.source, defval=close)
signal_length = input(title="Signal Smoothing", type=input.integer, minval = 1, maxval = 50, defval = 9)
sma_source = input(title="Simple MA(Oscillator)", type=input.bool, defval=false)
sma_signal = input(title="Simple MA(Signal Line)", type=input.bool, defval=false)

// Plot colors
col_grow_above = #26A69A
col_grow_below = #FFCDD2
col_fall_above = #B2DFDB
col_fall_below = #EF5350
col_macd = #0094ff
col_signal = #ff6a00
pine_ema(src, length) =>
    alpha = 2.0 / (length + 1)
    sum=0.0
    sum:= na(sum[1]) ? src : alpha * src + (1 - alpha) * nz(sum[1])

// Calculating
fast_ma =  pine_ema(close, fast_length)
slow_ma =  pine_ema(close, slow_length)
mymacd = fast_ma - slow_ma
signal = sma_signal ? sma(mymacd, signal_length) : pine_ema(mymacd, signal_length)
hist = mymacd - signal

plot(hist, title="Histogram", style=plot.style_columns, color=(hist>=0 ? (hist[1] < hist ? col_grow_above : col_fall_above) : (hist[1] < hist ? col_grow_below : col_fall_below) ), opacity=0 )
plot(mymacd, title="MACD", color=col_macd, opacity=0)
plot(signal, title="Signal", color=col_signal, opacity=0)
"#;
fn main() {

    let lib_info = LibInfo::new(
        vec![declare_ema_var(), declare_rma_var()],
        vec![("close", SyntaxType::float_series())],
    );
    let src = "m1 = ema(close, 3)\n";
    let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
    let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

    runner
        .run(
            &vec![(
                "close",
                AnySeries::from_float_vec(vec![Some(1f64)
                                               , Some(2f64), Some(3f64)
                                               , Some(4f64), Some(5f64), Some(6f64)
                                               , Some(7f64), Some(8f64), Some(9f64),
                ]
                ),
            )],
            None,
        )
        .unwrap();

    assert_eq!(
        runner.get_context().move_var(VarIndex::new(0, 0)),
        Some(PineRef::new(Series::from_vec(vec![
           // Some(0.5), Some(1.25), Some(2.125), Some(3.0625), Some(4.03125), Some(5.015625), Some(6.0078125), Some(7.00390625), Some(8.001953125)
            Some(1.0), Some(1.5), Some(2.25), Some(3.125), Some(4.0625), Some(5.03125), Some(6.015625), Some(7.0078125), Some(8.00390625)
        ])))
    );
   let a= runner.get_context().move_var(VarIndex::new(0, 0));

   // println!("{:?}3333",a);

//
//     use ta::indicators::ExponentialMovingAverage;
//     use ta::Next;
//
// // it can return an error, when an invalid length is passed (e.g. 0)
//     let mut ema = ExponentialMovingAverage::new(3).unwrap();
//
//     assert_eq!(ema.next(1.0), 1.0);
//     assert_eq!(ema.next(2.0), 1.5);
//     assert_eq!(ema.next(3.0), 2.25);
//     assert_eq!(ema.next(4.0), 3.125);
    //let ema1 = series_ema(close, fastlen, &mut self.ema1s)?;
// EMA of length=3
    let mut ema = EMA::new(3, &1f64).unwrap();



    println!("{}--2", ema.next(&2.0));
    println!("{}--3", ema.next(&3.0));
    println!("{}--4", ema.next(&4.0));
    println!("{}--5", ema.next(&5.0));
    // ema.next(&3.0);
    // ema.next(&4.0);
    // ema.next(&5.0);
    //
    // assert_eq!(ema.next(&6.0), 6.75);
    // assert_eq!(ema.next(&7.0), 9.375);
    // assert_eq!(
    //     runner.get_context().move_var(VarIndex::new(1, 0)),
    //     Some(PineRef::new(Series::from_vec(vec![
    //         None,
    //         Some(5f64),
    //         Some(12.5f64)
    //     ])))
    // );


}
