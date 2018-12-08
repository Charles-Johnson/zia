extern crate zia;

use zia::{Context, ZiaError};

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
