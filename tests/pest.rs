#![feature(test)]
extern crate test;

extern crate pest;
extern crate pest_grammars;

use pest::Parser;
use pest_grammars::json::*;

const CANADA : &str = include_str!("../assets/canada.json");
const DATA : &str = include_str!("../assets/data.json");

#[test]
fn canada_test() {
  //println!("canada:\n{:?}", JsonParser::parse(Rule::json, CANADA).unwrap());
  panic!();
}

#[test]
fn data_test() {
  println!("data:\n{:#?}", JsonParser::parse(Rule::json, DATA).unwrap());
  println!("data tokens:\n{:#?}", JsonParser::parse(Rule::json, DATA).unwrap().tokens().collect::<Vec<_>>());
  panic!();
}
