#![allow(unused)]

use std::{mem::size_of, borrow::Cow};
use std::ffi::CStr;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    // simple_pointers();

    // smart_pointers();

    memory_address_layout();
}

// pointers
fn simple_pointers() {
    let a = 42;
    let b = &B;
    let c = &C;

    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}

// smart pointer
fn smart_pointers() {
    let a = 42_usize;
    let b = &B;
    let c = Box::new(C);

    println!("a (an unsigned interger):");
    println!("  location: {:p}", &a);
    println!("  size:     {:?} bytes", size_of::<usize>());
    println!("  value:    {:?}", a);
    println!();

    println!("b (a reference to B):");
    println!("  location: {:p}", &b);
    println!("  size:     {:?} bytes", size_of::<&[u8; 10]>());
    println!("  value:    {:?}", b);
    println!();

    println!("c (a 'box' for C):");
    println!("  location: {:p}", &c);
    println!("  size:     {:?} bytes", size_of::<Box<[u8]>>());
    println!("  value:    {:?}", c);
    println!();

    println!("B (an array of 10 bytes):");
    println!("  location: {:p}", &B);
    println!("  size:     {:?} bytes", size_of::<[u8; 10]>());
    println!("  value:    {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!("  location: {:p}", &C);
    println!("  size:     {:?} bytes", size_of::<[u8; 11]>());
    println!("  value:    {:?}", C);
    println!();
}

fn memory_address_layout() {
    let a = 42;
    let b: String;
    let c: String;
    let d: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);

        let b_ptr = &C as *const u8 as *mut u8;
        c = String::from_raw_parts(b_ptr, 10, 10);
        
        let c_ptr = &C as *const u8 as *const c_char;
        d = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);
    let d = 10 * 12;
    println!("10 * 12: {}", d);
}
