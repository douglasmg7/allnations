use time::{offset, OffsetDateTime};

fn main() {
    let t = OffsetDateTime::now_utc().to_offset(offset!(-3));
    println!("{}", t.format("%FT%T%z"));

    let t2 = OffsetDateTime::parse("2019-02-22 20:12:34-0300", "%F %T%z").unwrap();
    println!("{}", t2.format("%F %T%z"));

    // let t = OffsetDateTime::now_utc().to_offset(UtcOffset::west_hours(3));
    // println!("t: {}", t.format("%FT%T.%N%z"));
}