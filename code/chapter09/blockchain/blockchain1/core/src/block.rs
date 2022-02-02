use std::thread;
use std::time::Duration;
use serde::Serialize;
use chrono::prelude::*;
use utils::serializer::{serialize, hash_str};

// 区块头
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
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
    pub fn new(txs: String, pre_hash: String) -> Self {
        // 用延迟 3 秒来模拟挖矿
        println!("Start mining .... ");
        thread::sleep(Duration::from_secs(3));

        // 准备时间、计算交易哈希值
        let time = Utc::now().timestamp();
        let txs_ser = serialize(&txs);
        let txs_hash = hash_str(&txs_ser);

        let mut block = Block {
            header: BlockHeader {
                time: time,
                txs_hash: txs_hash,
                pre_hash: pre_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        };
        block.set_hash();

        println!("Produce a new block!\n");

        block
    }

    // 计算并设置区块哈希值
    fn set_hash(&mut self) {
        let header = serialize(&(self.header));
        self.hash = hash_str(&header);
    }
}
