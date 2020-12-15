mod blockchain;
mod block;
mod hashable;

use blockchain::Blockchain as Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add("Hello World");
    blockchain.add("test");
    println!("Blockchain is valid {}", blockchain.is_valid());
}
