use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONNECTION, USER_AGENT};
use std::collections::HashMap;
use std::str;

pub fn request_provider(departure_station: &str) -> String {
    let client = Client::new();

    let url = "https://elcanoweb.adif.es/departures/reload";

    let mut map = HashMap::new();
    map.insert("key", "value");

    let station_id = if departure_station == "SABADELL NORD" {
        "78709"
    } else {
        panic!(
            "Unexpected departure station: {}. Expected 'SABADELL NORD'",
            departure_station
        );
    };

    map.insert("station", &station_id);
    map.insert("dest", "");
    map.insert("previous", "1");
    map.insert("showCercanias", "true");
    map.insert("showOtros", "false");
    map.insert("iframe", "false");
    map.insert("isNative", "true");

    let request = client
        .post(url)
        .header(AUTHORIZATION, "Basic ZGVpbW9zOmRlaW1vc3R0")
        // .header(ACCEPT_ENCODING, "gzip, deflate")
        .header(ACCEPT, "*/*")
        .header(USER_AGENT, "rust-reqwest")
        .header(CONNECTION, "keep-alive");

    let request = request.form(&map);
    let response = request.send().unwrap();
    let provider_response = String::from_utf8((&response.bytes().unwrap()).to_vec()).unwrap();
    provider_response
}
