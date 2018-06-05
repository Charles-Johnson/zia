use std::io;
extern crate zia;
use zia::oracle;
extern crate zia2sql;
use zia2sql::connect;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conn = connect(&args);
    let input = io::stdin();
    let mut buffer = String::new();
    input.read_line(&mut buffer).expect("Could not read line");
    while buffer != "" {
        println!("{:?}", oracle(&buffer, &conn));
        buffer = String::new();
        input.read_line(&mut buffer).expect("Could not read line");
    } 
}

