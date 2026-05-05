# Smart Contracts

All contracts are written in Rust using the [Soroban SDK](https://soroban.stellar.org) and deployed on the Stellar network.

## Building

```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

## Deploying (testnet)

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/settlement_escrow.wasm \
  --source <your_secret_key> \
  --network testnet
```

## Contracts

### CustodyLedger
Tracks fiat-backed on-chain balances. Only callable by the API's operator key.

### SettlementEscrow
Holds funds during in-flight settlements. Funds are locked on deposit initiation and released on confirmation.

### LiquidityPool
Allows liquidity providers to deposit and withdraw USDC. Used for instant settlement.

### FeeEngine
Stateless fee calculator. Default fee: 30 basis points (0.30%).

### ComplianceRegistry
Admin-controlled allowlist/blocklist for KYC/AML compliance.
