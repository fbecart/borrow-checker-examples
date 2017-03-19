use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("/etc/hosts").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    drop(file);
    println!("{}", contents);

    // Try closing the file a second time, just for fun
    //drop(file);
}
