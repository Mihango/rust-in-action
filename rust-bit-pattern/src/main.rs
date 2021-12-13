#![allow(unused)]

mod lib;

use std::slice::from_ref;

fn main() {
    // shift
    let x = 2_i16;
    let y = x << 1;
    let z = x << 2;
    let a = x << 3;
    println!("{:016b} == x = {:016b} == {}", x, x, x);
    println!("{:016b} << 1 = {:016b} == {}", x, y, y);
    println!("{:016b} << 1 = {:016b} == {}", x, z, z);
    println!("{:016b} << 1 = {:016b} == {}", x, a, a);

    sample_integers();

    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}

/// having same bit pattern produce by different types
fn same_bit_pattern_diff_types() {
    let a = 50_115_u16;
    let b = -15_421_i16;

    println!("a : {:032b} {}", a, a);
    println!("b : {:032b} {}", b, b);
}

/// what happens if Rust treats a bit pattern produced by one type as another
/// below code demonstrates
fn float_bit_as_int() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{}", frankentype); // views f32 as decimal interger
    println!("{:032b}", frankentype); // format as binary with 32 zeros padded to the left --b
    // inovkes the binary trait

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };
    println!("{}", b);
    assert_eq!(a, b); // both are same : symentrical
}

fn integet_overflow() {
    let mut i = 0_u16;
    println!("{:016b} {}..", i, i);

    loop {
        i += 1000;
        println!("{:016b} {}..", i, i);
        if i % 10000 == 0 {
            print!{"\n"}
        }
    }
}

fn sample_integers() {
    let zero: u16 = 0b0000_0000_0000_0000;
    let one:  u16 = 0b0000_0000_0000_0001;
    let two:  u16 = 0b0000_0000_0000_0010;
    // ...
    let sixtyfivethousand_533: u16 = 0b1111_1111_1111_1101;
    let sixtyfivethousand_534: u16 = 0b1111_1111_1111_1110;
    let sixtyfivethousand_535: u16 = 0b1111_1111_1111_1111;
   
    print!("{}, {}, {}, ..., ", zero, one, two);
    println!("{}, {}, {}", sixtyfivethousand_533, sixtyfivethousand_534, sixtyfivethousand_535);
}
