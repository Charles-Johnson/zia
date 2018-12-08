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
pub use utils::ZiaError;

#[cfg(test)]
mod definitions_and_reductions {
    use Context;
	use utils::ZiaError;
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
	#[test]
    fn sneeky_infinite_reduction_chain() {
        let mut cont = Context::new();
		assert_eq!(cont.execute("c (-> a)"), "");
        assert_eq!(
            cont.execute("a (:= (c b))"),
            ZiaError::ExpandingReduction.to_string()
        );
    }
}
#[cfg(test)]
mod other {
    use utils::ZiaError;
    use Context;
    #[test]
    fn fresh_symbol_is_not_a_program() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a"), ZiaError::NotAProgram.to_string());
	}
	#[test]
    fn fresh_pair_is_not_a_program() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a a"), ZiaError::NotAProgram.to_string());
	}
	#[test]
    fn fresh_nested_pair_is_not_a_program() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("a (a a)"), ZiaError::NotAProgram.to_string());
	}
	#[test]
	fn used_symbol_is_not_a_program() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(cont.execute("a"), ZiaError::NotAProgram.to_string());
	}
	#[test]
	fn used_symbol_in_a_pair_is_not_a_program() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(cont.execute("a a"), ZiaError::NotAProgram.to_string());
	}
	#[test]
	fn used_symbol_in_a_nested_pair_is_not_a_program() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("a (-> b)"), "");
        assert_eq!(cont.execute("a (a a)"), ZiaError::NotAProgram.to_string());
	}
	#[test]
	fn symbol_whose_normal_form_is_a_program_is_a_program() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("a (-> (b :=))"), "");
        assert_eq!(cont.execute("a"), "b");
	}
	#[test]
	fn symbol_whose_definition_is_a_program_is_a_program() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("a (:= (b :=))"), "");
        assert_eq!(cont.execute("a"), "b");
	}
	#[test]
	fn symbol_whose_normal_form_is_a_builtin_concept() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("a (-> :=)"), "");
        assert_eq!(cont.execute("b a"), "b");
	}
	#[test]
	fn lazy_normal_form_evaluation() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("-> (-> a)"), "");
		assert_eq!(cont.execute("b (-> ->)"), "");
		assert_eq!(cont.execute("c b"), "c");
	}
	#[test]
	fn lazy_reduction_evaluation() {
		let mut cont = Context::new();
		assert_eq!(cont.execute("-> (-> a)"), "");
		assert_eq!(cont.execute("b (-> ->)"), "");
		assert_eq!(cont.execute("c (b d)"), "");
		assert_eq!(cont.execute("c b"), "d");
	}
	#[test]
	fn builtin_concept_definition() {
		let mut cont = Context::new();
		assert_eq!(cont.execute(":= (:= (a b))"), "");
		assert_eq!(cont.execute("c (:= (d :=))"), "");
		assert_eq!(cont.execute("c"), "d");
	}
    #[test]
    fn empty_parentheses() {
        let mut cont = Context::new();
        assert_eq!(cont.execute("()"), ZiaError::EmptyParentheses.to_string());
    }
    #[test]
    fn ambiguous_expression() {
        let mut cont = Context::new();
        assert_eq!(
            cont.execute("(a b c)"),
            ZiaError::AmbiguousExpression.to_string()
        );
    }
}
