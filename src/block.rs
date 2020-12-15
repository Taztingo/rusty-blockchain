use std::fmt::{self, Debug, Formatter};
type BlockHash = Vec<u8>;
use crate::hashable::Hashable;
use rusty_blockchain::*;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub previous_block: BlockHash,
    pub hash: BlockHash,
    pub nonce: u64,
    pub payload: String
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {}",
        &self.index,
        &hex::encode(&self.hash),
        &self.timestamp,
        &self.payload)
    }
}

impl Block {
    pub fn new(index: u32, timestamp: u128, previous_block: BlockHash, nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            previous_block,
            hash: vec![0; 32],
            nonce,
            payload,
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.previous_block);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes
    }
}