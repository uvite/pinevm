mod binance;
mod models;
mod vm;
mod statistics;
#[cfg(test)]
mod test_statistics;
mod utils;

extern crate csv;


#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let result = binance::get_klines(client.clone(), "30m", "ETHUSDT", 500).await;

    let kline_data = match result {
        Some(kline_data) => kline_data,
        _ => {
            panic!("Something went wrong.");
        }
    };
    println!("first result: {:?}", kline_data[0]);

    let price_data: Vec<f64> = kline_data.iter().rev().take(100).map(|f| f.close).collect();



    let mut reader = csv::Reader::from_path("./output.csv").unwrap();
    // Date,Open,High,Low,Close,Volume
    //
    // open_time,open,high,low,close,volume,close_time,quote_asset_volume,number_of_trades,taker_buy_base_asset_volume,taker_buy_quote_asset_volume,ignore
    let mut closes: Vec<Option<f64>> = vec![];
    // for record in reader.deserialize() {
    //     let (open_time, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
    //         record.unwrap();
    //
    //     // println!("{} ", close);
    //     closes.push(Some(close))
    // }


    for i in price_data {
        closes.push(Some(i))
    }
    // let closedata=VecOption::from(price_data);




    const VI_SCRIPTS: &'static str = r#"
study(title = "Vortex Indicator", shorttitle="VI", format=format.price, precision=4)
len = input(30, title="Period", minval=2)

// Jurik Moving Average
f_jma(_src, _length, _phase, _power) =>
    phaseRatio  = _phase < -100 ? 0.5 : _phase > 100 ? 2.5 : _phase / 100 + 1.5

    beta        = 0.45 * (_length - 1) / (0.45 * (_length - 1) + 2)
    alpha       = math.pow(beta, _power)
    jma         = 0.0

    e0          = 0.0
    e0          := (1 - alpha) * _src + alpha * nz(e0[1])
    e1          = 0.0
    e1          := (_src - e0) * (1 - beta) + beta * nz(e1[1])
    e2          = 0.0
    e2          := (e0 + phaseRatio * e1 - nz(jma[1])) * math.pow(1 - alpha, 2) + math.pow(alpha, 2) * nz(e2[1])
    jma         := e2 + nz(jma[1])
    jma
ema=f_jma(close,7,50,1)
plot(ema, title="v2 +", color=#3cc3E4)

"#;
    // println!("SMA: {:?}", closes);
    let out_data=vm::runcode(VI_SCRIPTS,&closes);
    //  println!("SMA: {:?}", rs);
    println!("sma data {:?}", out_data.as_ref().unwrap().data_list);
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
