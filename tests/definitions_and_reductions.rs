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

use zia::{AbstractSyntaxTree, Context, ContextMaker, Execute, ZiaError};

#[test]
fn indirect_reduction() {
    let mut cont = Context::new();
    assert_eq!(cont.execute::<AbstractSyntaxTree>("a (:= (b c))"), "");
    assert_eq!(cont.execute::<AbstractSyntaxTree>("b (-> d)"), "");
    assert_eq!(cont.execute::<AbstractSyntaxTree>("c (-> e)"), "");
    assert_eq!(cont.execute::<AbstractSyntaxTree>("a ->"), "d e");
    assert_eq!(cont.execute::<AbstractSyntaxTree>("f (:= (d e))"), "");
    assert_eq!(cont.execute::<AbstractSyntaxTree>("a ->"), "f");
}
#[test]
fn sneeky_infinite_reduction_chain() {
    let mut cont = Context::new();
    assert_eq!(cont.execute::<AbstractSyntaxTree>("c (-> a)"), "");
    assert_eq!(
        cont.execute::<AbstractSyntaxTree>("a (:= (c b))"),
        ZiaError::ExpandingReduction.to_string()
    );
}
