use crate::block::Block;
use crate::pow::ProofOfWork;
use crate::transaction::Transaction;

const MINER_NAME: &str = "anonymous";

// 矿工
#[derive(Debug, Clone)]
pub struct Miner {
    name: String,
    balance: u64,
    address: String,
}

impl Miner {
    pub fn new(address: String) -> Self {
        Miner {
            name: MINER_NAME.to_string(),
            balance: 0,
            address: address,
        }
    }

    pub fn mine_block(&self, txs: &mut Vec<Transaction>, pre_hash: String, bits: u32)
        -> Block {
        let from = "0x0000".to_string();
        let to = self.address.clone();
        let sign = format!("{} -> {}: 50 btc", from, to);
        let coinbase = Transaction::new(from, to, 0, 0, 0, sign);

        // 加入 coinbase 交易和普通交易
        let mut txs_2: Vec<Transaction> = Vec::new();
        txs_2.push(coinbase);
        txs_2.append(txs);

        Self::mine_job(txs_2, pre_hash, bits)
    }

    // 挖矿任务-工作量证明
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
