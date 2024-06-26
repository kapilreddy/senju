#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_docs)]

//! This crate contains queries of the event data
mod models;
use anyhow::anyhow;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::debug;
use models::EventListModel;
use spin_sdk::sqlite::Connection;

const DB_NAME: &str = "default";
const QUERY_COMMAND: &str = "SELECT * FROM events ORDER BY id DESC LIMIT 10;";

/// The Queries struct encapsulates available queries
pub struct Queries {}

fn convert_timestamp_str_to_unix_secs(datetime_str: String) -> anyhow::Result<i64> {
    let naive_datetime = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")?;
    let datetime_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
    Ok(datetime_utc.timestamp())
}

impl Queries {
    /// Query to retrieve all products
    pub fn last_ten_events() -> anyhow::Result<Vec<EventListModel>> {
        let con = Connection::open(DB_NAME)?;
        let query_result = con.execute(QUERY_COMMAND, &[])?;
        let count = query_result.rows().count();
        let events = query_result
            .rows()
            .map(|row| {
                let event_timestamp_str = String::from(
                    row.get::<&str>("event_timestamp")
                        .ok_or_else(|| anyhow!("event_timestamp missing!"))?,
                );
                let created_at_str = String::from(
                    row.get::<&str>("created_at")
                        .ok_or_else(|| anyhow!("created_at not present"))?,
                );
                anyhow::Ok(EventListModel {
                    id: row
                        .get::<u64>("id")
                        .ok_or_else(|| anyhow!("id not present"))?,
                    event_timestamp: convert_timestamp_str_to_unix_secs(event_timestamp_str)
                        .unwrap(),
                    created_at: convert_timestamp_str_to_unix_secs(created_at_str).unwrap(),
                    sensor_id: String::from(
                        row.get::<&str>("sensor_id")
                            .ok_or_else(|| anyhow!("sensor_id not present"))?,
                    ),
                    node_id: String::from(
                        row.get::<&str>("node_id")
                            .ok_or_else(|| anyhow!("node_id not present"))?,
                    ),
                    humidity: row
                        .get::<i64>("humidity")
                        .ok_or_else(|| anyhow!("id not present"))?,
                })
            })
            .filter(|item| item.is_ok())
            .map(|item| item.unwrap())
            .collect();
        debug!("Events: {:?}, Count: {}", events, count);
        Ok(events)
    }
}
