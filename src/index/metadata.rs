use rocksdb::{DB, WriteBatch, ColumnFamily};
use serde_json::*;
use std::rc::Rc;
use bytekey2::{serialize};
use std::io::Write;

pub struct Metadata {
    cf_key: &'static str,
    db: Rc<DB>,
}

fn breadth_first_search_json<T, E, F>(start_key: String, json: serde_json::Value, mut f: F) -> std::result::Result<(), E> 
    where
        F: FnMut((String, &serde_json::Value)) -> std::result::Result<T, E>
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
                match f((key_str.to_owned(),v)) {
                    Ok(_) => {},
                    Err(e) => {return Err(e)}
                };
            },
        };

        nodes.append(&mut to_push);
        i += 1;
    }

    Ok(())
}

#[derive(Debug)]
pub enum SearchError {
    Any,
    F64CastError,
    SerializeError,
    UnreachableObjectReached, 
    UnreachableArrayReached,
    WriteError,
    CFOpenError,
}

impl Metadata {
    pub fn new(cf_key: &'static str, db: Rc<DB>) -> Metadata {

        Metadata {cf_key: cf_key, db: db}
    }

    pub fn insert_metadata(&self, id: u64, json: serde_json::Value) -> std::result::Result<(), SearchError> {
        
        let mut wb = WriteBatch::default();
        let cf = match self.db.cf_handle(self.cf_key) {
            Some(c) => c,
            None => return Err(SearchError::CFOpenError)
        };

        match breadth_first_search_json(format!("{0}", id), json, |value| {
            match value {
                (ref s, &Value::Bool(ref b)) => {
                    let key = format!("{0}_:t=b_{1}", *s, *b as u8);
                    wb.put_cf(cf, key, match b { true => vec![1], false => vec![0]});
                }
                (ref s, &Value::Null) => {
                    let key = format!("{0}_:t=n", *s);
                    wb.put_cf(cf, key, [0]);
                }
                (ref s, &Value::Number(ref nu)) => {
                    let value = serialize(&nu.as_f64().unwrap()).unwrap();
                    let mut key: Vec<u8> = Vec::new();
                    key.write(format!("{0}_:t=f_", *s).as_bytes()).unwrap();
                    key.write(&value).unwrap();
                    wb.put_cf(cf, key, value);
                }
                (ref s, &Value::String(ref s_)) => {
                    wb.put_cf(cf, format!("{0}_:t=s",*s), s);
                }

                (ref s, &Value::Object(ref o)) => {
                    return Err(SearchError::UnreachableObjectReached);
                }
                (ref s, &Value::Array(ref es)) => {
                    return Err(SearchError::UnreachableArrayReached);
                }
            };

            

            Ok(())
        }) {
            Ok(_) => {}
            Err(e) => {
                return Err(e)
            }
        }

        match self.db.write(wb) {
            Ok(_) => {},
            Err(e) => return Err(SearchError::WriteError)
        };

        Ok(())
        
    }

    fn search(&self, value: serde_json::Value) {
        
    }
        
}