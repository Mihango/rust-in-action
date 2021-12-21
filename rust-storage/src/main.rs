use serde::Serialize;


fn main() {
    let nairobi = City {
        name: String::from("Nairobi"),
        population: 4_700_000,
        latitude: 1.3107,
        longitude: 36.8250
    };

    println!("City: {:?}", nairobi);
}


#[derive(Debug, Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}
