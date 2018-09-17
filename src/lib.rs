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
#![feature(vec_remove_item)]
#[macro_use]
extern crate matches;

mod concept;
mod constants;
mod context;
mod token;
mod utils;

pub use context::Context;
use utils::ZiaResult;

pub fn oracle(buffer: &str, cont: &mut Context) -> ZiaResult<String> {
    let concept = try!(cont.concept_from_expression(buffer));
    cont.call(&concept)
}

#[cfg(test)]
mod reductions {
    use oracle;
    use utils::ZiaError;
    use Context;
    #[test]
    fn monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(a ->) b", &mut cont).unwrap(), "");
        assert_eq!(oracle("a ->", &mut cont).unwrap(), "b");
        assert_eq!(oracle("((not true) ->) false", &mut cont).unwrap(), "");
        assert_eq!(oracle("(not true) ->", &mut cont).unwrap(), "false");
    }
    #[test]
    fn nested_monads() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("((not true) ->) false", &mut cont).unwrap(), "");
        assert_eq!(oracle("((not false) ->) true", &mut cont).unwrap(), "");
        assert_eq!(oracle("(not(not true))->", &mut cont).unwrap(), "true");
    }
    #[test]
    fn chain() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(a ->) b", &mut cont).unwrap(), "");
        assert_eq!(oracle("(b ->) c", &mut cont).unwrap(), "");
        assert_eq!(oracle("a ->", &mut cont).unwrap(), "c")
    }
    #[test]
    fn prevent_loop() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(a ->) b", &mut cont).unwrap(), "");
        assert_matches!(oracle("(b ->) a", &mut cont), Err(ZiaError::Loop(_)));
        assert_eq!(oracle("b ->", &mut cont).unwrap(), "b");
    }
    #[test]
    fn trivial_parentheses() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(a) ->", &mut cont).unwrap(), "a");
    }
}
#[cfg(test)]
mod definitions {
    use oracle;
    use Context;
    #[test]
    fn monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(* :=) (repeated +)", &mut cont).unwrap(), "");
        assert_eq!(oracle("* :=", &mut cont).unwrap(), "repeated +");
    }
    #[test]
    fn nested_monads() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(2 :=) (++ (++ 0))", &mut cont).unwrap(), "");
        assert_eq!(oracle("2 :=", &mut cont).unwrap(), "++ (++ 0)");
    }
}
