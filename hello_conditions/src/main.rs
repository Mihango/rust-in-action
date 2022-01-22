fn main() {
    conditional_if();
    conditional_match();
    pattern_aware_match(23);
    sample_ref();
    arr_ref();
}

fn conditional_if() {
    let n = 123456;

    let description = if is_even(n) {
        "even"
    } else {
        "odd"
    };

    println!("{} is {}", n, description);
}

fn conditional_match() {
    let n = 98843;
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };

    println!("{} is {}", n, description);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn pattern_aware_match(item: i32) {
    let answer = match item {
        0 => "Baby",
        10..=20 => "Teen",
        40 | 80 => "Not Allowed",
        _ => "Allowed",
    };
    println!("{} is {}", item, answer);
}

fn sample_ref() {
    let a = 42;
    let r = &a;
    let b = a + *r;
    println!("a + a = {}",b)
}

fn arr_ref() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 52, 877, 4140, 132];

    for item in &haystack { 
        // comparing item == needle throw compile error cannot compare
        // between {integer} and &{integer}
        if *item == needle {
            println!("Needle item >>> {}", item);
        }
    }
}