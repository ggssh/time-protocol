use core::time;
use std::{
    io::{self, Error, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use chrono::{NaiveDate, Utc};

use crate::tool::i32_to_u8;

lazy_static! {
    // 1900和1970之间的timestamp差值
    static ref DIFF: i64 = NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp()
        .abs();
}

pub struct Server {
    address: String,
    port: u32,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 37,
        }
    }
}

impl Server {
    pub fn new(address: &str, port: u32) -> Self {
        Self {
            address: address.to_string(),
            port,
        }
    }

    fn get_cur_time() -> i32 {
        let ts = Utc::now().timestamp();

        let time_since_1900 = ts + *DIFF;

        let time = (time_since_1900 & 0x00000000FFFFFFFF) as i32;
        time
    }

    fn handle_client(mut stream: TcpStream) -> io::Result<()> {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let response = Self::get_cur_time();
        stream.write(&i32_to_u8(response))?;
        stream.flush().unwrap();
        thread::sleep(time::Duration::from_secs(1));
        Ok(())
    }

    pub fn start(&self) -> Result<(), Error> {
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port)).unwrap();
        let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

        for stream in listener.incoming() {
            let stream = stream.expect("failed");
            let handle = thread::spawn(move || {
                Self::handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
            });
            thread_vec.push(handle);
        }

        for handle in thread_vec {
            handle.join().unwrap();
        }
        Ok(())
    }

    pub fn set_address(&mut self, address: &str) {
        self.address = address.to_string();
    }

    pub fn set_port(&mut self, port: u32) {
        self.port = port;
    }
}
