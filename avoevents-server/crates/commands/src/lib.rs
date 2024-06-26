#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]

//! This crate contains commands of the event data

mod models;
use chrono::{DateTime, Utc};
pub use models::{CreateEventsModel, EventCreatedModel, EventModel, EventsCreatedModel};
use spin_sdk::sqlite::{Connection, Value};

const DB_NAME: &str = "default";
const COMMAND_CREATE_EVENT: &str =
    "INSERT INTO events (event_timestamp, sensor_id, node_id, humidity) VALUES (?, ?, ?, ?) RETURNING id;";

/// This struct contains all the commands that are allowed to run on the avoevents-server
pub struct Commands {}

fn convert_unix_secs_to_timestamp_str(datetime: i64) -> anyhow::Result<String> {
    let datetime_utc: DateTime<Utc> = DateTime::from_timestamp(datetime, 0).unwrap();
    Ok(datetime_utc.format("%Y-%m-%d %H:%M:%S").to_string())
}

impl Commands {
    /// Command to store events sent by our sensors
    pub fn create_events(model: CreateEventsModel) -> anyhow::Result<EventsCreatedModel> {
        let con = Connection::open(DB_NAME)?;
        let mut response_data = Vec::new();
        let _ = con.execute("BEGIN TRANSACTION;", &[]);
        for event in model.events {
            let event_params = [
                Value::Text(
                    convert_unix_secs_to_timestamp_str(event.event_timestamp.clone()).unwrap(),
                ),
                Value::Text(event.sensor_id.clone()),
                Value::Text(event.node_id.clone()),
                Value::Integer(event.humidity.clone()),
            ];
            let query_result = con.execute(COMMAND_CREATE_EVENT, &event_params)?;
            let row = query_result.rows().next().unwrap();
            let id = row.get("id").unwrap();
            response_data.push(EventCreatedModel { id });
        }
        let _ = con.execute("END TRANSACTION;", &[]);
        Ok(EventsCreatedModel {
            events: response_data,
        })
    }
}
