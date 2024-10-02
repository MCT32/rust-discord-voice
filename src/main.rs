use serde_json::Value;
use log::debug;

fn main() {
    // Set up logger
    simple_logger::init_with_level(
        if cfg!(debug_assertions) {
            log::Level::Trace
        } else {
            log::Level::Info
        }
    ).unwrap();

    // TODO: Cache gateway url like specified in documentation
    let resp = serde_json::from_str::<Value>(
        reqwest::blocking::get("https://discord.com/api/v10/gateway").unwrap()
            .text().unwrap()
            .as_str()
    ).unwrap();

    let gateway_url = resp.get("url").unwrap()
        .as_str().unwrap();

    debug!("Gateway url is {}", gateway_url)
}
