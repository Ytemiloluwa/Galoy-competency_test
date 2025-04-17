use anyhow::Result;
use bdk::{
    bitcoin::Network,
    database::MemoryDatabase,
    wallet::{AddressIndex, AddressInfo},
    Wallet,
};

fn main() -> Result<()> {
    // Create a simple descriptor wallet
    let descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)";
    let change_descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)";

    // Create a BDK wallet
    let wallet = Wallet::new(
        descriptor,
        Some(change_descriptor),
        Network::Testnet,
        MemoryDatabase::default(),
    )?;

    // Get a new address from the wallet
    let address: AddressInfo = wallet.get_address(AddressIndex::New)?;

    println!("BDK 0.28 Simple Bitcoin Wallet");
    println!("-----------------------------");
    println!("Network: Testnet");
    println!("New receiving address: {}", address.address);
    println!("Address index: {}", address.index);

    // Get wallet balance
    let balance = wallet.get_balance()?;
    println!("Wallet balance: {} sats", balance.get_total());

    // Display internal/change address
    let change_address = wallet.get_internal_address(AddressIndex::New)?;
    println!("Change address: {}", change_address.address);

    Ok(())
}

