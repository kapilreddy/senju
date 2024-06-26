#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]

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
    router.post("/avoevents", create_event);

    info!("Starting up the avoevents server!");
    // handle all the requests
    Ok(router.handle(req))
}

fn create_event(_req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    return Ok(Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body("{\"message\": \"I'm new\"}")
        .build());
}
