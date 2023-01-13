//#![feature(trace_macros)]
#![feature(test)]
extern crate test;

#[macro_use]
extern crate nom;

use nom::{alphanumeric, recognize_float, sp};

use test::Bencher;

use std::str;
use std::collections::HashMap;

pub fn is_string_character(c: u8) -> bool {
  //FIXME: should validate unicode character
  c != b'"' && c != b'\\'
}

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
  Str(&'a[u8]),
  Boolean(&'a[u8]),
  Num(&'a [u8]),
  Array(Vec<JsonValue<'a>>),
  //Object(HashMap<&'a [u8], JsonValue<'a>>),
  Object(Vec<(&'a [u8], JsonValue<'a>)>),
}

named!(float<&[u8]>, call!(recognize_float));

//FIXME: handle the cases like \u1234
named!(
  string<&[u8]>,
  recognize!(delimited!(
    char!('\"'),
    map_res!(
      escaped!(take_while1!(is_string_character), '\\', one_of!("\"bfnrt\\")),
      str::from_utf8
    ),
    char!('\"')
  ))
);

named!(
  boolean<&[u8]>,
  alt!(tag!("false") | tag!("true"))
);

named!(
  array<Vec<JsonValue>>,
  preceded!(sp, delimited!(
    char!('['),
    separated_list!(char!(','), value),
    preceded!(sp, char!(']'))
  ))
);

named!(
  key_value<(&[u8], JsonValue)>,
  preceded!(sp, separated_pair!(string, char!(':'), value))
);

/*
named!(
  hash<HashMap<&[u8], JsonValue>>,
  preceded!(sp, map!(
    delimited!(
      char!('{'),
      separated_list!(char!(','), key_value),
      preceded!(sp, char!('}'))
    ),
    |tuple_vec| tuple_vec
      .into_iter()
      .collect()
  ))
);
*/

named!(
  hash<Vec<(&[u8], JsonValue)>>,
  preceded!(
    sp,
    delimited!(
      char!('{'),
      separated_list!(char!(','), key_value),
      preceded!(sp, char!('}'))
    )
  )
);

named!(
  value<JsonValue>,
  preceded!(sp, alt!(
    hash    => { |h| JsonValue::Object(h)            } |
    array   => { |v| JsonValue::Array(v)             } |
    string  => { |s| JsonValue::Str(s)     } |
    float   => { |f| JsonValue::Num(f)               } |
    boolean => { |b| JsonValue::Boolean(b)           }
  ))
);

named!(
  root<JsonValue>,
  delimited!(
    call!(nom::sp),
    alt!(
      map!(hash, JsonValue::Object) |
      map!(array, JsonValue::Array)
    ),
    not!(complete!(nom::sp))
  )
);

const CANADA : &[u8] = include_bytes!("../assets/canada.json");
const DATA   : &[u8] = include_bytes!("../assets/data.json");
//const REDUCED   : &[u8] = include_bytes!("../assets/reduced.json");

#[test]
fn nom_spans_data_test() {
  println!("data:\n{:?}", root(&DATA[..]).unwrap());
}

#[test]
fn nom_spans_canada_test() {
  println!("canada:\n{:?}", root(&CANADA[..]).unwrap());
}

#[bench]
fn nom_spans_canada(b: &mut Bencher) {
  //println!("data:\n{:?}", value(&CANADA[..]));
  b.iter(||{
    root(&CANADA[..]).unwrap()
  });
}

#[bench]
fn nom_spans_data(b: &mut Bencher) {
  //println!("data:\n{:?}", value(&CANADA[..]));
  b.iter(||{
    root(&DATA[..]).unwrap()
  });
}
