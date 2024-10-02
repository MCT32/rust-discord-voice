use serde_json::Value;

fn main() {
    let resp = serde_json::from_str::<Value>(
        reqwest::blocking::get("https://discord.com/api/v10/gateway").unwrap()
            .text().unwrap()
            .as_str()
    ).unwrap();

    let gateway_url = resp.get("url").unwrap()
        .as_str().unwrap();

    println!("{}", gateway_url);
}
