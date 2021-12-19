#![allow(unused)]

fn main() {
    // same_bit_pattern();

    // bit_misrepresentation();

    integer_overflow();
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

