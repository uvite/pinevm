mod binance;
mod models;
mod statistics;
#[cfg(test)]
mod test_statistics;
mod utils;
mod vm;
use std::{env, fs::File, io::Read};

use num_traits::ToPrimitive;
use yata::core::PeriodType;

use pine::runtime::InputVal;
use yata::methods::SMA;
use yata::prelude::*;

use ta::indicators::ExponentialMovingAverage as Ema;
use ta::DataItem;
use ta::Next;
extern crate csv;

fn ema1(len: i64) {
    let mut reader = csv::Reader::from_path("./output.csv").unwrap();
    let l = len as usize;
    let mut ema = Ema::new(l).unwrap();

    for record in reader.deserialize() {
        let (open_time, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
            record.unwrap();

        let dt = DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()
            .unwrap();
        let ema_val = ema.next(&dt);
        println!("{} = {:2.2}", ema, ema_val);
    }
}
#[tokio::main]
async fn main() {
    let length = 88;
    // ema1(length);
    // let client = utils::get_client();
    // let result = binance::get_klines(client.clone(), "30m", "ETHUSDT", 500).await;
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

        // println!("{} ", close);
        closes.push(Some(close));
        highs.push(Some(high));
        lows.push(Some(low));
    }

    let mut fake: Vec<Option<f64>> = vec![];

    // These are all done without reallocating...
    //     for i in 0..100 {
    //         println!("{}",i);
    //         fake.push(i.to_f64());
    //     }

    let mut f = File::open("./pine/jma.ps").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
   // println!("SMA: {:?}", buffer);


    let out_data = vm::runcode(buffer, vec![Some(InputVal::Int(length))], &closes,&highs,&lows);
    println!("SMA: {:?}", out_data);
    //println!("sma data {:?}", out_data.as_ref().unwrap().data_list);
    //
   // let out = &out_data.as_ref().unwrap().data_list[0];
   //  println!("{:?}", out_data);
   //  let mut pd = vec![];
   //  for k in out {
   //      for val in &k.series[0] {
   //          match val {
   //              Some(val) => {
   //                  pd.push(val);
   //                  println!("{}", val);
   //              }
   //              None => pd.push(&0f64),
   //          }
   //      }
   //  }
    // // // //
    // let a=closes[0];
    // // // it can return an error, when an invalid length is passed (e.g. 0)
    // //let mut ema = ExponentialMovingAverage::new(26).unwrap();
    // let mut ema = SMA::new(length as PeriodType, &a.unwrap()).unwrap();
    //
    // // assert_eq!(ema.next(2.0), 2.0);
    //  closes.drain(0..=0);
    // let mut jj=1;
    // for i in closes {
    //     println!("ema:{}--ta.ema:{},id:{},close:{}", ema.next(&i.unwrap()),pd[jj],jj,&i.unwrap());
    //     jj=jj+1;
    // }
}
