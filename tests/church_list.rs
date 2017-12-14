#![cfg(feature = "church")]

extern crate lambda_calculus as lambda;

use lambda::*;
use lambda::church::list::*;
use lambda::church::numerals::{plus, is_zero};
use lambda::church::boolean::fls;

#[test]
fn church_last() {
    let list1 = || { Term::from(vec![1.into()]) };
    let list2 = || { Term::from(vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into()]) };

    assert_eq!(beta(app(last(), nil()), HAP, 0), nil());
    assert_eq!(beta(app(last(), list1()), HAP, 0), 1.into());
    assert_eq!(beta(app(last(), list2()), HAP, 0), 4.into());
}

#[test]
fn church_init() {
    let list1 = || { Term::from(vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into()]) };
    let list2 = || { Term::from(vec![0.into(), 1.into(), 2.into(), 3.into()]) };
    let list3 = || { Term::from(vec![2.into(), 3.into()]) };
    let list4 = || { Term::from(vec![2.into()]) };

    assert_eq!(beta(app(init(), list1()), HAP, 0), list2());
    assert_eq!(beta(app(init(), list3()), HAP, 0), list4());
    assert_eq!(beta(app(init(), list4()), HAP, 0), nil());
    assert_eq!(beta(app(init(), nil()), HAP, 0), nil());
}

#[test]
fn church_zip() {
    let l1 = || { Term::from(vec![0.into()]) };
    let l2 = || { Term::from(vec![0.into(), 1.into(), 2.into()]) };
    let l3 = || { Term::from(vec![2.into(), 1.into()]) };

    let p1 = || { Term::from(vec![(0.into(), 0.into()).into()]) }; // zip(l1, l1)
    let p2 = || { Term::from(vec![
        (0.into(), 0.into()).into(),
        (1.into(), 1.into()).into(),
        (2.into(), 2.into()).into(),
    ])}; //zip(l2, l2)
    let p3 = || { Term::from(vec![
        (0.into(), 2.into()).into(),
        (1.into(), 1.into()).into(),
    ])}; // zip(l2, l3)

    assert_eq!(beta(app!(zip(), nil(), nil()), HAP, 0), nil());
    assert_eq!(beta(app!(zip(), nil(), l1()), HAP, 0), nil());
    assert_eq!(beta(app!(zip(), l1(), nil()), HAP, 0), nil());
    assert_eq!(beta(app!(zip(), l1(), l1()), HAP, 0), p1());
    assert_eq!(beta(app!(zip(), l1(), l2()), HAP, 0), p1());
    assert_eq!(beta(app!(zip(), l2(), l1()), HAP, 0), p1());
    assert_eq!(beta(app!(zip(), l2(), l2()), HAP, 0), p2());
    assert_eq!(beta(app!(zip(), l2(), l3()), HAP, 0), p3());
}

#[test]
fn church_zip_with() {
    let l1 = || { Term::from(vec![1.into()]) };
    let l2 = || { Term::from(vec![2.into()]) };
    let l3 = || { Term::from(vec![1.into(), 2.into(), 3.into()]) };
    let l4 = || { Term::from(vec![2.into(), 4.into(), 6.into()]) };
    let l5 = || { Term::from(vec![3.into()]) };

    assert_eq!(beta(app!(zip_with(), plus(), nil(), nil()), HAP, 0), nil());
    assert_eq!(beta(app!(zip_with(), plus(), l1(), nil()), HAP, 0), nil());
    assert_eq!(beta(app!(zip_with(), plus(), nil(), l1()), HAP, 0), nil());
    assert_eq!(beta(app!(zip_with(), abs!(2, Var(1)), l1(), l1()), HAP, 0), l1());
    assert_eq!(beta(app!(zip_with(), plus(), l1(), l1()), HAP, 0), l2());
    assert_eq!(beta(app!(zip_with(), plus(), l3(), l3()), HAP, 0), l4());
    assert_eq!(beta(app!(zip_with(), plus(), l4(), l1()), HAP, 0), l5());
    assert_eq!(beta(app!(zip_with(), plus(), l1(), l4()), HAP, 0), l5());
    assert_eq!(beta(app!(zip_with(), fls(), l1(), l4()), HAP, 0), l2());
    assert_eq!(beta(app!(zip_with(), fls(), l4(), l1()), HAP, 0), l1());
}

#[test]
fn church_take() {
    let l1 = || { Term::from(vec![0.into()]) };
    let l2 = || { Term::from(vec![0.into(), 1.into()]) };
    let l3 = || { Term::from(vec![0.into(), 1.into(), 2.into()]) };
    let l4 = || { Term::from(vec![0.into(), 1.into(), 2.into(), 3.into()]) };

    assert_eq!(beta(app!(take(), 5.into(), l4()), HAP, 0), l4());
    assert_eq!(beta(app!(take(), 4.into(), l4()), HAP, 0), l4());
    assert_eq!(beta(app!(take(), 3.into(), l4()), HAP, 0), l3());
    assert_eq!(beta(app!(take(), 2.into(), l4()), HAP, 0), l2());
    assert_eq!(beta(app!(take(), 1.into(), l4()), HAP, 0), l1());
    assert_eq!(beta(app!(take(), 0.into(), l4()), HAP, 0), nil());
    assert_eq!(beta(app!(take(), 1.into(), l1()), HAP, 0), l1());
    assert_eq!(beta(app!(take(), 0.into(), l1()), HAP, 0), nil());
    assert_eq!(beta(app!(take(), 1.into(), nil()), HAP, 0), nil());
}

#[test]
fn church_take_while() {
    let l1 = || { Term::from(vec![0.into(), 0.into(), 2.into(), 3.into()]) };
    let l2 = || { Term::from(vec![0.into(), 0.into()]) };
    let l3 = || { Term::from(vec![1.into(), 4.into(), 2.into(), 3.into()]) };
    let l4 = || { Term::from(vec![0.into(), 4.into(), 0.into(), 0.into()]) };
    let l5 = || { Term::from(vec![0.into()]) };

    assert_eq!(beta(app!(take_while(), is_zero(), nil()), HAP, 0), nil());
    assert_eq!(beta(app!(take_while(), is_zero(), l1()), HAP, 0), l2());
    assert_eq!(beta(app!(take_while(), is_zero(), l2()), HAP, 0), l2());
    assert_eq!(beta(app!(take_while(), is_zero(), l3()), HAP, 0), nil());
    assert_eq!(beta(app!(take_while(), is_zero(), l4()), HAP, 0), l5());
}