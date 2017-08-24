// pest. The Elegant Parser
// Copyright (C) 2017  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;
extern crate pest;
extern crate pest_grammars;

use std::collections::HashMap;

use test::Bencher;

use pest::inputs::{Input, Span};
use pest::iterators::Pair;
use pest::Parser;

use pest_grammars::json::*;

enum Json<I: Input> {
    Null,
    Bool(bool),
    Number(f64),
    String(Span<I>),
    Array(Vec<Json<I>>),
    Object(HashMap<Span<I>, Json<I>>)
}

fn consume<I: Input>(pair: Pair<Rule, I>) -> Json<I> {
    fn value<I: Input>(pair: Pair<Rule, I>) -> Json<I> {
        let pair = pair.into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::null => Json::Null,
            Rule::bool => {
                match pair.into_span().as_str() {
                    "false" => Json::Bool(false),
                    "true" => Json::Bool(true),
                    _ => unreachable!()
                }
            }
            Rule::number => {
                Json::Number(pair.into_span().as_str().parse().unwrap())
            }
            Rule::string => {
                Json::String(pair.into_span())
            }
            Rule::array => {
                Json::Array(pair.into_inner().map(value).collect())
            }
            Rule::object => {
                let pairs = pair.into_inner().map(|pos| {
                    let mut pair = pos.into_inner();

                    let key = pair.next().unwrap().into_span();
                    let value = value(pair.next().unwrap());

                    (key, value)
                });

                Json::Object(pairs.collect())
            }
            _ => unreachable!()
        }
    }

    value(pair)
}

const CANADA : &str = include_str!("../assets/canada.json");

#[bench]
fn full_pest(b: &mut Bencher) {
    b.iter(|| {
        consume(JsonParser::parse_str(Rule::json, CANADA).unwrap().next().unwrap())
    });
}
