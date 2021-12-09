fn main() {
    loop_label_sample();
}

fn loop_label_sample() {
    'outer: for i in 3..6 {
        'inner: for j in 10..20 {
            let k = i + j;
            println!("{} + {} = {}", i, j, k);
            if k == 17 {
                break 'inner;
            }
        }
    }
}
