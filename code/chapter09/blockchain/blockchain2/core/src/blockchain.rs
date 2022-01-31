use crate::block::Block;

// 难度值和创世区块哈希值
const CURR_BITS: u32 = 0x2100FFFF;
const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

// 区块链
pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub curr_bits: u32,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![Self::genesis_block()],
            curr_bits: CURR_BITS,
        }
    }

    fn genesis_block() -> Block {
        Block::new("创世区块".to_string(), PRE_HASH.to_string(), CURR_BITS)
    }

    pub fn add_block(&mut self, txs: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let pre_hash  = pre_block.hash.clone();
        let new_block = Block::new(txs, pre_hash, self.curr_bits);
        self.blocks.push(new_block);
    }

    pub fn block_info(&self) {
        for b in self.blocks.iter() {
            println!("{:#?}", b);
        }
    }
}
