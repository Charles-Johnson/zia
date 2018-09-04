/*  Copyright (C) 2018  Charles Johnson

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod schema;
mod token;
mod tree;

pub use db::{memory_database, SqliteConnection, ZiaResult};
use token::Token;
use tree::extract_tree_from_token;

pub fn oracle(buffer: &str, conn: &SqliteConnection) -> ZiaResult<String> {
    let tree = try!(extract_tree_from_token(
        &Token::Expression(buffer.to_string()),
        conn
    ));
    let mut string = String::new();
    match try!(tree.call(conn)) {
        Some(s) => string = s,
        None => (),
    };
    Ok(string)
}

#[cfg(test)]
mod reductions {
    use {memory_database, oracle};
    #[test]
    fn monad() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(a ->) b", &conn).unwrap(), "");
        assert_eq!(oracle("a ->", &conn).unwrap(), "b");
        assert_eq!(oracle("((not true) ->) false", &conn).unwrap(), "");
        assert_eq!(oracle("(not true) ->", &conn).unwrap(), "false");
    }
    #[test]
    fn nested_monads() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("((not true) ->) false", &conn).unwrap(), "");
        assert_eq!(oracle("((not false) ->) true", &conn).unwrap(), "");
        assert_eq!(oracle("(not(not true))->", &conn).unwrap(), "true");
    }
    #[test]
    fn chain() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(a ->) b", &conn).unwrap(), "");
        assert_eq!(oracle("(b ->) c", &conn).unwrap(), "");
        assert_eq!(oracle("a ->", &conn).unwrap(), "c")
    }
}
#[cfg(test)]
mod definitions {
    use memory_database;
    use oracle;
    #[test]
    fn monad() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(* :=) (repeated +)", &conn).unwrap(), "");
        assert_eq!(oracle("* :=", &conn).unwrap(), "repeated +");
    }
    #[test]
    fn nested_monads() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(2 :=) (++ (++ 0))", &conn).unwrap(), "");
        assert_eq!(oracle("2 :=", &conn).unwrap(), "++ (++ 0)");
    }
}
