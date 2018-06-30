extern crate learning_rust;

use learning_rust::balanced;
use learning_rust::word_count;

#[test]
fn balanced_works() {
    assert_eq!(balanced("()"), true);
    assert_eq!(balanced("(a)"), true);
    assert_eq!(balanced("{[]cd"), false);
}

#[test]
fn word_count_works() {
    let map = word_count("hello world this is is hello world");
    assert_eq!(Some(&2), map.get("hello"));
}
