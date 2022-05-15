use crate::block::Block;
use crate::pow::ProofOfWork;
use crate::transaction::Transaction;

const MINER_NAME: &str = "anonymous";

#[derive(Debug, Clone)]
pub struct Miner {
    name: String,
    pub balance: u64,
    address: String,
}

impl Miner {
    pub fn new(address: String) -> Self {
        Miner {
            name: MINER_NAME.to_string(),
            balance: 100,
            address: address,
        }
    }

    pub fn mine_block(&mut self, txs: &mut Vec<Transaction>, pre_hash: String, bits: u32)
        -> Block {
        let mut fee = 0; // 挖矿手续费
        for tx in txs.iter() {
            fee += tx.fee.clone();
        }

        let from = "0x0000".to_string();
        let to = self.address.clone();
        let sign = format!("{} -> {}: 50 btc", from, to);
        let coinbase = Transaction::new(from, to, 0, 0, 0, sign);

        let mut txs_all: Vec<Transaction> = Vec::new();
        txs_all.push(coinbase);
        txs_all.append(txs);
        let block = Self::mine_job(txs_all, pre_hash, bits);

        self.balance += 50; // 挖矿奖励，实际中会半衰 50、25、12.5
        self.balance += fee;

        block
    }

    fn mine_job(txs: Vec<Transaction>, pre_hash: String, bits: u32) -> Block {
        let mut block = Block::new(txs, pre_hash, bits);
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    pub fn miner_info(&self) {
        println!("{:#?}", &self);
    }
}
