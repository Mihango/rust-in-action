pub mod samples;

pub fn reference_coercion_to_value() {
    let a = 42_i64;
    let a_ptr = &a as *const i64;

    println!("a: {} ({:p})", a, a_ptr);
}

pub fn derefencing_pointer() {
    let a = 42_i64;
    let a_ptr = &a as *const i64; // cast a as a raw pointer

    let a_addr: usize = unsafe {
        std::mem::transmute(a_ptr)
    };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
}

pub fn create_raw_pointer() {
    let ptr = 42 as *const Vec<String>;
    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}