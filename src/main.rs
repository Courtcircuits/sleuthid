use chrono::{NaiveDateTime, TimeZone, Utc};
use clap::{Parser, Subcommand};

pub mod analyze;
pub mod forge;

// A analysis and exploitation tool for uuidv1 based on rfc4122
#[derive(Parser, Debug)]
#[clap(name = "sleuthid", version)]
struct Sleuthid {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, PartialEq, Eq, Hash)]
pub enum Command {
    // decode a give UUID(v1) and returns as JSON the timestamp, mac address, version and clock sequence
    Analyze {
        // a uuid v1
        uuid: String,
    },
    // generates a UUIDv1 from a given date+time (precise to the nanosecond) and a UUID. The mac address, version and clock sequence of the give UUID will be kept in the generated UUID
    Forge {
        // offset in nanoseconds to compensate for clock drift
        #[clap(long, default_value = "0", short = 'o')]
        offset: i32,
        // number of UUIDs to generate to compensate for clock drift
        #[clap(long, default_value = "1", short = 'n')]
        number: i32,
        // Date time at the format : %Y-%m-%d %H:%M:%S%.f
        #[clap(long, short = 't')]
        time: String,
        // a uuid v1
        uuid: String,
    },
}

fn main() {
    let args: Sleuthid = Sleuthid::parse();

    match args.command {
        Command::Analyze { uuid } => {
            let uuid_analyze = analyze::UUIDAnalyze::from(uuid.as_str());
            let json_analyze = serde_json::to_string(&uuid_analyze).unwrap();
            println!("{}", json_analyze);
        }
        Command::Forge {
            time,
            uuid,
            number,
            offset,
        } => {
            let parsed_date =
                NaiveDateTime::parse_from_str(time.as_str(), "%Y-%m-%d %H:%M:%S%.f").unwrap();
            let utc_parsed_date = Utc.from_utc_datetime(&parsed_date);
            for i in 0..number {
                let to_forge = forge::UUIDForge::from_uuid(uuid.as_str())
                    .with_custom_time(utc_parsed_date)
                    .forge(i - offset);
                println!("{}", to_forge);
            }
        }
    }
}
