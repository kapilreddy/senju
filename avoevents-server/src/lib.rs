#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]

use avoevents_commands::{Commands, CreateEventsModel};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

#[http_component]
fn handle_event(req: Request) -> anyhow::Result<impl IntoResponse> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();
    let mut router = Router::default();

    // register routes for queries
    // router.get("/avoevents", last_ten_events);

    // register routes for commands
    router.post("/avoevents", create_events);

    info!("Starting up the avoevents server!");
    // handle all the requests
    Ok(router.handle(req))
}

fn create_events(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(model) = serde_json::from_slice::<CreateEventsModel>(req.body()) else {
        return Ok(Response::builder().status(400).body(()).build());
    };
    let response = Commands::create_events(model)?;
    let payload = serde_json::to_vec(&response)?;
    return Ok(Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(payload)
        .build());
}
