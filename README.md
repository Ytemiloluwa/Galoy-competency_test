# Galoy-competency_test
Galoy_competency_test

## BDK 0.x to BDK 1.0 Upgrade Documentation

### Changes Made:
1. **Dependency Update**:
   - Updated the `Cargo.toml` file to use `bdk_wallet = { version = "1.0.0", features = ["rusqlite"] }`.

2. **Code Changes**:
   - Replaced `bdk` with `bdk_wallet` in the `main.rs` file.
   - Updated the wallet creation process to use `Wallet::create` instead of `Wallet::new`.
   - Modified address generation to use `peek_address` and `reveal_addresses_to` methods.
   - Updated balance retrieval to use the `balance` method.
   - Added persistence of wallet changes using `wallet.persist`.

### Summary:
The upgrade to BDK 1.0 involved updating dependencies and modifying the wallet creation, address generation, balance retrieval, and persistence processes to align with the new API.
