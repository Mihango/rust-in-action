#![allow(unused)]
fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    // println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    let a_status = check_status(sat_a); // does not compile - used after move error
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    // println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}

fn drop_item<T>(_item: T) {}

fn test_primitives_borrow() {
    let num = 0;
    drop_item(num);
    println!("item >>> {}", num); // primitive values implement Copy trait so they can be used
                                  // later after deallocation

    let name = "Testing slice";
    drop_item(name);
    println!("is name dropped >>> {}", name); // string slice does not droop too...implements
                                              // Copy trait

    let test = String::from(name);
    drop_item(test);
    // println!("is String dropped >>>> {}", test); cannot compile: compiler complains at this instance since
    // String struct does not implement Copy and has already moved
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

// return the borrowed item
fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

// can derive Clone and Copy
#[derive(Debug)]
struct CubeSat {
    id: u64
}
