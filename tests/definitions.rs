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

use zia::{Context, Display, ZiaError};
#[test]
fn fresh_symbol() {
    let mut cont = Context::new();
    assert_eq!(cont.execute("a :="), "a");
}
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
fn refactor() {
    let mut cont = Context::new();
    assert_eq!(cont.execute("a (:= (b c))"), "");
    assert_eq!(cont.execute("d (:= b)"), "");
    assert_eq!(cont.execute("a :="), "d c");
}
#[test]
fn bad_refactor() {
    let mut cont = Context::new();
    assert_eq!(cont.execute("a (:= (b c))"), "");
    assert_eq!(
        cont.execute("b (:= a)"),
        ZiaError::DefinitionCollision.to_string()
    );
}
#[test]
fn defining_used_symbol_as_used_pair() {
    let mut cont = Context::new();
    assert_eq!(cont.execute("a (:= (b c))"), "");
    assert_eq!(cont.execute("f (:= (d e))"), "");
    assert_eq!(
        cont.execute("d (:= (b c))"),
        ZiaError::DefinitionCollision.to_string()
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
fn nested_definition_loop() {
    let mut cont = Context::new();
    assert_eq!(
        cont.execute("a (:= ((a b) b))"),
        ZiaError::InfiniteDefinition.to_string()
    );
}
#[test]
fn chained_definitions_loop() {
    let mut cont = Context::new();
    assert_eq!(cont.execute("c (:= (a b))"), "");
    assert_eq!(
        cont.execute("a (:= (c b))"),
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
