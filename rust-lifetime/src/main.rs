fn main() {
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
    println!("is String dropped >>>> {}", test); // compiler complains at this instance since
    // String struct does not implement Copy and has already moved
}

fn drop_item<T>(_item: T) {}
