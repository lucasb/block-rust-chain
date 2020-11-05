use crypto::digest::Digest;
use crypto::sha2::Sha256;

use serde_json;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
  index: usize,
  timestamp: DateTime<Utc>,
  pub proof: i64,
  previous_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
  pub chain: Vec<Block>,
}

impl Blockchain {
  pub fn new() -> Self {
    let mut blockchain = Blockchain { chain: Vec::new() };
    Blockchain::create_block(&mut blockchain, 1, "0".to_string());
    blockchain
  }

  pub fn create_block(&mut self, proof:i64, previous_hash:String) -> Block {
    let block = Block { 
      index: self.chain.len() + 1,
      timestamp: Utc::now(),
      proof: proof, 
      previous_hash: previous_hash,
    };
    self.chain.push(block.clone());
    block
  }
  
  pub fn get_previous_block(&self) -> &Block {
    self.chain.last().unwrap()
  }

  pub fn is_chain_valid(&self) -> bool {
    let mut previous_block = &self.chain[0];
    let mut block_number = 1;
    while block_number < self.chain.len() {
      let block = &self.chain[block_number];
      if block.previous_hash != Blockchain::hash(&previous_block) 
          && !Blockchain::is_proof_valid(block.proof, previous_block.proof) {
        return false;
      }
      previous_block = block;
      block_number += 1;
    }
    true
  }

  pub fn proof_of_work(previous_proof:i64) -> i64 {
    let mut new_proof = 0;
    let mut valid_proof = false;
    while !valid_proof {
      new_proof += 1;
      valid_proof = Blockchain::is_proof_valid(new_proof, previous_proof);
    }
    new_proof
  }

  pub fn is_proof_valid(proof:i64, previous_proof:i64) -> bool {
    let mut hasher = Sha256::new();
    hasher.input_str(&(proof.pow(2) - previous_proof.pow(2)).to_string());
    let hash = hasher.result_str();
    hash[..4] == "0000".to_owned()
  }

  pub fn hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&serde_json::to_string(block).unwrap());
    hasher.result_str()
  }
}
