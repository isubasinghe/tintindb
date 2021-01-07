use rocksdb::{DB};
use std::rc::Rc;

pub struct CatMan {
    db: Rc<DB>,
    cf_name: &'static str,
}


impl CatMan {
    pub fn new(cf_name: &'static str, db: Rc<DB>) -> CatMan {
        CatMan{db: db, cf_name: cf_name}
    }

    #[inline(always)]
    pub fn in_cat(self, word: &'static str, cat: &'static str) {

    }
}
