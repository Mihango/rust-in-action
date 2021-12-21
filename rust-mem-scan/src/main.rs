#![allow(unused)]

use std::f32::consts::PI;

fn main() {
    memory_scan();
}

fn memory_scan_with_fault() {
    let mut n_non_zero = 0;

    for i in 1..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr !=0 {
            n_non_zero += 1;
        }
    }
    println!("non-zero bytes in memory: {}", n_non_zero);
}

static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

fn memory_scan() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();

    println!("GLOBAL:    {:p}", &GLOBAL as *const i32);
    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const i32);
    println!("boxed_int: {:p}", Box::into_raw(boxed_int));
    println!("boxed_str: {:p}", Box::into_raw(boxed_str));
    println!("fn_int:    {:p}", fn_int);

    let pointer = &GLOBAL as *const i32;
    let result: usize = unsafe {
        std::mem::transmute(pointer)
    };
    unsafe {
        println!("Result from pointer {:p} is {}", pointer, *pointer);
    }
}