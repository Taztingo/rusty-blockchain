mod block;
use block::Block as Block;
use rusty_blockchain::*;

fn main() {
    let mut block = Block::new(0, now(), vec![0;32], 0, "Genesis Block".to_owned());

    println!("{:?}", &block);

    let h = block.hash();
    println!("{:?}", &h);

    block.hash = h;
    println!("{:?}", &block);
}
