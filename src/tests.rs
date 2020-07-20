#![cfg(test)]

use crate::*;

#[test]
fn new() {
    AreYouSure::new(1);
}

#[test]
fn value() {
    assert_eq!(
        AreYouSure::new(2).yes_i_am_sure(),
        2
    );
}

#[test]
fn value_macro() {
    let x = make_sure!(3);
    assert_eq!(
        x.yes_i_am_sure(),
        3
    );
}

#[test]
fn deferred_macro() {
    use std::cmp::min;
    let x = make_sure!(min(4, 5));
    assert_eq!(
        x.yes_i_am_sure(),
        4
    );
}

#[test]
fn deferred_namespaced() {
    let x = make_sure!(std::cmp::min(4, 5));
    assert_eq!(
        x.yes_i_am_sure(),
        4
    );
}

#[test]
fn deferred_method() {
    let mut v = Vec::new();
    let x = make_sure!(v.push(5));
    x.yes_i_am_sure();
    assert_eq!(
        v,
        vec![5]
    );
}

fn sideeffecting(val: usize) -> usize {
        println!("{}", val);
        val
}

#[test]
fn deferred_sideffects_macro() {
    let x = make_sure!(sideeffecting(6));
    assert_eq!(
        x.yes_i_am_sure(),
        6
    );
}

#[test]
fn block_deferred_macro() {
    let x = make_sure!({
        let y = 7;
        println!("{}", y);
        y
    });
    assert_eq!(
        x.yes_i_am_sure(),
        7
    );
}

#[test]
fn value_macro_chain() {
    assert_eq!(
        make_sure!(8).yes_i_am_sure(),
        8
    )
}
