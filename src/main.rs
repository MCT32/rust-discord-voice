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
    ).expect("Failed to initialise logger");

    // TODO: Cache gateway url like specified in documentation
    let resp = serde_json::from_str::<Value>(
        reqwest::blocking::get("https://discord.com/api/v10/gateway")
            .expect("Failed to get discord gateway url")
            // TODO: Add a test to make sure responce was ok (2xx)
            .text().expect("Responce from discord gateway endpoint had no body")
            .as_str()
    ).expect("Discord gateway endpoint responce could not be parsed as json");

    let gateway_url = resp.get("url").expect("Discord gateway endpoint reponse had to url key")
        .as_str().expect("Discord gateway endpoint responce had an unexpected type");

    debug!("Gateway url is {}", gateway_url)


    
}
