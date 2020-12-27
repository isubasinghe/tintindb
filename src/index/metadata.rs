use rocksdb::{DB, Options, Cache, ColumnFamilyDescriptor};
use serde_json::*;
use std::cell::{Cell, RefCell};
use std::borrow::Cow;

pub struct Metadata {
    cf_name: &'static str
}

pub enum Process<'a> {
    Single(String, &'a Value),
}

impl Metadata {
    fn new(cf_name: &'static str) -> Metadata {
        Metadata {cf_name: cf_name}
    }

    pub fn insert_metadata(&self, id: u64, json: serde_json::Value) {
        let mut i = 0;
        let mut nodes: Vec<(String, &Value)> = Vec::new();
        let mut processed: Vec<(String, &Value)> = Vec::new();
        nodes.push(("".to_string(), &json));

        loop {
            if i >= nodes.len() {
                break;
            }
            
            let mut to_push: Vec<(String, &Value)>  = Vec::new();

            match nodes.get_mut(i).unwrap() {
                (key_str, Value::Null) => {
                    processed.push((key_str.to_owned(), &nodes[i].1));
                },
                (key_str, Value::Array(es)) => {

                    let mut iter = es.iter();
                    let mut count: u64 = 0;

                    while let Some(e) = iter.next() {
                        let key = format!("{0}_{1}", key_str, count);
                        to_push.push((key, e));
                        count += 1;
                    }
                },

                (key_str, Value::Bool(b)) => {
                    processed.push((key_str.to_owned(), &nodes[i].1));
                },
                (key_str, Value::Number(n)) => {
                    processed.push((key_str.to_owned(), &nodes[i].1));
                },
                (key_str, Value::Object(o)) => {
                    let mut iter =o.iter();
                    let mut count: u64 = 0;

                    while let Some((k, v)) = iter.next() {
                        let key = format!("{0}_{1}", key_str, k);
                        to_push.push((key, v));
                    }
                }
                (key_str, Value::String(s)) => {
                    processed.push((key_str.to_owned(), &nodes[i].1));
                }
            }

            nodes.append(& mut to_push);

            i += 1;

        }
        
    }
        
}

mod test {
    use super::*;

    #[test]
    fn test() {
        let q: &'static str = r#"
            {
                "hello": "asd",
                "main": [1, 2, {
                    "hello": 2, 
                    "z": "z"
                }]
            }
        "#;
        let x: Value = serde_json::from_str(&q).unwrap();
        println!("{0}", x);

        let meta = Metadata::new("A");
        meta.insert_metadata(10, x);

    }
}