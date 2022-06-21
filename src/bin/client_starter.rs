use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;

use chrono::{DateTime, NaiveDateTime, Utc};
use time_protocol::time_client::{CN_TIMEZONE, DIFF};
use time_protocol::tool::u8_to_i32;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:37")?;

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
