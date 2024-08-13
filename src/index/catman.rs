use rocksdb::DB;
use std::sync::Arc;

#[derive(Debug)]
pub struct CatMan {
    db: Arc<DB>,
    cf_name: &'static str,
}

impl CatMan {
    pub fn new(cf_name: &'static str, db: Arc<DB>) -> CatMan {
        CatMan {
            db: db,
            cf_name: cf_name,
        }
    }

    #[inline(always)]
    pub fn in_cat(self, word: &'static str, cat: &'static str) {}
}
