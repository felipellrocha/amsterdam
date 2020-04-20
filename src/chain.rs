use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::utils::{DIFFICULT_LEVEL, get_difficult_string};

#[derive(Debug)]
pub struct Block {
    index: i32,
    time: DateTime<Utc>,
    transactions: Vec<Transaction>,
    proof: i32,
    previous_hash: String,
    hash: String,
}

#[derive(Debug)]
pub struct Transaction {
    id: Uuid,
    sender: Uuid,
    recipient: Uuid,
    amount: i64,
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub transactions: Vec<Transaction>,
}

impl Block {
  pub fn new(index: i32, proof: i32, transactions: Vec<Transaction>, last_block: Option<&Block>) -> Block {
    Block {
      index,
      time: Utc::now(),
      transactions,
      proof,
      hash: String::new(),
      previous_hash: match last_block {
        None => String::from("genesis"),
        Some(data) => data.calculate_hash(),
      },
    }
  }
  /*
    None => {
      let mut hash = vec![];
      hash.extend(b"genesis");
      crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &hash)
    },
  */

  pub fn mine(&mut self) {
    let target = get_difficult_string();

    let mut proof = 0;

    while !self.calculate_proof(&target, &self.previous_hash, proof) {
      proof += 1;
    }

    self.proof = proof;
  }

  pub fn calculate_proof(&self, target: &str, hash: &str, proof: i32) -> bool {
      let mut bytes = vec![];

      bytes.extend(hash.as_bytes());
      bytes.extend(&proof.to_ne_bytes());

      let work = crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes);
      //println!("({:}, {:}), checking {:}", proof, hash, work);
      
      &work[..DIFFICULT_LEVEL as usize] == target
  }

  pub fn calculate_hash(&self) -> String {
    let mut hash = vec![];

    hash.extend(&self.time.timestamp().to_ne_bytes());
    hash.extend(self.transactions
      .iter()
      .flat_map(|transaction| transaction.bytes())
      .collect::<Vec<u8>>()
    );
    hash.extend(&self.proof.to_ne_bytes());
    hash.extend(self.previous_hash.as_bytes());

    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &hash)
  }
}

impl Transaction {
  pub fn bytes(&self) -> Vec<u8> {
    let mut bytes = vec![];

    bytes.extend(self.id.as_bytes());
    bytes.extend(self.sender.as_bytes());
    bytes.extend(self.recipient.as_bytes());
    bytes.extend(&self.amount.to_ne_bytes());

    bytes
  }

  pub fn new(sender: Uuid, recipient: Uuid, amount: i64) -> Transaction {
    let id = Uuid::new_v4();
    Transaction { id, sender, recipient, amount }
  }
}

impl Blockchain {
  pub fn new() -> Blockchain {
    Blockchain {
      chain: vec![],
      transactions: vec![],
    }
  }

  pub fn receive_transaction(&mut self, transaction: Transaction) -> i32 {
    self.transactions.push(transaction);

    if self.chain.len() > 0 {
      let block = &self.chain[self.chain.len() - 1];

      block.index + 1
    } else {
      0
    }
  }

  pub fn create_block(mut self, proof: i32) -> Self {
    let index = self.chain.len() as i32;
    let transactions = self.transactions;

    self.transactions = vec![];

    let mut block = Block::new(
      index,
      proof,
      transactions,
      self.chain.last(),
    );

    block.mine();

    self.chain.push(block);

    self
  }
}
