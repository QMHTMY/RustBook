use leveldb::kv::KV;
use leveldb::database::Database;
use leveldb::options::{Options, WriteOptions};
use utils::bkey;
use std::{env, fs};

pub struct BlockChainDb;

impl BlockChainDb {
    pub fn new(path: &str) -> Database<bkey::BKey> {
        let mut dir = env::current_dir().unwrap();
        dir.push(path);

        let path_buf = dir.clone();
        fs::create_dir_all(dir).unwrap();

        let path = path_buf.as_path();
        let mut opts = Options::new();
        opts.create_if_missing = true;

        let database = match Database::open(path, opts) {
            Ok(db) => db,
            Err(e) => panic!("Failed to open database: {:?}", e),
        };

        database
    }

    pub fn write_db(db: &mut Database<bkey::BKey>, key: bkey::BKey, val: &[u8]) {
        let write_opts = WriteOptions::new();
        match db.put(write_opts, key, &val) {
            Ok(_) => (),
            Err(e) => panic!("Failed to write block to database: {:?}", e),
        }
    }
}
