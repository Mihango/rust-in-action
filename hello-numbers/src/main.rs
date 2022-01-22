fn main() {
    // addition
    let twenty = 20;
    let twenty_one = 21;
    let twenty_two = 22_i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);
    
    let one_million = 1_000_000_i64;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32
    ];
    println!("{:02}", forty_twos[0]);

    base_numerics();
}

fn base_numerics() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10 {} {} {}", three, thirty, three_hundred);
    println!("base 2 {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8 {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16 {:x} {:x} {:x}", three, thirty, three_hundred);
}
