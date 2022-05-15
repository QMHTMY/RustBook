use serde::Serialize;
use utils::serializer::{serialize, hash_str};

#[derive(Serialize, Debug)]
pub struct Transaction {
    pub nonce: u64,
    pub amount: u64,
    pub fee: u64,
    pub from: String,
    pub to: String,
    pub sign: String,
    pub hash: String,
}

impl Transaction {
    pub fn new(from: String, to: String,
               amount: u64, fee: u64,
               nonce: u64, sign: String) -> Self
    {
        let mut tx = Transaction {
            nonce,
            amount,
            fee,
            from,
            to,
            sign,
            hash:"".to_string(),
        };
        tx.set_hash();

        tx
    }

    pub fn set_hash(&mut self) {
        let txs_ser = serialize(&self);
        self.hash = hash_str(&txs_ser);
    }
}
