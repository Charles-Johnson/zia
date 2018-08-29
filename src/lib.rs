extern crate zia2sql;

pub use zia2sql::{memory_database, SqliteConnection, ZiaResult};

mod token;
mod precedence;
mod tree;

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
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("(-> b)a", &conn).unwrap(),"");
        assert_eq!(oracle("a ->", &conn).unwrap(),"b");
        assert_eq!(oracle("(-> false)(not true)", &conn).unwrap(), "");
        assert_eq!(oracle("(not true)->", &conn).unwrap(),"false");
    }
    #[test]
    fn nested_monads() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("(-> false)(not true)", &conn).unwrap(), "");
        assert_eq!(oracle("(-> true)(not false)", &conn).unwrap(), "");
        assert_eq!(oracle("(not(not true))->", &conn).unwrap(), "true");
    }
    #[test]
    fn chain() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("(-> b) a", &conn).unwrap(),"");
        assert_eq!(oracle("(-> c) b", &conn).unwrap(), "");
        assert_eq!(oracle("a ->", &conn).unwrap(), "c")
    }
    #[test]
    fn diad() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("(0 + 1)->", &conn).unwrap(), "1");
    }
    #[test]
    fn lambda() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("((lambda x_)(_f _x))_y ->", &conn).unwrap(),"_f _y");
    }
    #[test]
    fn wrong_variable() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("_x -> _y", &conn).unwrap(), "Error! Variable _y does not appear in the expression '_x'.");
    }
    #[test]
    fn labelling_a_variable() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("a := _x", &conn).unwrap(),"Error! Cannot label variable expression '_x'.");
        assert_eq!(oracle("a := x_", &conn).unwrap(),"Error! Cannot label dummy expression 'x_'.");
    }
    #[test]
    fn variable_label() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("_x := a", &conn).unwrap(), "Error! Cannot use '_x' as a label.");
        assert_eq!(oracle("x_ := a", &conn).unwrap(), "Error! Cannot use 'x_' as a label.");
    }
    #[test]
    fn variable_reduction() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("_x and false ->", &conn).unwrap(), "false");
    }
    #[test]
    fn set_precedence() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("(>- b) a", &conn).unwrap(), "");
        assert_eq!(oracle("((>- b) a) ->", &conn).unwrap(), "true");
    }
}

mod definitions {
    use oracle;
    use memory_database;
    #[test]
    fn monad() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("(:= (repeated +))*", &conn).unwrap(), "");
        assert_eq!(oracle("* :=", &conn).unwrap(), "repeated +");
    }
    #[test]
    fn nested_monads() {
        let (conn, _file_conn) = memory_database().unwrap();
        assert_eq!(oracle("(:= (++ (++ 0)))2", &conn).unwrap(), "");
        assert_eq!(oracle("2 :=", &conn).unwrap(), "++ (++ 0)");
    }
}

