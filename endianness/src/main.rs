#![allow(unused)]

use std::intrinsics::transmute;

fn main() {
    isolate_sign_bit();
}

fn sample_endians() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };

    println!("{} vs {}", a, b);
}

fn isolate_sign_bit() {
    let n = 42.42_f32;
    let n_bits = n.to_bits();
    let sign_bit = n_bits >> 31;

    println!("{}", n);
    println!("{:032b}", n_bits);
    println!("{}", sign_bit);
}
