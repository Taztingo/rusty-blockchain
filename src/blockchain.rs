use crate::block::Block;
use crate::hashable::Hashable;
use rusty_blockchain::now;

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Blockchain {
            blocks: Vec::new()
        };
        chain.add("Genesis");
        chain
    }

    pub fn add(&mut self, payload: &str) {
        if self.blocks.len() == 0 {
            let block = Block::new(self.blocks.len() as u32, 
        now(), vec![0;32], 0, payload.to_owned());
            self.blocks.push(block);
        } else {
            let previous_hash = self.blocks[self.blocks.len() - 1].hash.clone();
            let mut block = Block::new(self.blocks.len() as u32, 
        now(), previous_hash, 0, payload.to_owned());
            block.hash = block.hash();
            self.blocks.push(block);
        }
    }

    pub fn latest(&self) -> Option<&Block> {
        if self.blocks.len() == 0 {
            None
        } else {
            Some(&self.blocks[self.blocks.len() - 1])
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i-1];

            if current_block.hash != current_block.hash() {
                return false;
            }

            if current_block.previous_block != previous_block.hash {
                return false;
            }
        }

        true
    }
}

