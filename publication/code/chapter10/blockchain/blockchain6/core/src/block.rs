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
        let txs_hash = Self::merkle_hash_str(&txs);

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

    fn merkle_hash_str(txs: &Vec<Transaction>) -> String {
        if txs.len() == 0 {
            return "00000000".to_string();
        }

        let mut merkle_tree: Vec<String> = Vec::new();
        for tx in txs {
            merkle_tree.push(tx.hash.clone());
        }

        let mut j: u64 = 0;
        let mut size = merkle_tree.len();
        while size > 1 {
            let mut i: u64 = 0;
            let temp = size as u64;

            while i < temp {
                let k = Self::min(i + 1, temp - 1);
                let idx1 = (j + i) as usize;
                let idx2 = (j + k) as usize;
                let hash1 = merkle_tree[idx1].clone();
                let hash2 = merkle_tree[idx2].clone();
                let merge: String = format!("{}-{}",hash1, hash2);
                let merge_ser = serialize(&merge);
                let merge_hash = hash_str(&merge_ser);
                merkle_tree.push(merge_hash);
                i += 2;
            }

            j += temp;
            size = (size + 1) >> 1;
        }

        if merkle_tree.len() != 0 {
            merkle_tree.pop().unwrap()
        } else {
            "00000000".to_string()
        }
    }

    fn min(num1: u64, num2: u64) -> u64 {
        if num1 >= num2 {
            num2
        } else {
            num1
        }
    }
}
