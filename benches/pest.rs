#![feature(test)]
extern crate test;

extern crate pest;
extern crate pest_grammars;

use test::Bencher;

use pest::Parser;
use pest_grammars::json;

const CANADA : &str = include_str!("../assets/canada.json");

#[bench]
fn pest(b: &mut Bencher) {
  b.iter(||{
    json::JsonParser::parse_str(json::Rule::json, CANADA).unwrap_or_else(|e| panic!("{}", e))
  });

}
