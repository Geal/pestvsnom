#![feature(test)]
extern crate pestvsnom;
extern crate test;

use test::Bencher;

use pestvsnom::ujson4c_parse;

const CANADA : &str = include_str!("../assets/canada.json");

#[bench]
fn ujson4c(b: &mut Bencher) {
  b.iter(||{
    ujson4c_parse(CANADA)
  });
}
