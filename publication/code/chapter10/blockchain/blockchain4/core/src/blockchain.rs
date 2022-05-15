use bigint::U256;
use leveldb::database::Database;
use utils::bkey::BKey;
use utils::serializer::{serialize, hash_u8};
use crate::block::Block;
use crate::bcdb::BlockChainDb;
use crate::transaction::Transaction;

const CURR_BITS: u32 = 0x2100FFFF;
const SAVE_DIR: &str = "bc_db";
const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

pub struct BlockChain {
    pub blocks: Vec<Block>,
    curr_bits: u32,
    blocks_db: Box<Database<BKey>>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut db = BlockChainDb::new(SAVE_DIR);
        let genesis = Self::genesis_block();
        Self::write_block(&mut db, &genesis);
        Self::write_tail(&mut db, &genesis);
        println!("New produced block saved!\n");

        BlockChain {
            blocks: vec![genesis],
            curr_bits: CURR_BITS,
            blocks_db: Box::new(db),
        }
    }

    fn genesis_block() -> Block {
        let from = "0x0000".to_string();
        let to   = "0x0000".to_string();
        let sign = "创世区块".to_string();
        let tx = Transaction::new(from, to, 0, 0, 0, sign);
        Block::new(vec![tx], PRE_HASH.to_string(), CURR_BITS)
    }

    pub fn add_block(&mut self, txs: Vec<Transaction>) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let pre_hash  = pre_block.hash.clone();
        let new_block = Block::new(txs, pre_hash, self.curr_bits);
        Self::write_block(&mut (self.blocks_db), &new_block);
        Self::write_tail(&mut (self.blocks_db), &new_block);
        println!("New produced block saved!\n");
        self.blocks.push(new_block);
    }

    fn write_block(db: &mut Database<BKey>, block: &Block) {
        let header_ser = serialize(&(block.header));
        let mut hash_u: [u8; 32] = [0; 32];
        hash_u8(&header_ser, &mut hash_u);

        let key = BKey{ val: U256::from(hash_u) };
        let val = serialize(&block);
        BlockChainDb::write_db(db, key, &val);
    }

    fn write_tail(mut db: &mut Database<BKey>, block: &Block) {
        let key = BKey{ val: U256::from("tail".as_bytes()) };
        let val = serialize(&(block.hash));
        BlockChainDb::write_db(&mut db, key, &val);
    }

    pub fn block_info(&self) {
        for b in self.blocks.iter() {
            println!("{:#?}", b);
        }
    }
}
