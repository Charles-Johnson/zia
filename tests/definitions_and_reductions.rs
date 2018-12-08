extern crate zia;

use zia::{Context, ZiaError};

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
