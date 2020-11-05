mod blockchain;
use blockchain::{Blockchain, Block};


fn mine_block(blockchain:&mut Blockchain) -> Block {
  let previous_block = blockchain.get_previous_block();
  let previous_proof = previous_block.proof;
  let proof = Blockchain::proof_of_work(previous_proof);
  let previous_hash = Blockchain::hash(previous_block);
  let block = blockchain.create_block(proof, previous_hash);
  block
}

fn main() {
  // initialize blockchain
  let mut blockchain = Blockchain::new();
  println!("BlockRustChain Initialized!");

  for i in 0..5 {
    // mine block to use blockchain implementation
    let block = mine_block(&mut blockchain);
    println!("Count: {} \n", i);
    println!("Block Mine: {}", serde_json::to_string(&block).unwrap());
    println!("Blockchain List: {}", serde_json::to_string(&blockchain).unwrap());
    println!("Blockchain Length: {}", blockchain.chain.len());
    println!("Blockchain is valid: {}", blockchain.is_chain_valid());
  }
}
