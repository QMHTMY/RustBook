use std::sync::Mutex;
use std::collections::HashMap;
use bigint::U256;
use utils::bkey::BKey;
use utils::serializer::{serialize, hash_str, hash_u8};
use leveldb::database::Database;
use crate::block::Block;
use crate::bcdb::BlockChainDb;
use crate::transaction::Transaction;
use crate::pow::ProofOfWork;

const INIT_BITS: u32 = 0x2100FFFF;
const SAVE_DIR: &str = "bc_db";
const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

pub struct BlockChain {
    blocks_db: Box<Database<BKey>>,
    blocks_index: Mutex<HashMap<String, Block>>,
    pub gnes_hash: String,
    pub curr_hash: String,
    pub curr_bits: u32,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut db = BlockChainDb::new(SAVE_DIR);
        let genesis = Self::genesis_block();
        Self::write_block(&mut db, &genesis);
        Self::write_tail(&mut db, &genesis);
        println!("New produced block saved!\n");

        let gene_block = genesis.clone();
        let mut block_index = Mutex::new(HashMap::new());
        Self::update_hmap(&mut block_index, gene_block);

        let gnes_hash = genesis.hash.clone();
        let curr_hash = genesis.hash.clone();
        BlockChain {
            blocks_db: Box::new(db),
            blocks_index: block_index,
            gnes_hash: gnes_hash,
            curr_hash: curr_hash,
            curr_bits: INIT_BITS,
        }
    }

    fn genesis_block() -> Block {
        println!("Start mining .... ");
        let from = "0x0000".to_string();
        let to   = "0x0000".to_string();
        let sign = "创世区块".to_string();
        let tx = Transaction::new(from, to, 0, 0, 0, sign);
        let mut block  = Block::new(vec![tx], PRE_HASH.to_string(), INIT_BITS);

        let header_ser = ProofOfWork::prepare_data(&mut block, 0);
        block.hash = hash_str(&header_ser);
        println!("Produced a new block!");

        block
    }

    pub fn add_block(&mut self, block: Block) {
        Self::write_block(&mut (self.blocks_db), &block);
        Self::write_tail(&mut (self.blocks_db), &block);
        println!("New produced block saved!\n");
        self.curr_hash = block.hash.clone();
        self.curr_bits = block.header.bits.clone();
        Self::update_hmap(&mut self.blocks_index, block);
    }

    fn update_hmap(hmap: &mut Mutex<HashMap<String, Block>>, block: Block) {
        let mut hmap = hmap.lock().unwrap();
        let hash = block.hash.clone();
        hmap.insert(hash, block);
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
        let mut hash = self.curr_hash.clone();
        let hmap = self.blocks_index.lock().unwrap();
        let mut blocks: Vec<Block> = Vec::new();

        loop {
            if let Some(b) = hmap.get(&hash) {
                blocks.push(b.clone());
                hash = b.header.pre_hash.clone();
            } else {
                panic!("Error getting block");
            }

            if hash == self.gnes_hash {
                if let Some(b) = hmap.get(&hash) {
                    blocks.push(b.clone());
                }
                break;
            }
        }
        blocks.reverse();

        for b in blocks {
            println!("{:#?}", b);
        }
    }
}
