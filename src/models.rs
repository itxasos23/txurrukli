use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainDeparture {
    pub time: String,
    pub destination: String,
    pub service: String,
    pub station: String,
    pub platform: Option<String>,
    pub date: String,
}
