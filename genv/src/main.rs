mod binance;
mod models;
mod vm;
mod statistics;
#[cfg(test)]
mod test_statistics;
mod utils;
use yata::prelude::*;
use yata::methods::EMA;
extern crate csv;


#[tokio::main]
async fn main() {
    // let client = utils::get_client();
    // let result = binance::get_klines(client.clone(), "1d", "BTCUSDT", 500).await;
    //
    // let kline_data = match result {
    //     Some(kline_data) => kline_data,
    //     _ => {
    //         panic!("Something went wrong.");
    //     }
    // };
    // println!("first result: {:?}", kline_data[0]);
    //
    // let price_data: Vec<f64> = kline_data.iter().rev().take(100).map(|f| f.close).collect();



    let mut reader = csv::Reader::from_path("./output.csv").unwrap();
    // Date,Open,High,Low,Close,Volume
    //
    // open_time,open,high,low,close,volume,close_time,quote_asset_volume,number_of_trades,taker_buy_base_asset_volume,taker_buy_quote_asset_volume,ignore
    let mut closes: Vec<Option<f64>> = vec![];
    let mut highs: Vec<Option<f64>> = vec![];
    let mut lows: Vec<Option<f64>> = vec![];
    for record in reader.deserialize() {
        let (open_time, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
            record.unwrap();

        closes.push(Some(close));
        highs.push(Some(high));
        lows.push(Some(low));
        println!("h{},l{},c{} ", high,low,close);

    }
    let a=closes[0];
    // it can return an error, when an invalid length is passed (e.g. 0)
    //let mut ema = ExponentialMovingAverage::new(26).unwrap();
    let mut ema = EMA::new(26, &a.unwrap()).unwrap();

    // assert_eq!(ema.next(2.0), 2.0);
     closes.drain(0..=0);
    for i in closes {
        println!("{}", ema.next(&i.unwrap()));
    }

    //
    // for i in price_data {
    //     closes.push(Some(i))
    // }
    // let closedata=VecOption::from(price_data);




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
    sum = 0.0
    sum := na(sum[1]) ? src : alpha * src + (1 - alpha) * nz(sum[1])
// Calculating
fast_ma = ta.ema(close, 12)
slow_ma = pine_ema(close, 120)
mymacd = fast_ma - slow_ma
signal = sma_signal ? sma(mymacd, signal_length) : ema(mymacd, signal_length)
hist = mymacd - signal
//plot(mymacd, title="MACD", color=col_macd, opacity=0)

plot(slow_ma )
//plot(fast_ma, title="Signal", color=col_signal, opacity=0)
"#;
    // println!("SMA: {:?}", closes);
    //let out_data=vm::runcode(MACD_SCRIPT,&closes,&highs,&lows);
    //  println!("SMA: {:?}", rs);
   // println!("sma data {:?}", out_data.as_ref().unwrap().data_list[0]);
   // println!("sma data {:?}", out_data.as_ref().unwrap().data_list[1]);
   // println!("sma data {:?}", out_data.as_ref().unwrap().data_list[2]);
   // println!("sma data {:?}", out_data.as_ref().unwrap().data_list);
    // println!("Out data {:?}", out_data.as_ref().unwrap().data_list[1]);

    //
    // let result = statistics::simple_moving_average(&price_data, 26);
    //
    // let sma_data = match result {
    //     Some(data) => data,
    //     _ => panic!("Calculating SMA failed"),
    // };
    //
    // println!("SMA: {:?}", sma_data);
    //
    // let result = statistics::exponential_moving_average(&price_data, 26);
    //
    // let ema_data = match result {
    //     Some(data) => data,
    //     _ => panic!("Calculating EMA failed"),
    // };
    //
    // println!("EMA: {:?}", ema_data);
    //
    // let result = statistics::moving_average_convergence_divergence(&price_data, 12, 26, 9);
    //
    // let macd_data = match result {
    //     Some(data) => data,
    //     _ => panic!("Calculating MACD failed"),
    // };
    //
    // println!("MACD: {:?}", macd_data);
    //
    // let typical_price_data: Vec<f64> = kline_data
    //     .iter()
    //     .rev()
    //     .take(100)
    //     .map(|f| (f.high + f.low + f.close) / 3.0)
    //     .collect();
    // let result = statistics::bollinger_bands(&typical_price_data, 20, 2.0);
    //
    // let boll_data = match result {
    //     Some(data) => data,
    //     _ => panic!("Calculating BOLL failed"),
    // };
    //
    // println!("BOLL: {:?}", boll_data);
    //
    // let result = statistics::relative_strength_index(&price_data, 14);
    //
    // let rsi_data = match result {
    //     Some(data) => data,
    //     _ => panic!("Calculating RSI failed"),
    // };
    //
    // println!("RSI: {:?}", rsi_data);
}
