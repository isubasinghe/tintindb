use rocksdb::{DB};
use serde_json::*;
use std::rc::Rc;
pub struct Metadata {
    cf_name: &'static str, 
    db: Rc<DB>,
}

fn breadth_first_search_json<T, F>(start_key: String, json: serde_json::Value, f: F) 
    where
        F: Fn((String, &serde_json::Value)) -> T
{
    let mut i = 0;
    let mut nodes: Vec<(String, &serde_json::Value)> = Vec::new();

    nodes.push((start_key.to_owned(), &json));

    loop {
        if i >= nodes.len() {
            break;
        }

        let mut to_push: Vec<(String, &Value)> = Vec::new();

        match nodes.get_mut(i).unwrap() {
            (key_str, Value::Array(es)) => {

                let mut iter = es.iter();
                let mut count: u64 = 0;

                while let Some(e) = iter.next() {
                    let key = format!("{0}_{1}", key_str, count);
                    to_push.push((key, e));
                    count += 1;
                }
            },
            
            (key_str, Value::Object(o)) => {
                let mut iter =o.iter();
                while let Some((k, v)) = iter.next() {
                    let key = format!("{0}_{1}", key_str, k);
                    to_push.push((key, v));
                }
            }

            (key_str, v) => {
                f((key_str.to_owned(),v));
            },
        };

        nodes.append(&mut to_push);
        i += 1;
    }
}

impl Metadata {
    pub fn new(cf_name: &'static str, db: Rc<DB>) -> Metadata {
        Metadata {cf_name: cf_name, db: db}
    }

    pub fn insert_metadata(&self, id: u64, json: serde_json::Value) {

        breadth_first_search_json(format!("{0}", id), json, |x| {
            println!("{0}\t:\t{1}", x.0, x.1);
        });
        
    }
        
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bfs_json() {
        let data: &'static str = r#"
            {
                "hello": {
                    "f": [0, 1]
                },
                "main": [1, 2, {
                    "hello": 2, 
                    "z": "z",
                    "q": null
                }]
            }
        "#;
        
        let parsed_data: Value = serde_json::from_str(&data).unwrap();
    }
}