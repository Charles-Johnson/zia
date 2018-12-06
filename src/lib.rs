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
    fn symbol_to_symbol() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(cont.execute("a ->"), "b");
	}
	#[test]
	fn pair_to_symbol() {
		let mut cont = Context::new();
        assert_eq!(cont.execute("(not true) (-> false)"), "");
        assert_eq!(cont.execute("(not true) ->"), "false");
    }
    #[test]
    fn nested_pairs_to_symbol() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("(not true) (-> false)"), "");
        assert_eq!(cont.execute("(not false) (-> true)"), "");
        assert_eq!(cont.execute("(not(not true))->"), "true");
    }
    #[test]
    fn chain() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(cont.execute("b (-> c)"), "");
        assert_eq!(cont.execute("a ->"), "c");
    }
    #[test]
    fn cycle() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(
            cont.execute("b (-> a)"),
            ZiaError::CyclicReduction.to_string()
        );
        assert_eq!(cont.execute("b ->"), "b");
    }
    #[test]
    fn trivial_parentheses() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("(a) ->"), "a");
    }
    #[test]
    fn remove_reduction() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("(b c) (-> a)"), "");
        assert_eq!(cont.execute("(b c) (-> (b c))"), "");
        assert_eq!(cont.execute("(b c) ->"), "b c");
    }
    #[test]
    fn infinite_expansion() {
        let mut cont = Context::new();
        assert_eq!(
            cont.execute("b (-> (a b))"),
            ZiaError::ExpandingReduction.to_string()
        );
    }
    #[test]
    fn broken_end_chain() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(cont.execute("b (-> c)"), "");
        assert_eq!(cont.execute("b (-> b)"), "");
        assert_eq!(cont.execute("a ->"), "b");
    }
    #[test]
    fn broken_middle_chain() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(cont.execute("b (-> c)"), "");
        assert_eq!(cont.execute("c (-> d)"), "");
        assert_eq!(cont.execute("b (-> b)"), "");
        assert_eq!(cont.execute("a ->"), "b");
    }
    #[test]
    fn change_reduction_rule() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(cont.execute("a (-> c)"), "");
        assert_eq!(cont.execute("a ->"), "c");
    }
    #[test]
    fn redundancy() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(
            cont.execute("a (-> b)"),
            ZiaError::RedundantReduction.to_string()
        );
    }
}
#[cfg(test)]
mod definitions {
    use utils::ZiaError;
    use Context;
    #[test]
    fn fresh_pair() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (:= (b c))"), "");
        assert_eq!(cont.execute("a :="), "b c");
    }
    #[test]
    fn fresh_nested_pairs() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (:= (b (c d)))"), "");
        assert_eq!(cont.execute("a :="), "b (c d)");
    }
    #[test]
    fn defining_used_symbol_as_fresh_pair() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (:= (b c))"), "");
        assert_eq!(cont.execute("b (:= (d e))"), "");
        assert_eq!(cont.execute("b :="), "d e");
    }
    #[test]
    fn defining_fresh_symbol_as_used_pair() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (:= (b c))"), "");
        assert_eq!(cont.execute("d (:= (b c))"), "");
        assert_eq!(cont.execute("d :="), "b c");
    }
    #[test]
    fn old_pair() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (:= (b c))"), "");
        assert_eq!(cont.execute("d (:= (e f))"), "",);
        assert_eq!(cont.execute("b (:= (e c))"), "");
        assert_eq!(cont.execute("b :="), "e c");
    }
    #[test]
    fn pair_on_the_left() {
        let mut cont = Context::new();
        assert_eq!(
            cont.execute("(a b) (:= c)"),
            ZiaError::BadDefinition.to_string()
        );
    }
    #[test]
    fn fresh_refactor() {
        let mut cont = Context::new();
        assert_eq!(
            cont.execute("a (:= b)"),
            ZiaError::RedundantRefactor.to_string()
        );
    }
    #[test]
    fn definition_loop() {
        let mut cont = Context::new();
        assert_eq!(
            cont.execute("a (:= (a b))"),
            ZiaError::InfiniteDefinition.to_string()
        );
    }
    #[test]
    fn remove_definition() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (:= (b c))"), "");
        assert_eq!(cont.execute("a (:= a)"), "");
        assert_eq!(cont.execute("a :="), "a");
        assert_eq!(
            cont.execute("a (:= b)"),
            ZiaError::RedundantRefactor.to_string()
        );
    }
    #[test]
    fn redundancy() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (:= (b c))"), "");
        assert_eq!(
            cont.execute("a (:= (b c))"),
            ZiaError::RedundantDefinition.to_string()
        );
    }
}
#[cfg(test)]
mod definitions_and_reductions {
	use Context;
    #[test]
    fn indirect_reduction() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (:= (b c))"), "");
        assert_eq!(cont.execute("b (-> d)"), "");
        assert_eq!(cont.execute("c (-> e)"), "");
        assert_eq!(cont.execute("a ->"), "d e");
        assert_eq!(cont.execute("f (:= (d e))"), "");
        assert_eq!(cont.execute("a ->"), "f");
    }
}
#[cfg(test)]
mod other {
    use utils::ZiaError;
    use Context;
    #[test]
    fn not_a_program() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a"), ZiaError::NotAProgram.to_string());
        assert_eq!(cont.execute("a a"), ZiaError::NotAProgram.to_string());
        assert_eq!(cont.execute("a (a a)"), ZiaError::NotAProgram.to_string());
        assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(cont.execute("a a"), ZiaError::NotAProgram.to_string());
        assert_eq!(cont.execute("a (a a)"), ZiaError::NotAProgram.to_string());
    }
	#[test]
	fn empty_parentheses() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("()"), ZiaError::EmptyParentheses.to_string());
	}
	#[test]
	fn ambiguous_expression() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("(a b c)"), ZiaError::AmbiguousExpression.to_string());
	}
}
