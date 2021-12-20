pub fn reference_coercion_to_value() {
    let a = 42_i64;
    let a_ptr = &a as *const i64;

    println!("a: {} ({:p})", a, a_ptr);
}