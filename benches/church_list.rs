#![cfg(feature = "church")]

#![feature(test)]
extern crate test;

#[macro_use]
extern crate lambda_calculus as lambda;

use test::Bencher;
use lambda::church::list::*;
use lambda::church::numerals::*;
use lambda::church::boolean::fls;
use lambda::combinators::C;
use lambda::*;

fn list1() -> Term { Term::from(vec![1.into()]) }
fn list2() -> Term { Term::from(vec![2.into(), 3.into()]) }
fn list3() -> Term { Term::from(vec![1.into(), 2.into(), 3.into()]) }
fn   gt1() -> Term { app!(C(), gt(), 1.into()) }

#[bench]
fn church_null(b: &mut Bencher) {
    b.iter(|| { beta(app(null(), list3()), HAP, 0) } );
}

#[bench]
fn church_head(b: &mut Bencher) {
    b.iter(|| { beta(app(head(), list3()), HAP, 0) } );
}

#[bench]
fn church_tail(b: &mut Bencher) {
    b.iter(|| { beta(app(tail(), list3()), HAP, 0) } );
}

#[bench]
fn church_length(b: &mut Bencher) {
    b.iter(|| { beta(app(length(), list3()), HAP, 0) } );
}

#[bench]
fn church_list(b: &mut Bencher) {
    b.iter(|| { beta(app!(list(), 3.into(), 1.into(), 2.into(), 3.into()), HAP, 0) } );
}

#[bench]
fn church_reverse(b: &mut Bencher) {
    b.iter(|| { beta(app(reverse(), list3()), HAP, 0) } );
}

#[bench]
fn church_append(b: &mut Bencher) {
    b.iter(|| { beta(app!(append(), list1(), list2()), HAP, 0) } );
}

#[bench]
fn church_index(b: &mut Bencher) {
    b.iter(|| { beta(app!(index(), 1.into(), list3()), HAP, 0) } );
}

#[bench]
fn church_map(b: &mut Bencher) {
    b.iter(|| { beta(app!(map(), gt1(), list3()), HAP, 0) } );
}

#[bench]
fn church_foldl(b: &mut Bencher) {
    b.iter(|| { beta(app!(foldl(), plus(), 0.into(), list3()), HAP, 0) } );
}

#[bench]
fn church_foldr(b: &mut Bencher) {
    b.iter(|| { beta(app!(foldr(), plus(), 0.into(), list3()), HAP, 0) } );
}

#[bench]
fn church_filter(b: &mut Bencher) {
    b.iter(|| { beta(app!(filter(), gt1(), list3()), HAP, 0) } );
}

#[bench]
fn church_init(b: &mut Bencher) {
    b.iter(|| { beta(app(init(), list3()), HAP, 0) } );
}

#[bench]
fn church_last(b: &mut Bencher) {
    b.iter(|| { beta(app(last(), list3()), HAP, 0) } );
}

#[bench]
fn church_zip(b: &mut Bencher) {
    b.iter(|| { beta(app!(zip(), list3(), list3()), HAP, 0) } );
}

#[bench]
fn church_zip_with(b: &mut Bencher) {
    b.iter(|| { beta(app!(zip_with(), fls(), list3(), list3()), HAP, 0) } );
}

#[bench]
fn church_take(b: &mut Bencher) {
    b.iter(|| { beta(app!(take(), 2.into(), list3()), HAP, 0) } );
}

#[bench]
fn church_take_while(b: &mut Bencher) {
    b.iter(|| { beta(app!(take_while(), is_zero(), list3()), HAP, 0) } );
}