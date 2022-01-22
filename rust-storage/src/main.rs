use bincode::{config::Configuration, Decode, Encode, encode_to_vec as to_bincode};
use serde::Serialize;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;

fn main() {
    let nairobi = City {
        name: String::from("Nairobi"),
        population: 4_700_000,
        latitude: 1.3107,
        longitude: 36.8250,
    };

    println!("City: {:?}", nairobi);

    let as_json = to_json(&nairobi).unwrap();
    let as_cbor = to_cbor(&nairobi).unwrap();

    // set bincode configuration
    let config = Configuration::standard();
    let as_bincode = to_bincode(&nairobi, config).unwrap();

    println!("json: \n {} \n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("bincode:\n{:?}\n", &as_bincode);

    // convert to string
    println!("json (as UTF-8):\n{}\n", String::from_utf8_lossy(as_json.as_bytes()));
    println!("cbor (as UTF-8):\n{:?}\n",String::from_utf8_lossy(&as_cbor));
    println!("bincode (as UTF-8):\n{:?}\n",String::from_utf8_lossy(&as_bincode));
}

#[derive(Debug, Serialize, Encode, Decode)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}
