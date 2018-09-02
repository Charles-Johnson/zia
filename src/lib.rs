#[macro_use]
extern crate diesel;
extern crate dotenv;

mod token;
mod tree;
mod db;
mod schema;

pub use db::{memory_database, SqliteConnection, ZiaResult};
use tree::extract_tree_from_token;
use token::Token;

pub fn oracle(buffer: &str, conn: &SqliteConnection)-> ZiaResult<String> {
    let tree = try!(extract_tree_from_token(&Token::Expression(buffer.to_string()), conn));
    let mut string = String::new();
    match try!(tree.call(conn))
        {Some(s) => string = s,
         None => ()
         };
    Ok(string)
}

#[cfg(test)]
mod reductions {
    use {oracle, memory_database};
    #[test]
    fn monad() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(-> b)a", &conn).unwrap(),"");
        assert_eq!(oracle("a ->", &conn).unwrap(),"b");
        assert_eq!(oracle("(-> false)(not true)", &conn).unwrap(), "");
        assert_eq!(oracle("(not true)->", &conn).unwrap(),"false");
    }
    #[test]
    fn nested_monads() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(-> false)(not true)", &conn).unwrap(), "");
        assert_eq!(oracle("(-> true)(not false)", &conn).unwrap(), "");
        assert_eq!(oracle("(not(not true))->", &conn).unwrap(), "true");
    }
    #[test]
    fn chain() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(-> b) a", &conn).unwrap(),"");
        assert_eq!(oracle("(-> c) b", &conn).unwrap(), "");
        assert_eq!(oracle("a ->", &conn).unwrap(), "c")
    }
}
#[cfg(test)]
mod definitions {
    use oracle;
    use memory_database;
    #[test]
    fn monad() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(:= (repeated +))*", &conn).unwrap(), "");
        assert_eq!(oracle("* :=", &conn).unwrap(), "repeated +");
    }
    #[test]
    fn nested_monads() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(:= (++ (++ 0)))2", &conn).unwrap(), "");
        assert_eq!(oracle("2 :=", &conn).unwrap(), "++ (++ 0)");
    }
}

