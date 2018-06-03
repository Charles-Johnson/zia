use std::io;
extern crate zia;
use zia::oracle;

fn main() {
    let input = io::stdin();
    let mut buffer = String::new();
    input.read_line(&mut buffer).expect("Could not read line");
    while buffer != "" {
        println!("{:?}", oracle(&buffer[..]));
        buffer = String::new();
        input.read_line(&mut buffer).expect("Could not read line");
    } 
}

