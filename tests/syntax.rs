extern crate zia;

use zia::{Context, ZiaError};

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
