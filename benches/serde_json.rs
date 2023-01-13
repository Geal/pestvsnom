#![feature(test)]

extern crate test;

use test::Bencher;

const CANADA : &str = include_str!("../assets/canada.json");
const DATA   : &str = include_str!("../assets/data.json");

#[bench]
fn serde_json_canada(b: &mut Bencher) {
    b.iter(|| {
        let output: serde_json::Value = serde_json::from_str(CANADA).unwrap();
    });
}

#[bench]
fn serde_json_data(b: &mut Bencher) {
    b.iter(|| {
        let output: serde_json::Value = serde_json::from_str(DATA).unwrap();
    });
}
