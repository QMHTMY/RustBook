use std::thread;
use std::time::Duration;
use crate::block::Block;
use utils::serializer::{serialize, hash_str, hash_u8};
use bigint::U256;

const MAX_NONCE: u32 = 0x7FFFFFFF;

pub struct ProofOfWork {
    target: U256,
}

impl ProofOfWork {
    pub fn new(bits: u32) -> Self {
        let (mant, expt) = {
            let unshifted_expt = bits >> 24;
            if unshifted_expt <= 3 {
                ((bits & 0xFFFFFF) >> (8 * (3-unshifted_expt as usize)), 0)
            } else {
                (bits & 0xFFFFFF, 8 * ((bits >> 24) - 3))
            }
        };

        if mant > 0x7FFFFF {
            Self {
                target: Default::default(),
            }
        } else {
            Self {
                target: U256::from(mant as u64) << (expt as usize),
            }
        }
    }

    pub fn run(&self, mut block: &mut Block) {
        println!("Start mining .... ");
        thread::sleep(Duration::from_secs(3));

        let mut nonce: u32 = 0;
        while nonce <= MAX_NONCE {
            let header_ser = Self::prepare_data(&mut block, nonce);
            let mut hash_u: [u8; 32] = [0; 32];
            hash_u8(&header_ser, &mut hash_u);

            let hash_int = U256::from(hash_u);
            if hash_int <= self.target {
                block.hash = hash_str(&header_ser);
                println!("Produce a new block!");
                return;
            }

            nonce += 1;
        }
    }

    fn prepare_data(block: &mut Block, nonce: u32) -> Vec<u8> {
        block.header.nonce = nonce;
        serialize(&(block.header))
    }
}
