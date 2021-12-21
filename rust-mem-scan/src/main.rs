fn main() {
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
