#![allow(unused_variables)]
use std::vec;
use rand::prelude::*;

fn main() {
    // let mut f2 = File {
    //     name: String::from("2.txt"),
    //     data: vec![114, 117, 116, 33],
    // };

    let mut f2 = File::new("2.txt");
    f2.data = vec![114, 117, 116, 33];

    let f3 = File::new_with_data("3.txt", &vec![114, 117, 116, 33]);

    let mut buffer: Vec<u8> = vec![];
    f2 = open(f2).unwrap();
    let f_length = f2.read(&mut buffer);
    f2 = close(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f_length.unwrap());
    println!("{}", text);
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        // File {
        //     name: String::from(name),
        //     data: data.clone(),
        // }
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
    
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10_000){
        let err_msg = String::from("Permisssion denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}