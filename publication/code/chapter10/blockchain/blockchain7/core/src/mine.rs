use crate::miner::Miner;
use crate::blockchain::BlockChain;
use crate::transaction::Transaction;

const MINER_ADDRESS: &str = "0x1b2d";

// 挖矿任务
pub struct Mine {
    pub miner: Miner,
    pub blockchain: BlockChain,
}

impl Mine {
    pub fn new() -> Self {
        Mine {
            blockchain: BlockChain::new(),
            miner: Miner::new(MINER_ADDRESS.to_string()),
        }
    }

    pub fn mining(&mut self, txs: &mut Vec<Transaction>) {
        // 准备 pre_hash 和难度值
        let pre_hash = self.blockchain.curr_hash.clone();
        let bits = self.blockchain.curr_bits.clone();

        // 核心代码点：开始挖矿
        let block = self.miner.mine_block(txs, pre_hash, bits);

        // 区块保存
        self.blockchain.add_block(block);
    }
}
