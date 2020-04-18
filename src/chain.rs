use uuid::Uuid;

#[derive(Debug)]
pub struct Block {
    index: i32,
    timestamp: i32,
    transactions: Vec<Uuid>,
    proof: i32,
    previous_hash: Uuid,
}

#[derive(Debug)]
pub struct Transaction {
    id: Uuid,
    sender: Uuid,
    recipient: Uuid,
    amount: i64,
}

#[derive(Debug)]
pub struct Chain {
    pub chain: Vec<Block>,
    pub transactions: Vec<Transaction>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            chain: vec![],
            transactions: vec![],
        }
    }

    pub fn new_block(&self) {

    }

    pub fn new_transaction(&mut self, sender: Uuid, recipient: Uuid, amount: i64) -> i32 {
        let id = Uuid::new_v4();
        let transaction = Transaction { id, sender, recipient, amount };

        self.transactions.push(transaction);

        if self.chain.len() > 0 {
            let block = &self.chain[self.chain.len() - 1];

            block.index + 1
        } else {
            0
        }
    }

    pub fn hash(&self) {

    }

    pub fn last_block(&self) -> Option<Block> {
      unimplemented!()
    }
}
