use crate::tool::u8_to_i32;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, Utc};
use std::io::{BufReader, Error, Read, Write};
use std::net::TcpStream;

lazy_static! {
    // 1900和1970之间的timestamp差值
    pub static ref DIFF: i64 = NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp()
        .abs();

    // 将格林尼治时间+8:00转化为中国时间
    pub static ref CN_TIMEZONE:FixedOffset =FixedOffset::east(8 * 3600);
}

pub struct Client {
    address: String,
    port: u32,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            address: "127.0.0.1".to_string(),
            port: 37,
        }
    }
}

impl Client {
    pub fn new(address: &str, port: u32) -> Self {
        Client {
            address: address.to_string(),
            port,
        }
    }

    pub fn update(&self) -> Result<(), Error> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.address, self.port))?;

        stream
            .write("Getting timestamp".as_bytes())
            .expect("Failed to write");

        let mut reader = BufReader::new(&stream);
        let mut buffer: [u8; 4] = [0; 4];

        reader.read(&mut buffer)?;

        let time = u8_to_i32(buffer);

        let from_epoch = (time as i64 & 0xFFFFFFFF as i64) - *DIFF;

        let dt = NaiveDateTime::from_timestamp(from_epoch, 0);

        let now = DateTime::<Utc>::from_utc(dt, Utc).with_timezone(&*CN_TIMEZONE);
        println!("CurrentTime: {}", now);
        Ok(())
    }
}

// fn get_time() {
//     let now = Utc::now();
//     println!(
//         "UTC now in a custom format is: {}",
//         now.format("%a %b %e %T %Y")
//     );

//     let m = now.timestamp();
//     println!("{}", &m);

//     // let ms_since_1970 = now.timestamp();
//     // println!("{}", ms_since_1970);
//     // println!("{}", NaiveDateTime::from_timestamp(ms_since_1970, 0));
//     // let dt = now.with_timezone(&china_time_zone);
//     // println!("{}",dt);
//     // let ms_china = dt.timestamp();
//     // println!("{}",ms_china);
//     // println!("{}", NaiveDateTime::from_timestamp(ms_china, 0))
// }

// pub fn send_time() {
//     let ts = Utc::now().timestamp();
//     println!("ts: {}", ts);

//     let time_since_1900 = ts + *DIFF;

//     let time = (time_since_1900 & 0x00000000FFFFFFFF) as i32;
//     println!("time: {:X}", &time);

//     let from_epoch = (time as i64 & 0xFFFFFFFF as i64) - *DIFF;
//     // println!("from_epoch: {:X}", &from_epoch);
//     // println!("diff: {}", &diff);
//     let dt = NaiveDateTime::from_timestamp(from_epoch, 0);

//     let now = DateTime::<Utc>::from_utc(dt, Utc).with_timezone(&*CN_TIMEZONE);
//     println!("now: {}", now);
// }
