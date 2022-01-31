use chrono::prelude::*;
use serde::Serialize;
use crate::pow::ProofOfWork;
use utils::serializer::{serialize, hash_str};

// 区块头
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub nonce: u32,
    pub bits: u32,
    pub time: i64,
    pub txs_hash: String,
    pub pre_hash: String,
}

// 区块
#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub tranxs: String,
    pub hash: String,
}

impl Block {
    pub fn new(txs: String, pre_hash: String, bits: u32) -> Self {
        let time = Utc::now().timestamp();
        let txs_ser = serialize(&txs);
        let txs_hash = hash_str(&txs_ser);

        let mut block = Block {
            header: BlockHeader {
                time: time,
                txs_hash: txs_hash,
                pre_hash: pre_hash,
                bits: bits,
                nonce: 0,
            },
            tranxs: txs,
            hash: "".to_string(),
        };

        // 初始化挖矿任务并开始挖矿
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }
}
