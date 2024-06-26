PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS events (
    id integer PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    event_timestamp DATETIME NOT NULL,
    sensor_id text NOT NULL,
    node_id text NOT NULL,
    humidity integer NOT NULL
);
