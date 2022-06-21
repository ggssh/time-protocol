use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
// use chrono::TimeZone;
use chrono::Utc;

lazy_static! {
    // 1900和1970之间的timestamp差值
    pub static ref DIFF: i64 = NaiveDate::from_ymd(1900, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp()
        .abs();

    pub static ref CN_TIMEZONE:FixedOffset =FixedOffset::east(8 * 3600);
}

fn get_time() {
    let now = Utc::now();
    println!(
        "UTC now in a custom format is: {}",
        now.format("%a %b %e %T %Y")
    );

    let m = now.timestamp();
    println!("{}", &m);

    // let ms_since_1970 = now.timestamp();
    // println!("{}", ms_since_1970);
    // println!("{}", NaiveDateTime::from_timestamp(ms_since_1970, 0));
    // let dt = now.with_timezone(&china_time_zone);
    // println!("{}",dt);
    // let ms_china = dt.timestamp();
    // println!("{}",ms_china);
    // println!("{}", NaiveDateTime::from_timestamp(ms_china, 0))
}

pub fn send_time() {
    let ts = Utc::now().timestamp();
    println!("ts: {}", ts);

    let time_since_1900 = ts + *DIFF;

    let time = (time_since_1900 & 0x00000000FFFFFFFF) as i32;
    println!("time: {:X}", &time);

    let from_epoch = (time as i64 & 0xFFFFFFFF as i64) - *DIFF;
    // println!("from_epoch: {:X}", &from_epoch);
    // println!("diff: {}", &diff);
    let dt = NaiveDateTime::from_timestamp(from_epoch, 0);

    let now = DateTime::<Utc>::from_utc(dt, Utc).with_timezone(&*CN_TIMEZONE);
    println!("now: {}", now);
}