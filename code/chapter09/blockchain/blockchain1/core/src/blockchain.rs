use crate::block::Block;

// 创世区块 pre_hash
const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain { blocks: vec![Self::genesis_block()] }
    }

    // 生成创世区块
    fn genesis_block() -> Block {
        Block::new("创世区块".to_string(), PRE_HASH.to_string())
    }

    // 添加区块，形成区块链
    pub fn add_block(&mut self, data: String) {
        // 获取前一个区块的哈希值
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let pre_hash  = pre_block.hash.clone();

        // 构建新区块并加入链
        let new_block = Block::new(data, pre_hash);
        self.blocks.push(new_block);
    }

    // 打印区块信息
    pub fn block_info(&self) {
        for b in self.blocks.iter() {
            println!("{:#?}", b);
        }
    }
}
