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
mod concepts;
mod constants;
mod context;
mod token;
mod traits;
mod utils;

pub use context::Context;

#[cfg(test)]
mod reductions {
    use utils::ZiaError;
    use Context;
    #[test]
    fn monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a ->) b").unwrap(), "");
        assert_eq!(cont.execute("a ->").unwrap(), "b");
        assert_eq!(cont.execute("((not true) ->) false").unwrap(), "");
        assert_eq!(cont.execute("(not true) ->").unwrap(), "false");
    }
    #[test]
    fn nested_monads() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("((not true) ->) false").unwrap(), "");
        assert_eq!(cont.execute("((not false) ->) true").unwrap(), "");
        assert_eq!(cont.execute("(not(not true))->").unwrap(), "true");
    }
    #[test]
    fn chain() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a ->) b").unwrap(), "");
        assert_eq!(cont.execute("(b ->) c").unwrap(), "");
        assert_eq!(cont.execute("a ->").unwrap(), "c");
    }
    #[test]
    fn circular_loop() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a ->) b").unwrap(), "");
        assert_matches!(cont.execute("(b ->) a"), Err(ZiaError::Loop(_)));
        assert_eq!(cont.execute("b ->").unwrap(), "b");
    }
    #[test]
    fn trivial_parentheses() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a) ->").unwrap(), "a");
    }
    #[test]
    fn remove_reduction() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("((b c) ->) a").unwrap(), "");
        assert_eq!(cont.execute("((b c) ->) (b c)").unwrap(), "");
        assert_eq!(cont.execute("(b c) ->").unwrap(), "b c");
    }
    #[test]
    fn infinite_loop() {
        let mut cont = Context::new().unwrap();
        assert_matches!(cont.execute("(b ->) (a b)"), Err(ZiaError::Loop(_)));
    }
    #[test]
    fn broken_end_chain() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a ->) b").unwrap(), "");
        assert_eq!(cont.execute("(b ->) c").unwrap(), "");
        assert_eq!(cont.execute("(b ->) b").unwrap(), "");
        assert_eq!(cont.execute("a ->").unwrap(), "b");
    }
    #[test]
    fn broken_middle_chain() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a ->) b").unwrap(), "");
        assert_eq!(cont.execute("(b ->) c").unwrap(), "");
        assert_eq!(cont.execute("(c ->) d").unwrap(), "");
        assert_eq!(cont.execute("(b ->) b").unwrap(), "");
        assert_eq!(cont.execute("a ->").unwrap(), "b");
    }
    #[test]
    fn change_reduction_rule() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a ->) b").unwrap(), "");
        assert_eq!(cont.execute("(a ->) c").unwrap(), "");
        assert_eq!(cont.execute("a ->").unwrap(), "c");
    }
    #[test]
    fn redundancy() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a ->) b").unwrap(), "");
        assert_matches!(cont.execute("(a ->) b"), Err(ZiaError::Redundancy(_)));
    }
}
#[cfg(test)]
mod definitions {
    use utils::ZiaError;
    use Context;
    #[test]
    fn fresh_monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(* :=) (repeated +)").unwrap(), "");
        assert_eq!(cont.execute("* :=").unwrap(), "repeated +");
    }
    #[test]
    fn fresh_nested_monads() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(2 :=) (++ (++ 0))").unwrap(), "");
        assert_eq!(cont.execute("2 :=").unwrap(), "++ (++ 0)");
    }
    #[test]
    fn left_fresh_monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(((2 (repeated +)) 2) ->) 4").unwrap(), "",);
        assert_eq!(cont.execute("(* :=) (repeated +)").unwrap(), "");
        assert_eq!(cont.execute("* :=").unwrap(), "repeated +");
    }
    #[test]
    fn right_fresh_monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(((2 *) 2) ->) 4").unwrap(), "");
        assert_eq!(cont.execute("(* :=) (repeated +)").unwrap(), "");
        assert_eq!(cont.execute("* :=").unwrap(), "repeated +");
    }
    #[test]
    fn old_monad() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(((2 *) 2) ->) 4").unwrap(), "");
        assert_eq!(cont.execute("(((2 (repeated +)) 2) ->) 4").unwrap(), "",);
        assert_eq!(cont.execute("(* :=) (repeated +)").unwrap(), "");
        assert_eq!(cont.execute("* :=").unwrap(), "repeated +");
    }
    #[test]
    fn monad_on_the_left() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("((x y) ->) c").unwrap(), "");
        assert_matches!(cont.execute("((a b) :=) c"), Err(ZiaError::Syntax(_)));
    }
    #[test]
    fn fresh_refactor() {
        let mut cont = Context::new().unwrap();
        assert_matches!(cont.execute("(a :=) b"), Err(ZiaError::Redundancy(_)));
    }
    #[test]
    fn definition_loop() {
        let mut cont = Context::new().unwrap();
        assert_matches!(cont.execute("(a :=) (a b)"), Err(ZiaError::Loop(_)));
    }
    #[test]
    fn remove_definition() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a :=) (b c)").unwrap(), "");
        assert_eq!(cont.execute("(a :=) a").unwrap(), "");
        assert_eq!(cont.execute("a :=").unwrap(), "a");
    }
    #[test]
    fn redundancy() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a :=) (b c)").unwrap(), "");
        assert_matches!(cont.execute("(a :=) (b c)"), Err(ZiaError::Redundancy(_)));
    }
    #[test]
    fn definition_reduction() {
        let mut cont = Context::new().unwrap();
        assert_eq!(cont.execute("(a :=) (b c)").unwrap(), "");
        assert_eq!(cont.execute("(b ->) d").unwrap(), "");
        assert_eq!(cont.execute("(c ->) e").unwrap(), "");
        assert_eq!(cont.execute("a ->").unwrap(), "d e");
        assert_eq!(cont.execute("(f :=) (d e)").unwrap(), "");
        assert_eq!(cont.execute("a ->").unwrap(), "f");
    }
}
