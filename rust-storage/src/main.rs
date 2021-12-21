use serde::Serialize;
use serde_json::to_string as to_json;


fn main() {
    let nairobi = City {
        name: String::from("Nairobi"),
        population: 4_700_000,
        latitude: 1.3107,
        longitude: 36.8250
    };

    println!("City: {:?}", nairobi);

    let as_json = to_json(&nairobi).unwrap();
}


#[derive(Debug, Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}
