use std::fmt;

use chrono::{DateTime, Utc};
/*
Example of UUID v1 : 2c8e6343-2676-11f0-be4a-0242ac100011
[time-low]-[time-mid]-[time-high-and-version]-[clock-seq-and-reserved clock-seq-low]-[node]
time-low               = 4hexOctet
time-mid               = 2hexOctet
time-high-and-version  = 2hexOctet
clock-seq-and-reserved = hexOctet e.g 0xbe
clock-seq-low          = hexOctet e.g 0x4a these two sections are concatenated
node                   = 6hexOctet


Source : https://datatracker.ietf.org/doc/html/rfc4122#section-3
*/

pub const OFFSET: u64 = 122192928000000000;

pub type RawTimestamp = u64;
pub struct Timestamp(RawTimestamp);

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timestamp = self.0;
        println!("Timestamp: {}", timestamp);
        let dt: DateTime<Utc> = DateTime::from_timestamp_nanos(timestamp as i64);
        write!(f, "{}", dt.to_rfc2822())
    }
}

impl Timestamp {
    pub fn from_date_time(datetime: DateTime<Utc>) -> Timestamp {
        let timestamp_ns: u64 = match datetime.timestamp_nanos_opt() {
            Some(val) => val as u64,
            None => 0 as u64,
        };
        println!("Timestamp: {}", timestamp_ns);

        let full_time: u64 = (timestamp_ns / 100 + OFFSET);

        Timestamp(full_time)
    }

    pub fn get_time_low(&self) -> u32 {
        let time_low = (self.0 & 0x00000000FFFFFFFF) as u32;
        time_low - 1
    }

    pub fn get_time_mid(&self) -> u16 {
        let time_mid = ((self.0 & 0x0000FFFF00000000) >> 32) as u16;
        time_mid
    }

    pub fn get_time_high(&self) -> u16 {
        let time_high = ((self.0 & 0xFFFF000000000000) >> 48) as u16;
        time_high
    }
}

pub type RawMAC = [u8; 6];

pub struct MAC(RawMAC);
impl fmt::Debug for MAC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut index = 0;
        for byte in self.0 {
            match index {
                5 => {
                    write!(f, "{:02X?}", byte)?;
                }
                _ => {
                    write!(f, "{:02X?}:", byte)?;
                }
            };
            index += 1;
        }
        Ok(())
    }
}
impl MAC {
    pub fn to_string(&self) -> String {
        let mut res: String = "".to_string();
        for byte in self.0 {
            res = format!("{}{:02X?}", res, byte);
        }
        res.to_string()
    }
}

#[derive(Debug)]
pub struct UUIDAnalyze {
    // ns timestamp from 1970
    pub timestamp: Timestamp,
    // mac address
    pub mac: MAC,
    //version
    pub version: u8,
    //clock sequence
    pub clock_seq: u16,
}

impl From<&str> for UUIDAnalyze {
    fn from(s: &str) -> Self {
        let parts = s.split("-").collect::<Vec<&str>>();
        let time_high_and_version = u64::from_str_radix(parts[2], 16).unwrap();
        let version = ((time_high_and_version >> 12) & 0b1111) as u8;
        let time_high = (time_high_and_version) & 0b111111111;
        let time_low = u64::from_str_radix(parts[0], 16).unwrap();
        let time_mid = u64::from_str_radix(parts[1], 16).unwrap();
        let node = parts[4];
        let clock_seq = u16::from_str_radix(parts[3], 16).unwrap();

        //  The timestamp is a 60-bit value.  For UUID version 1, this is
        // represented by Coordinated Universal Time (UTC) as a count of 100-
        // nanosecond intervals since 00:00:00.00, 15 October 1582 (the date of
        // Gregorian reform to the Christian calendar).

        // Sequentially number the bits in a field,
        // starting with zero for the least significant bit.
        // Set the time_low field equal to the least significant 32 bits
        // (bits zero through 31) of the timestamp in the same order of
        // significance.
        let full_time: u64 = ((time_low | (time_mid) << 32 | (time_high) << 48) - OFFSET) * 100;

        // For UUID version 1, the node field consists of an IEEE 802 MAC
        // address, usually the host address.  For systems with multiple IEEE
        // 802 addresses, any available one can be used.  The lowest addressed
        // octet (octet number 10) contains the global/local bit and the
        // unicast/multicast bit, and is the first octet of the address
        // transmitted on an 802.3 LAN.
        let result_mac_vec: Vec<u8> = node
            .as_bytes()
            .chunks(2)
            .map(|chunk| {
                let hex_str = std::str::from_utf8(chunk).unwrap();
                u8::from_str_radix(hex_str, 16).unwrap()
            })
            .collect();
        let mac: [u8; 6] = result_mac_vec.try_into().unwrap();

        Self {
            version,
            timestamp: Timestamp(full_time),
            clock_seq,
            mac: MAC(mac),
        }
    }
}
