use uuid::Uuid;

mod chain;

fn main() {
    let mut chain = chain::Chain::new();

    chain.new_transaction(Uuid::new_v4(), Uuid::new_v4(), 5000);
    chain.new_transaction(Uuid::new_v4(), Uuid::new_v4(), 20000);
    println!("hello, {:?}", chain);
}
