#![allow(unused_variables)] 
use std::vec;

fn main() {
    // let mut f2 = File {
    //     name: String::from("2.txt"),
    //     data: vec![114, 117, 116, 33],
    // };

    let mut f2 = File::new("2.txt");
    f2.data = vec![114, 117, 116, 33];

    let mut buffer: Vec<u8> = vec![];
    open(&mut f2);
    let f_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f_length);
    println!("{}", text);

}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File { name: String::from(name), 
            data: Vec::new() 
        }
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}