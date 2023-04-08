use std::error::Error;

use url::Url;

fn get_port_number(url: &str) -> Option<u16> {
    let parsed_url = Url::parse(url).ok()?;
    let port = match parsed_url.port_or_known_default() {
        Some(port) => port,
        None => return None,
    };
    Some(port)
}
fn set_port_number(url: &str, port: u16) -> Result<String, Box<dyn Error>> {
    let mut parsed_url = Url::parse(url)?;
    parsed_url
        .set_port(Some(port))
        .map_err(|_| "cannot be base")?;
    Ok(parsed_url.to_string())
}
fn main() {}
