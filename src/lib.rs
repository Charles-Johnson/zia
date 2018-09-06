/*  Library for the Zia programming language.
    Copyright (C) 2018  Charles Johnson

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
#[macro_use]
extern crate matches;

mod db;
mod schema;
mod token;
mod tree;

pub use db::memory_database;
use db::{SqliteConnection, ZiaResult};
use tree::extract_tree_from_expression;

pub fn oracle(buffer: &str, conn: &SqliteConnection) -> ZiaResult<String> {
    let tree = try!(extract_tree_from_expression(buffer, conn));
    Ok(try!(tree.call(conn)).unwrap_or_default())
}

#[cfg(test)]
mod reductions {
    use db::{memory_database, DBError};
    use oracle;
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
    #[test]
    fn prevent_loop() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(a ->) b", &conn).unwrap(), "");
        assert_matches!(oracle("(b ->) a", &conn), Err(DBError::Loop(_)));
        assert_eq!(oracle("b ->", &conn).unwrap(), "b");
    }
    #[test]
    fn trivial_parentheses() {
        let conn = memory_database().unwrap();
        assert_eq!(oracle("(a) ->", &conn).unwrap(), "a");
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
