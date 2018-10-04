#![feature(test)]
extern crate test;

extern crate pest;
extern crate pest_grammars;

use test::Bencher;

use pest::Parser;
use pest_grammars::json::*;

const CANADA : &str = include_str!("../assets/canada.json");
const DATA : &str = include_str!("../assets/data.json");


#[bench]
fn pest_canada(b: &mut Bencher) {
  b.iter(|| JsonParser::parse(Rule::json, CANADA).unwrap());
}

#[bench]
fn pest_data(b: &mut Bencher) {
  b.iter(|| JsonParser::parse(Rule::json, DATA).unwrap());
}

#[bench]
fn pest_canada_collect(b: &mut Bencher) {
  b.iter(|| {
    let tokens: Vec<_> = JsonParser::parse(Rule::json, CANADA).unwrap().collect();
    tokens
  });
}

#[bench]
fn pest_data_collect(b: &mut Bencher) {
  b.iter(|| {
    let tokens: Vec<_> = JsonParser::parse(Rule::json, DATA).unwrap().collect();
    tokens
  });
}

