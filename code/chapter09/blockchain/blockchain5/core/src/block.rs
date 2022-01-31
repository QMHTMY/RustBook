use chrono::prelude::*;
use serde::Serialize;
use utils::serializer::{serialize, hash_str};
use crate::pow::ProofOfWork;
use crate::transaction::Transaction;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub nonce: u32,
    pub bits: u32,
    pub time: i64,
    pub txs_hash: String,
    pub pre_hash: String,
}

#[derive(Serialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub tranxs: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(txs: Vec<Transaction>, pre_hash: String, bits: u32) -> Self {
        let time = Utc::now().timestamp();
        let txs_ser = serialize(&txs);
        let txs_hash = hash_str(&txs_ser);

        let mut block = Block {
            header: BlockHeader {
                nonce: 0,
                time: time,
                bits: bits,
                txs_hash: txs_hash,
                pre_hash: pre_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        };

        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }
}
