fn main() {
    let mut grains: Vec<Cereals> = vec![];
    grains.push(Cereals::Rice);
    drop(grains);
    println!("{:?}", grains); // fails to compile since the pointer grain have been 
    // moved and dropped at method call drop
}

#[derive(Debug)]
enum Cereals {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}
