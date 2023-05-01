use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONNECTION, USER_AGENT};
use std::collections::HashMap;
use std::{error::Error, str};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StationInfo {
    station_id: String,
    description: String,
    lat: String,
    lon: String,
    address: String,
    postal_code: String,
    city: String,
    province: String,
    country: String,
    is_cercanias: String,
    is_feve: String,
}


fn get_station_id_from_name(station_name: &str) -> Result<String, Box<dyn Error>>  {
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).delimiter(b';').from_path("src/clients/listado-estaciones-completo.csv")?;
    let mut station_hashmap = HashMap::<String, String>::new();

    for station_data in rdr.deserialize() {
        let station: StationInfo = station_data?;
        station_hashmap.insert(station.description, station.station_id);
    }

    let station_id = match station_hashmap.get(station_name) {
        Some(station_id) => station_id,
        None => return Err("Could not find station".into())
    };

    return Ok(station_id.to_string());
}


pub fn request_provider(departure_station_name: &str) -> String {
    let station_id = get_station_id_from_name(departure_station_name).unwrap();
    let client = Client::new();

    let url = "https://elcanoweb.adif.es/departures/reload";

    let mut map = HashMap::new();
    map.insert("key", "value");

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
