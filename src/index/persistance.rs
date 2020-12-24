use rocksdb::{DB, Options, Cache, ColumnFamily,  ColumnFamilyDescriptor};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Deserialize, Serialize)]
pub enum Size {
    GB(u64),
    MB(u64),
    KB(u64),
    B(u64)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StoreConfig {
    pub path: String,
    pub cache: Size,
    pub wal: Size,
}

impl StoreConfig {
    
    #[inline(always)]
    fn get_bytes(&self, size: &Size) -> u64 {
        match size {
            Size::GB(n) => n * 1000 * 1000 * 1000,
            Size::MB(n) => n * 1000 * 1000,
            Size::KB(n) => n * 1000,
            Size::B(n) => n * 1
        }
    }

    #[inline(always)]
    fn get_cache_bytes(&self) -> u64 {
        self.get_bytes(&self.cache)
    }

    #[inline(always)]
    fn get_wal_bytes(&self) -> u64 {
        self.get_bytes(&self.wal)
    }
}

pub struct DocumentStore {
    db: DB,

}

#[derive(Debug)]
pub enum IntialisationError {
    DBOpenFail,
    CacheSetupError,
    CFHandleFailure, 
}

impl DocumentStore {
    pub fn new(config: &StoreConfig) -> Result<DocumentStore, IntialisationError>{

        let cache = match Cache::new_lru_cache(config.get_cache_bytes() as usize) {
            Ok(cache) => cache, 
            Err(err) => {
                return Err(IntialisationError::CacheSetupError);
            }
        };

        let mut opts = Options::default();
        // Max WAL to 4 GB
        opts.set_max_total_wal_size(config.get_wal_bytes());
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.increase_parallelism(4);
        opts.set_row_cache(&cache);
        opts.set_inplace_update_support(true);
        opts.enable_statistics();

        let cf_opts = Options::default();

        let document_cf = ColumnFamilyDescriptor::new("documents", cf_opts.clone());
        let corpus_cf =  ColumnFamilyDescriptor::new("corpus", cf_opts.clone());
        let frequencies_cf = ColumnFamilyDescriptor::new("frequencies", cf_opts.clone());

        let db = match DB::open_cf_descriptors(&opts, config.path.to_owned(), vec![document_cf, corpus_cf, frequencies_cf]) {
            Ok(db) => db, 
            Err(err) => {
                return Err(IntialisationError::DBOpenFail);
            }
        };

        // test if handles actually exist
        match db.cf_handle("documents") {
            Some(_) => {}, 
            None => return Err(IntialisationError::CFHandleFailure)
        };

        match db.cf_handle("corpus") {
            Some(_) =>{}, 
            None => return Err(IntialisationError::CFHandleFailure)
        };

        Ok(DocumentStore{db: db})
    }
}