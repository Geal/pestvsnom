#![feature(test)]

extern crate test;

use test::Bencher;
use faster_pest::*;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "src/json.pest"]
pub struct JsonParser;

#[derive(Debug)]
enum Value<'i> {
    String(Cow<'i, str>),
    Number(f64),
    Boolean(bool),
    Array(Vec<Value<'i>>),
    Object(HashMap<Cow<'i, str>, Value<'i>>),
    Null,
}

impl<'i> Value<'i> {
    fn from_ident_ref(value: IdentRef<'i, Ident>) -> Self {
        match value.as_rule() {
            Rule::string => Value::String(json_text_to_string(value.as_str())),
            Rule::number => Value::Number(value.as_str().parse().unwrap()),
            Rule::boolean => Value::Boolean(value.as_str() == "true"),
            Rule::array => {
                let mut array = Vec::new();
                array.extend(value.children().map(Value::from_ident_ref));
                Value::Array(array)
            }
            Rule::object => {
                let mut object = HashMap::new();
                for property in value.children() {
                    let mut property_children = property.children();
                    let name = property_children.next().unwrap();
                    let name = json_text_to_string(name.as_str());
                    let value = property_children.next().unwrap();
                    object.insert(name, Value::from_ident_ref(value));
                }
                Value::Object(object)
            }
            Rule::null => Value::Null,
            Rule::property | Rule::file => unreachable!(),
        }
    }
}

fn json_text_to_string(s: &str) -> Cow<str> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < s.len() {
        if bytes[i] == b'\\' {
            let mut result = bytes.to_vec();
            let mut j = i;
            while j < result.len() {
                if result[j] == b'\\' {
                    result.remove(j);
                    match result[j] {
                        b'n' => result[j] = b'\n',
                        b'\\'  | b'"' | b'/' => (),
                        b't' => result[j] = b'\t',
                        b'r' => result[j] = b'\r',
                        b'b' => result[j] = b'\x08',
                        b'f' => result[j] = b'\x0C',
                        _ => todo!()
                    }
                }
                j += 1;
            }
            return Cow::Owned(unsafe { String::from_utf8_unchecked(result) })
        }
        i += 1;
    }
    Cow::Borrowed(s)
}

const CANADA : &str = include_str!("../assets/canada.json");
const DATA   : &str = include_str!("../assets/data.json");

#[bench]
fn faster_pest_canada_shallow(b: &mut Bencher) {
    b.iter(|| {
        JsonParser::parse_file(CANADA).expect("unsuccessful parse");
    });
}

#[bench]
fn faster_pest_canada(b: &mut Bencher) {
    b.iter(|| {
        let output = JsonParser::parse_file(CANADA).map_err(|e| e.print(CANADA)).unwrap();
        let file = output.into_iter().next().unwrap();
        let main_object = file.children().next().unwrap();
        let output = Value::from_ident_ref(main_object);
    });
}

#[bench]
fn faster_pest_data_shallow(b: &mut Bencher) {
    b.iter(|| {
        JsonParser::parse_file(DATA).expect("unsuccessful parse");
    });
}

#[bench]
fn faster_pest_data(b: &mut Bencher) {
    b.iter(|| {
        let output = JsonParser::parse_file(DATA).map_err(|e| e.print(DATA)).unwrap();
        let file = output.into_iter().next().unwrap();
        let main_object = file.children().next().unwrap();
        let output = Value::from_ident_ref(main_object);
    });
}
