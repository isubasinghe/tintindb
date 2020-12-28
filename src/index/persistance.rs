use rocksdb::{DB, Options, Cache, ColumnFamilyDescriptor};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use crate::dtos;
use crate::index::metadata::{Metadata};

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

static DOCUMENTS: &'static str = "DOCUMENTS";
static CORPUS: &'static str = "CORPUS";
static FREQUENCIES: &'static str = "FREQUENCIES";
static METADATA: &'static str = "METADTA";

pub struct DocumentStore {
    db: Rc<DB>,
    metadata: Metadata,
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

        let document_cf = ColumnFamilyDescriptor::new(DOCUMENTS, cf_opts.clone());
        let corpus_cf =  ColumnFamilyDescriptor::new(CORPUS, cf_opts.clone());
        let frequencies_cf = ColumnFamilyDescriptor::new(FREQUENCIES, cf_opts.clone());

        let db = Rc::new(match DB::open_cf_descriptors(&opts, config.path.to_owned(), vec![document_cf, corpus_cf, frequencies_cf]) {
            Ok(db) => db, 
            Err(err) => {
                return Err(IntialisationError::DBOpenFail);
            }
        });

        // test if handles actually exist
        match db.cf_handle(DOCUMENTS) {
            Some(_) => {}, 
            None => return Err(IntialisationError::CFHandleFailure)
        };

        match db.cf_handle(CORPUS) {
            Some(_) =>{}, 
            None => return Err(IntialisationError::CFHandleFailure)
        };

        match db.cf_handle(FREQUENCIES) {
            Some(_) =>{}, 
            None => return Err(IntialisationError::CFHandleFailure)
        };

        match db.cf_handle(METADATA) {
            Some(_) =>{}, 
            None => return Err(IntialisationError::CFHandleFailure)
        };

        Ok(DocumentStore{db: db.clone(), metadata: Metadata::new(METADATA, db.clone())})
    }
    
}