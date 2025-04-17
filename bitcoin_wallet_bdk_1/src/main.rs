use anyhow::Result;
use bdk_wallet::{
    bitcoin::Network,
    KeychainKind,
    Wallet,
    rusqlite::Connection,
};

fn main() -> Result<()> {
    // Create a simple descriptor wallet
    let external_descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
    let internal_descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";

    // Create a database for the wallet
    let mut db = Connection::open_in_memory()?;

    // Create a BDK 1.0 wallet
    let mut wallet = Wallet::create(external_descriptor, internal_descriptor)
        .network(Network::Testnet)
        .create_wallet(&mut db)?;

    // Get a new address from the wallet (peek at index 0)
    let address = wallet.peek_address(KeychainKind::External, 0);

    println!("BDK 1.0 Simple Bitcoin Wallet");
    println!("----------------------------");
    println!("Network: Testnet");
    println!("New receiving address: {}", address);

    // Reveal addresses to generate them in the wallet
    for addr in wallet.reveal_addresses_to(KeychainKind::External, 5) {
        println!("Revealed address: {}", addr);
    }

    // Get wallet balance
    let balance = wallet.balance();
    println!("Wallet balance: {} sats", balance.total());

    // Get a change address
    let change_address = wallet.peek_address(KeychainKind::Internal, 0);
    println!("Change address: {}", change_address);

    // Persist wallet changes
    wallet.persist(&mut db)?;

    Ok(())
}