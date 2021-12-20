#![allow(unused)]

mod q7lib;

fn main() {
    // same_bit_pattern();

    // bit_misrepresentation();

    // integer_overflow();

    // float_sign_bit_isolation();
    // float_exponent_isolation();
    float_mantissa_isolation();
    float_mantissa_isolation_2();
    rust_float_isolation();

}

fn same_bit_pattern() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}

fn bit_misrepresentation() {
    let a = 42.42_f32;
    let franken_type: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{}", franken_type);
    println!("{:032b}", franken_type);

    let b: f32 = unsafe {
        std::mem::transmute(franken_type)
    };

    println!("{}", b);
    assert_eq!(a, b);
}

fn integer_overflow() {
    let mut i = 0_u16;
    print!("{}..", i);

    loop {
        i += 1000;
        print!("{}..", i);
        if i % 10000 == 0 {
            print!("\n");
        }
    }
}

fn float_sign_bit_isolation() {
    let n = 42.42_f32;
    let n_bits = n.to_bits();
    let sign_bit = n_bits >> 31;

    println!("Sign bit >>> {}", sign_bit);
}

/// to isolate the exponent, two bit manipulations are required. Perfom right shift
/// to overwrite the mantissa's bits (>> 23). Then use and Amd mask (& 0xff) to exclude
/// the sign bit. And mask only allows the right most bits to pass through
fn float_exponent_isolation() {
    let n = 42.42_f32;
    let n_bits = n.to_bits();
    let exponent_with_sign = n_bits >> 23;
    println!("Exponent with sign {:b}", exponent_with_sign);
    let float: u32 = unsafe { std::mem::transmute(exponent_with_sign) };
    println!("Transmuted value {}", float);
    let exponent = exponent_with_sign & 0xff;
    let exponent = (exponent as i32) - 127;
    println!("Exponent value : {}", exponent);
}

/// to decode mantissa there are two ways:
/// 1. use AND mask to remove the sign bit and exponent (& 0x7fffff)
///
/// 2. multiply each bit by its weight and sum the result. The first bit's weight is 0.5,
/// and each subsequent bit's weight is half of the current weight; for example 0.5(2^-1), 0.25(2^-2)...
/// 0.000000119209(2^-23). An implicit 24th bit that represents 1.0(2^-0) is always considered to be on,
/// except when special cases are t
fn float_mantissa_isolation() {
    let n = 42.42_f32;
    let n_bits = n.to_bits();
    let mut mantissa = n_bits & 0x7fffff;
    println!("Mantissa {}", mantissa);
}

fn float_mantissa_isolation_2() {
    let n = 42.42_f32;
    let n_bits = n.to_bits();
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    println!("Mantissa >>> 2: {}", mantissa);
}

/// the above 3 methods can be combined as below to use rust
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn rust_float_isolation() {
    let n = 42.42_f32;

    let(sign, exp, frac) = to_parts(n);
    let(sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!("field     | as bits   | as real number");
    println!("sign      |       {:01b}  |   {}", sign, sign_);
    println!("exponent  |   {:08b}  |   {}", exp, exp_);
    println!("mantissa  |   {:023b} |   {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);
    let mut mantissa = 1.0;

    for index in 0..23 {
        let mask = 1 << index;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = index as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    (signed_1, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

