use chrono::{DateTime, Local, Utc};

use crate::analyze::{self, MAC, Timestamp, UUIDAnalyze};

pub struct UUIDForge {
    // must precise to the nanosecond
    uuid_time: DateTime<Utc>,
    mac: MAC,
    version: u8,
    clock_seq: u16,
}

impl UUIDForge {
    pub fn new(mac: MAC, version: u8, clock_seq: u16) -> Self {
        let dt = Utc::now();
        Self {
            uuid_time: dt,
            mac,
            version,
            clock_seq,
        }
    }

    pub fn from_uuid(uuid: &str) -> Self {
        let uuid_analyze: UUIDAnalyze = analyze::UUIDAnalyze::from(uuid);
        Self::new(
            uuid_analyze.mac,
            uuid_analyze.version,
            uuid_analyze.clock_seq,
        )
    }

    pub fn with_custom_time(&mut self, time: DateTime<Utc>) -> &mut Self {
        self.uuid_time = time;
        self
    }

    pub fn forge(&self) -> String {
        let part_four = self.mac.to_string();
        let part_three = format!("{:02X?}", self.clock_seq);
        let timestamp = Timestamp::from_date_time(self.uuid_time);
        println!("timestamp: {:?}", timestamp);
        println!("time low : {}", timestamp.get_time_low());
        println!("time high : {}", timestamp.get_time_high());
        println!("time mid : {}", timestamp.get_time_mid());
        let part_zero = format!("{:02X?}", timestamp.get_time_low());
        let part_one = format!("{:02X?}", timestamp.get_time_mid());
        // the high field of the timestamp multiplexed with the version number.
        // set the 12 least significant bits of the time_high_and_version field (48 to 59) from the timestamp in the same order of significance
        // set the bits 12 to 15 of the time_high_and_version field to the 4-bit version number
        let time_high_and_version =
            timestamp.get_time_high() & 0x0FFF | ((self.version as u16 & 0xF) << 12);
        let part_two = format!("{:02X?}", time_high_and_version);

        format!(
            "{}-{}-{}-{}-{}",
            part_zero, part_one, part_two, part_three, part_four
        )
        .to_lowercase()
    }
}
