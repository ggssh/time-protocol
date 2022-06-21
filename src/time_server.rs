use chrono::{NaiveDate, Utc};

lazy_static! {
    // 1900和1970之间的timestamp差值
    static ref DIFF: i64 = NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp()
        .abs();
}

pub fn get_cur_time() -> i32 {
    let ts = Utc::now().timestamp();
    // println!("ts: {}", ts);

    let time_since_1900 = ts + *DIFF;

    let time = (time_since_1900 & 0x00000000FFFFFFFF) as i32;
    time
}

