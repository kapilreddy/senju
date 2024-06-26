use serde::{Deserialize, Serialize};

/// Each request will have multiple incoming events
#[derive(Debug, Deserialize)]
pub struct CreateEventsModel {
    pub events: Vec<EventModel>,
    pub timestamp: i64,
    pub node_id: String,
}

/// An incoming new event needs to conform to this model
#[derive(Debug, Deserialize)]
pub struct EventModel {
    #[serde(rename = "timestamp")]
    pub event_timestamp: i64,
    pub sensor_id: String,
    pub node_id: String,
    pub humidity: i64, // @TODO: Figure out how to make this i8 later
}

/// for each event we store, we return the ID of the event
#[derive(Debug, Serialize)]
pub struct EventCreatedModel {
    pub id: i64,
}

/// Our final response to the actual request is all the newly created ids.
#[derive(Debug, Serialize)]
pub struct EventsCreatedModel {
    pub events: Vec<EventCreatedModel>,
}
