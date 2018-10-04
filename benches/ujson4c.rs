#![feature(test)]
extern crate pestvsnom;
extern crate test;

use test::Bencher;

use pestvsnom::ujson4c_parse;

const CANADA : &str = include_str!("../assets/canada.json");
const DATA : &str = include_str!("../assets/data.json");

#[bench]
fn ujson4c_canada(b: &mut Bencher) {
  b.iter(||{
    ujson4c_parse(CANADA)
  });
}

#[bench]
fn ujson4c_data(b: &mut Bencher) {
  b.iter(||{
    ujson4c_parse(DATA)
  });
}
