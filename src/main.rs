mod chain;
mod utils;

use uuid::Uuid;
use crate::chain::{Blockchain, Transaction};

fn main() {
    let mut chain = Blockchain::new();

    chain.receive_transaction(Transaction::new(Uuid::new_v4(), Uuid::new_v4(), 5000));
    chain.receive_transaction(Transaction::new(Uuid::new_v4(), Uuid::new_v4(), 20000));

    chain = chain.create_block(123);

    chain.receive_transaction(Transaction::new(Uuid::new_v4(), Uuid::new_v4(), 120000));

    chain = chain.create_block(456);

    println!("hello, {:#?}", chain);
}
