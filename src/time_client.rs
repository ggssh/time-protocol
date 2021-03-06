use crate::tool::u8_to_i32;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, Utc};
use std::io::{BufReader, Error, Read, Write};
use std::net::TcpStream;

lazy_static! {
    // 1900和1970之间的timestamp差值
    static ref DIFF: i64 = NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp()
        .abs();

    // 将格林尼治时间+8:00转化为中国时间
    static ref CN_TIMEZONE:FixedOffset =FixedOffset::east(8 * 3600);
}

#[derive(Debug)]
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

    pub fn update(&self) -> Result<String, Error> {
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
        // format!("CurrentTime: {}", now);
        Ok(format!("CurrentTime: {}", now))
    }

    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }

    pub fn set_port(&mut self, port: u32) {
        self.port = port;
    }
}
