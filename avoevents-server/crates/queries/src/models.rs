use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EventListModel {
    pub id: u64,
    #[serde(rename = "timestamp")]
    pub event_timestamp: i64,
    pub created_at: i64,
    pub sensor_id: String,
    pub node_id: String,
    pub humidity: i64, // [ref: humidity_has_a_small_value]
}
