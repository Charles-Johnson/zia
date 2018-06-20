extern crate zia2sql;

pub use zia2sql::{memory_database, SqliteConnection};

mod token;

mod tree;

use tree::extract_tree_from_token;

use token::Token;

pub fn oracle(buffer: &str, conn: &SqliteConnection)-> String{
    let mut application_tree = extract_tree_from_token(&Token::Expression(buffer.to_string()), conn);
    application_tree.call_reduction_rule(conn);
    application_tree.call_definition(conn);
    let mut string = String::new();
    match application_tree.call_normal_form(conn) {None => (),
                                                   Some(s) => string = s};
    match application_tree.call_expansion(conn) {None => (),
                                                 Some(s) => string = s};    
    string
}

#[cfg(test)]
mod reductions {
    use {oracle, memory_database};
    #[test]
    fn monad() {
        let conn = memory_database();
        assert_eq!(oracle("(-> b)a", &conn),"");
        assert_eq!(oracle("a ->", &conn),"b");
        assert_eq!(oracle("(-> false)(not true)", &conn), "");
        assert_eq!(oracle("(not true)->", &conn),"false");
    }
    #[test]
    fn nested_monads() {
        let conn = memory_database();
        assert_eq!(oracle("(-> false)(not true)", &conn), "");
        assert_eq!(oracle("(-> true)(not false)", &conn), "");
        assert_eq!(oracle("(not(not true))->", &conn), "true");
    }
    #[test]
    fn chain() {
        let conn = memory_database();
        assert_eq!(oracle("(-> b) a", &conn),"");
        assert_eq!(oracle("(-> c) b", &conn), "");
        assert_eq!(oracle("a ->", &conn), "c")
    }
    #[test]
    fn diad() {
        let conn = memory_database();
        assert_eq!(oracle("(0 + 1)->", &conn), "1");
    }
    #[test]
    fn lambda() {
        let conn = memory_database();
        assert_eq!(oracle("((lambda x_)(_f _x))_y ->", &conn),"_f _y");
    }
    #[test]
    fn wrong_variable() {
        let conn = memory_database();
        assert_eq!(oracle("_x -> _y", &conn), "Error! Variable _y does not appear in the expression '_x'.");
    }
    #[test]
    fn labelling_a_variable() {
        let conn = memory_database();
        assert_eq!(oracle("a := _x", &conn),"Error! Cannot label variable expression '_x'.");
        assert_eq!(oracle("a := x_", &conn),"Error! Cannot label dummy expression 'x_'.");
    }
    #[test]
    fn variable_label() {
        let conn = memory_database();
        assert_eq!(oracle("_x := a", &conn), "Error! Cannot use '_x' as a label.");
        assert_eq!(oracle("x_ := a", &conn), "Error! Cannot use 'x_' as a label.");
    }
    #[test]
    fn variable_reduction() {
        let conn = memory_database();
        assert_eq!(oracle("_x and false ->", &conn), "false");
    }
}

mod definitions {
    use oracle;
    use memory_database;
    #[test]
    fn monad() {
        let conn = memory_database();
        assert_eq!(oracle("(:= (repeated +))*", &conn), "");
        assert_eq!(oracle("* :=", &conn), "repeated +");
    }
}

