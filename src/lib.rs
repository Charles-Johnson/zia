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

mod ast;
mod concept;
mod constants;
mod context;
mod token;
mod traits;
mod utils;

pub use context::Context;
use utils::ZiaResult;

pub fn oracle(buffer: &str, cont: &mut Context) -> ZiaResult<String> {
    let ast = try!(cont.ast_from_expression(buffer));
    cont.call(&ast)
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
        assert_eq!(oracle("a ->", &mut cont).unwrap(), "c");
    }
    #[test]
    fn circular_loop() {
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
    #[test]
    fn remove_reduction() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("((b c) ->) a", &mut cont).unwrap(), "");
        assert_eq!(oracle("((b c) ->) (b c)", &mut cont).unwrap(), "");
        assert_eq!(oracle("(b c) ->", &mut cont).unwrap(), "b c");
    }
    #[test]
    fn infinite_loop() {
        let mut cont = Context::new().unwrap();
        assert_matches!(oracle("(b ->) (a b)", &mut cont), Err(ZiaError::Loop(_)),);
    }
    #[test]
    fn broken_end_chain() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(a ->) b", &mut cont).unwrap(), "");
        assert_eq!(oracle("(b ->) c", &mut cont).unwrap(), "");
        assert_eq!(oracle("(b ->) b", &mut cont).unwrap(), "");
        assert_eq!(oracle("a ->", &mut cont).unwrap(), "b");
    }
    #[test]
    fn broken_middle_chain() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(a ->) b", &mut cont).unwrap(), "");
        assert_eq!(oracle("(b ->) c", &mut cont).unwrap(), "");
        assert_eq!(oracle("(c ->) d", &mut cont).unwrap(), "");
        assert_eq!(oracle("(b ->) b", &mut cont).unwrap(), "");
        assert_eq!(oracle("a ->", &mut cont).unwrap(), "b");
    }
    #[test]
    fn change_reduction_rule() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(a ->) b", &mut cont).unwrap(), "");
        assert_eq!(oracle("(a ->) c", &mut cont).unwrap(), "");
        assert_eq!(oracle("a ->", &mut cont).unwrap(), "c");
    }
}
#[cfg(test)]
mod definitions {
    use oracle;
    use utils::ZiaError;
    use Context;
    #[test]
    fn fresh_monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(* :=) (repeated +)", &mut cont).unwrap(), "");
        assert_eq!(oracle("* :=", &mut cont).unwrap(), "repeated +");
    }
    #[test]
    fn fresh_nested_monads() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(2 :=) (++ (++ 0))", &mut cont).unwrap(), "");
        assert_eq!(oracle("2 :=", &mut cont).unwrap(), "++ (++ 0)");
    }
    #[test]
    fn left_fresh_monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(
            oracle("(((2 (repeated +)) 2) ->) 4", &mut cont).unwrap(),
            "",
        );
        assert_eq!(oracle("(* :=) (repeated +)", &mut cont).unwrap(), "");
        assert_eq!(oracle("* :=", &mut cont).unwrap(), "repeated +");
    }
    #[test]
    fn right_fresh_monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(((2 *) 2) ->) 4", &mut cont).unwrap(), "");
        assert_eq!(oracle("(* :=) (repeated +)", &mut cont).unwrap(), "");
        assert_eq!(oracle("* :=", &mut cont).unwrap(), "repeated +");
    }
    #[test]
    fn old_monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(((2 *) 2) ->) 4", &mut cont).unwrap(), "");
        assert_eq!(
            oracle("(((2 (repeated +)) 2) ->) 4", &mut cont).unwrap(),
            "",
        );
        assert_eq!(oracle("(* :=) (repeated +)", &mut cont).unwrap(), "");
        assert_eq!(oracle("* :=", &mut cont).unwrap(), "repeated +");
    }
    #[test]
    fn monad_on_the_left() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("((x y) ->) c", &mut cont).unwrap(), "");
        assert_matches!(oracle("((a b) :=) c", &mut cont), Err(ZiaError::Syntax(_)),);
    }
    #[test]
    fn fresh_refactor() {
        let mut cont = Context::new().unwrap();
        assert_matches!(oracle("(a :=) b", &mut cont), Err(ZiaError::Redundancy(_)),);
    }
    #[test]
    fn definition_loop() {
        let mut cont = Context::new().unwrap();
        assert_matches!(oracle("(a :=) (a b)", &mut cont), Err(ZiaError::Loop(_)),);
    }
    #[test]
    fn remove_definition() {
        let mut cont = Context::new().unwrap();
        assert_eq!(oracle("(a :=) (b c)", &mut cont).unwrap(), "");
        assert_eq!(oracle("(a :=) a", &mut cont).unwrap(), "");
        assert_eq!(oracle("a :=", &mut cont).unwrap(), "a");
    }
}
