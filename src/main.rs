use html_parser::Result;
use serde_json;

mod clients;
mod models;
mod parsers;

use crate::clients::elcano_client::request_provider;
use crate::parsers::elcano_parser::parse_elcano_departures;

fn main() -> Result<()> {
    let departure_station = "SABADELL NORD";

    let provider_response = request_provider(&departure_station);
    let train_departures = parse_elcano_departures(&provider_response, &departure_station);

    println!("{}", serde_json::to_string(&train_departures).unwrap());
    Ok(())
}
