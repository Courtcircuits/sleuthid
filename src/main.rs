use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};

pub mod analyze;
pub mod forge;
fn main() {
    let to_analyze: Vec<&str> = vec![
        "6f0595f4-2679-11f0-be4a-0242ac100011",
        "2c8e6343-2676-11f0-be4a-0242ac100011",
    ];

    // Parse the date string into a DateTime object
    let parsed_date =
        NaiveDateTime::parse_from_str("2025-05-01 02:09:53.910365", "%Y-%m-%d %H:%M:%S%.f")
            .unwrap();
    let utc_parsed_date = Utc.from_utc_datetime(&parsed_date);

    let uuid = analyze::UUIDAnalyze::from(to_analyze[0]);
    let to_forge = forge::UUIDForge::from_uuid("2c8e6343-2676-11f0-be4a-0242ac100011")
        .with_custom_time(utc_parsed_date)
        .forge();
    println!("{:?}", to_forge);
}
