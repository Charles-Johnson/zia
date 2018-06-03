use std::io;
extern crate zia;
use zia::{oracle, establish_connection, setup_database, SqliteConnection};
use std::env;
use std::path::Path;

fn memory_database()->SqliteConnection {
    let conn = establish_connection(":memory:");
    setup_database(&conn);
    conn}

fn file_database(name: &String)->SqliteConnection {
    let database_exists = Path::new(name).exists();
    let conn = establish_connection(&name);
    if database_exists {setup_database(&conn);}
    conn
}

fn main() {
    let new_database = true;
    let args: Vec<String> = env::args().collect();
    let argslen = args.len();
    let conn: SqliteConnection = 
              match argslen {1 => memory_database(),
                             2 => file_database(&args[1]),
                             _ => panic!("Only one argument allowed.")};
    let input = io::stdin();
    let mut buffer = String::new();
    input.read_line(&mut buffer).expect("Could not read line");
    while buffer != "" {
        println!("{:?}", oracle(&buffer, &conn));
        buffer = String::new();
        input.read_line(&mut buffer).expect("Could not read line");
    } 
}

