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
extern crate zia;

use zia::{Context, Display, Execute, ZiaError};

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
