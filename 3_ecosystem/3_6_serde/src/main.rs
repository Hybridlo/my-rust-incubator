use std::{time::Duration, fs};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use url::Url;
use uuid::Uuid;


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all="lowercase")]
enum RequestType {
    Success,
    Failure
}

#[derive(Serialize, Deserialize)]
struct TariffCommon {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String
}

#[derive(Serialize, Deserialize)]
struct PublicTariff {
    id: u64,
    price: u64,
    #[serde(flatten)]
    common_info: TariffCommon
}

#[derive(Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u64,
    #[serde(flatten)]
    common_info: TariffCommon
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
struct Gift {
    id: u64,
    price: u64,
    description: String
}

#[derive(Serialize, Deserialize)]
struct DebugInfo {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    #[serde(with = "time::serde::iso8601")]
    at: OffsetDateTime
}

#[derive(Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u64,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Serialize, Deserialize)]
struct Request {
    #[serde(rename="type")]
    req_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: DebugInfo
}

fn main() {
    let req_json = fs::read_to_string("request.json").expect("File must exist");

    let serialized: Request = serde_json::from_str(&req_json).expect("Json should be able to be parsed");

    println!("{}", serde_yaml::to_string(&serialized).expect("Data to be serializable to YAML"));
    println!("{}", toml::to_string(&serialized).expect("Data to be serializable to TOML"));
}

#[cfg(test)]
mod tests {
    use time::{UtcOffset, Month};
    use uuid::uuid;

    use super::*;

    #[test]
    fn test_request() {
        let req_json = fs::read_to_string("request.json").expect("File must exist");

        let serialized: Request = serde_json::from_str(&req_json).expect("Json should be able to be parsed");
        assert_eq!(serialized.req_type, RequestType::Success);

        assert_eq!(serialized.stream.user_id, uuid!("8d234120-0bda-49b2-b7e0-fbd3912f6cbf"));
        assert_eq!(serialized.stream.is_private, false);
        assert_eq!(serialized.stream.settings, 45345);
        assert_eq!(serialized.stream.shard_url, "https://n3.example.com/sapi".parse().unwrap());

        assert_eq!(serialized.stream.public_tariff.id, 1);
        assert_eq!(serialized.stream.public_tariff.price, 100);
        assert_eq!(serialized.stream.public_tariff.common_info.duration, Duration::from_secs(3600));
        assert_eq!(serialized.stream.public_tariff.common_info.description, "test public tariff");
        
        assert_eq!(serialized.stream.private_tariff.client_price, 250);
        assert_eq!(serialized.stream.private_tariff.common_info.duration, Duration::from_secs(60));
        assert_eq!(serialized.stream.private_tariff.common_info.description, "test private tariff");
        
        assert!(serialized.gifts.contains(&Gift { id: 1, price: 2, description: "Gift 1".to_string() }));
        assert!(serialized.gifts.contains(&Gift { id: 2, price: 3, description: "Gift 2".to_string() }));
        
        assert_eq!(serialized.debug.at.year(), 2019);
        assert_eq!(serialized.debug.at.month(), Month::June);
        assert_eq!(serialized.debug.at.day(), 28);
        assert_eq!(serialized.debug.at.hour(), 8);
        assert_eq!(serialized.debug.at.minute(), 35);
        assert_eq!(serialized.debug.at.second(), 46);
        assert_eq!(serialized.debug.at.offset(), UtcOffset::UTC);
        assert_eq!(serialized.debug.duration, Duration::from_millis(234));
    }
}