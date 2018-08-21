use std::io;
use std::process::exit;
extern crate zia;
use zia::oracle;
extern crate zia2sql;
use zia2sql::connect;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (conn, _file_conn) = match connect(&args)
        {Ok(x) => x,
         Err(e) => {eprintln!("{}", e);
                    exit(0);}
         };
    let input = io::stdin();
    let mut buffer = String::new();
    read_line(&input, &mut buffer);
    while buffer != "" {
        buffer = String::new();
        read_line(&input, &mut buffer);
        let result = match oracle(&buffer, &conn)
            {Ok(s) => s,
             Err(e) => {eprintln!("{}", e);
                        continue;}
             };
        println!("{}", result);
    } 
}

fn read_line(input: &io::Stdin, buffer: &mut String) {
    match input.read_line(buffer)
        {Ok(_) => (),
         Err(e) => eprintln!("{}", e)};
}

