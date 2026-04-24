# ✈️🏨 LoyaltyChain — Airline & Hotel Points on Stellar

> **Tradeable loyalty tokens built with Soroban smart contracts on the Stellar network.**  
> Turn airline miles and hotel points into real, peer-transferable digital assets.

---

## 📖 Project Description

LoyaltyChain is a **Soroban smart contract** that enables airlines and hotels to issue their loyalty points as **fully tradeable tokens** on the Stellar blockchain. Instead of siloed, non-transferable points locked inside a company's proprietary system, LoyaltyChain brings rewards onto a public, auditable ledger — giving members true ownership over the points they've earned.

Built on **Stellar's Soroban** (the EVM-compatible smart contract layer), the contract wraps Stellar's native SEP-41 token standard, meaning issued points are interoperable with the broader Stellar DeFi ecosystem: DEXes, wallets, liquidity pools, and atomic swaps.

Whether you're an airline, a hotel chain, or a travel platform, LoyaltyChain provides the foundation for a **next-generation loyalty programme** that members actually trust and value.

---

## 🚀 What It Does

LoyaltyChain manages the full lifecycle of loyalty points on-chain:

| Action | Who | Description |
|---|---|---|
| **Initialize** | Admin | Deploys and configures the programme (name, symbol, decimals) |
| **Award Points** | Issuer (airline/hotel) | Mints points to a member after a qualifying event (flight, stay) |
| **Redeem Points** | Member | Burns points in exchange for an upgrade, free night, or reward |
| **Transfer Points** | Member | Sends points peer-to-peer to any Stellar address — fully tradeable |
| **Check Balance** | Anyone | Reads a member's current points balance on-chain |
| **Update Issuer** | Admin | Rotates the issuer key without redeploying the contract |
| **Transfer Admin** | Admin | Hands off contract ownership securely |

Every action emits a **structured on-chain event**, enabling real-time indexing, analytics dashboards, and audit trails without relying on a centralised database.

---

## ✨ Features

### 🪙 Tradeable Loyalty Tokens
Points are **real SEP-41 tokens** — not database entries. Members can transfer them to friends, sell them on a DEX, or pool them with family members. True ownership, no walled garden.

### 🏗️ Built on Soroban (Stellar Smart Contracts)
Leverages Stellar's **Soroban** execution environment for:
- Near-zero transaction fees (fractions of a cent)
- 5-second finality
- Carbon-neutral consensus (Stellar Proof of Agreement)
- Native cross-border settlement

### 🔐 Role-Based Access Control
- **Admin** — deploys and governs the contract
- **Issuer** — the airline or hotel operator; the only address permitted to mint new points
- **Member** — any Stellar address that holds and controls their own points

Auth is enforced on-chain via `require_auth()` — no off-chain trust required.

### 🔄 Full Lifecycle Management
Award → Hold → Transfer → Redeem. The contract handles every stage:
- Minting on qualifying events (flights booked, nights stayed)
- Burning on redemption (upgrades, free nights, vouchers)
- Peer transfer between any two Stellar wallets

### 📣 On-Chain Events
Every state change (`award`, `redeem`, `transfer`, `new_issuer`, `new_admin`) emits a **structured event** that any indexer or frontend can subscribe to — making it trivial to build a real-time points dashboard or loyalty history feed.

### 🛡️ Safety Guards
- Re-initialisation protection (panics if called twice)
- Positive-amount enforcement on all mints, burns, and transfers
- Insufficient-balance check before redemptions
- All sensitive operations gated by Stellar's cryptographic auth

### 🔧 Configurable Per Programme
- Custom token name & symbol (e.g. `StellarAir Miles / SAM`, `GrandHotel Points / GHP`)
- Configurable decimal places (e.g. `2` for fractional points)
- Issuer address rotatable without contract redeployment

### 🧪 Unit Tested
Ships with an embedded Rust test suite using `soroban-sdk/testutils`:
- Happy-path initialisation
- Double-init panic protection
- (Extendable with award/redeem/transfer coverage)

---

## 🗂️ Project Structure

```
loyalty-points/
├── Cargo.toml                          # Workspace manifest
└── contracts/
    └── loyalty_token/
        ├── Cargo.toml                  # Contract crate config
        └── src/
            └── lib.rs                  # Smart contract source
```

---

## 🛠️ Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
cd loyalty-points
stellar contract build
```

The compiled `.wasm` binary lands at:
```
target/wasm32-unknown-unknown/release/loyalty_token.wasm
```

### Run Tests

```bash
cargo test --features testutils
```

### Deploy to Testnet

```bash
# Fund a testnet account
stellar keys generate --global alice --network testnet --fund

# Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/loyalty_token.wasm \
  --source alice \
  --network testnet
```

### Invoke — Award Points

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- \
  award_points \
  --token_address <TOKEN_ADDRESS> \
  --member <MEMBER_ADDRESS> \
  --amount 5000 \
  --memo "Flight JFK→LAX"
```

---

## 🔭 Roadmap

- [ ] Expiry / time-locked points (TTL-based storage)
- [ ] Tiered status tiers (Silver / Gold / Platinum) stored on-chain
- [ ] Cross-programme points swap via Stellar DEX
- [ ] Partner network: earn Hotel points on Airline bookings
- [ ] Frontend dashboard (React + Stellar Wallets Kit)
- [ ] Testnet demo deployment

---

## 📄 Licence

MIT — free to use, fork, and build upon.

---

> Built with ❤️ on [Stellar](https://stellar.org) · Powered by [Soroban](https://soroban.stellar.org)
wallet address:# ✈️🏨 LoyaltyChain — Airline & Hotel Points on Stellar

> **Tradeable loyalty tokens built with Soroban smart contracts on the Stellar network.**  
> Turn airline miles and hotel points into real, peer-transferable digital assets.

---

## 📖 Project Description

LoyaltyChain is a **Soroban smart contract** that enables airlines and hotels to issue their loyalty points as **fully tradeable tokens** on the Stellar blockchain. Instead of siloed, non-transferable points locked inside a company's proprietary system, LoyaltyChain brings rewards onto a public, auditable ledger — giving members true ownership over the points they've earned.

Built on **Stellar's Soroban** (the EVM-compatible smart contract layer), the contract wraps Stellar's native SEP-41 token standard, meaning issued points are interoperable with the broader Stellar DeFi ecosystem: DEXes, wallets, liquidity pools, and atomic swaps.

Whether you're an airline, a hotel chain, or a travel platform, LoyaltyChain provides the foundation for a **next-generation loyalty programme** that members actually trust and value.

---

## 🚀 What It Does

LoyaltyChain manages the full lifecycle of loyalty points on-chain:

| Action | Who | Description |
|---|---|---|
| **Initialize** | Admin | Deploys and configures the programme (name, symbol, decimals) |
| **Award Points** | Issuer (airline/hotel) | Mints points to a member after a qualifying event (flight, stay) |
| **Redeem Points** | Member | Burns points in exchange for an upgrade, free night, or reward |
| **Transfer Points** | Member | Sends points peer-to-peer to any Stellar address — fully tradeable |
| **Check Balance** | Anyone | Reads a member's current points balance on-chain |
| **Update Issuer** | Admin | Rotates the issuer key without redeploying the contract |
| **Transfer Admin** | Admin | Hands off contract ownership securely |

Every action emits a **structured on-chain event**, enabling real-time indexing, analytics dashboards, and audit trails without relying on a centralised database.

---

## ✨ Features

### 🪙 Tradeable Loyalty Tokens
Points are **real SEP-41 tokens** — not database entries. Members can transfer them to friends, sell them on a DEX, or pool them with family members. True ownership, no walled garden.

### 🏗️ Built on Soroban (Stellar Smart Contracts)
Leverages Stellar's **Soroban** execution environment for:
- Near-zero transaction fees (fractions of a cent)
- 5-second finality
- Carbon-neutral consensus (Stellar Proof of Agreement)
- Native cross-border settlement

### 🔐 Role-Based Access Control
- **Admin** — deploys and governs the contract
- **Issuer** — the airline or hotel operator; the only address permitted to mint new points
- **Member** — any Stellar address that holds and controls their own points

Auth is enforced on-chain via `require_auth()` — no off-chain trust required.

### 🔄 Full Lifecycle Management
Award → Hold → Transfer → Redeem. The contract handles every stage:
- Minting on qualifying events (flights booked, nights stayed)
- Burning on redemption (upgrades, free nights, vouchers)
- Peer transfer between any two Stellar wallets

### 📣 On-Chain Events
Every state change (`award`, `redeem`, `transfer`, `new_issuer`, `new_admin`) emits a **structured event** that any indexer or frontend can subscribe to — making it trivial to build a real-time points dashboard or loyalty history feed.

### 🛡️ Safety Guards
- Re-initialisation protection (panics if called twice)
- Positive-amount enforcement on all mints, burns, and transfers
- Insufficient-balance check before redemptions
- All sensitive operations gated by Stellar's cryptographic auth

### 🔧 Configurable Per Programme
- Custom token name & symbol (e.g. `StellarAir Miles / SAM`, `GrandHotel Points / GHP`)
- Configurable decimal places (e.g. `2` for fractional points)
- Issuer address rotatable without contract redeployment

### 🧪 Unit Tested
Ships with an embedded Rust test suite using `soroban-sdk/testutils`:
- Happy-path initialisation
- Double-init panic protection
- (Extendable with award/redeem/transfer coverage)

---

## 🗂️ Project Structure

```
loyalty-points/
├── Cargo.toml                          # Workspace manifest
└── contracts/
    └── loyalty_token/
        ├── Cargo.toml                  # Contract crate config
        └── src/
            └── lib.rs                  # Smart contract source
```

---

## 🛠️ Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
cd loyalty-points
stellar contract build
```

The compiled `.wasm` binary lands at:
```
target/wasm32-unknown-unknown/release/loyalty_token.wasm
```

### Run Tests

```bash
cargo test --features testutils
```

### Deploy to Testnet

```bash
# Fund a testnet account
stellar keys generate --global alice --network testnet --fund

# Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/loyalty_token.wasm \
  --source alice \
  --network testnet
```

### Invoke — Award Points

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- \
  award_points \
  --token_address <TOKEN_ADDRESS> \
  --member <MEMBER_ADDRESS> \
  --amount 5000 \
  --memo "Flight JFK→LAX"
```

---

## 🔭 Roadmap

- [ ] Expiry / time-locked points (TTL-based storage)
- [ ] Tiered status tiers (Silver / Gold / Platinum) stored on-chain
- [ ] Cross-programme points swap via Stellar DEX
- [ ] Partner network: earn Hotel points on Airline bookings
- [ ] Frontend dashboard (React + Stellar Wallets Kit)
- [ ] Testnet demo deployment

---

## 📄 Licence

MIT — free to use, fork, and build upon.

---

> Built with ❤️ on [Stellar](https://stellar.org) · Powered by [Soroban](https://soroban.stellar.org)
# ✈️🏨 LoyaltyChain — Airline & Hotel Points on Stellar

> **Tradeable loyalty tokens built with Soroban smart contracts on the Stellar network.**  
> Turn airline miles and hotel points into real, peer-transferable digital assets.

---

## 📖 Project Description

LoyaltyChain is a **Soroban smart contract** that enables airlines and hotels to issue their loyalty points as **fully tradeable tokens** on the Stellar blockchain. Instead of siloed, non-transferable points locked inside a company's proprietary system, LoyaltyChain brings rewards onto a public, auditable ledger — giving members true ownership over the points they've earned.

Built on **Stellar's Soroban** (the EVM-compatible smart contract layer), the contract wraps Stellar's native SEP-41 token standard, meaning issued points are interoperable with the broader Stellar DeFi ecosystem: DEXes, wallets, liquidity pools, and atomic swaps.

Whether you're an airline, a hotel chain, or a travel platform, LoyaltyChain provides the foundation for a **next-generation loyalty programme** that members actually trust and value.

---

## 🚀 What It Does

LoyaltyChain manages the full lifecycle of loyalty points on-chain:

| Action | Who | Description |
|---|---|---|
| **Initialize** | Admin | Deploys and configures the programme (name, symbol, decimals) |
| **Award Points** | Issuer (airline/hotel) | Mints points to a member after a qualifying event (flight, stay) |
| **Redeem Points** | Member | Burns points in exchange for an upgrade, free night, or reward |
| **Transfer Points** | Member | Sends points peer-to-peer to any Stellar address — fully tradeable |
| **Check Balance** | Anyone | Reads a member's current points balance on-chain |
| **Update Issuer** | Admin | Rotates the issuer key without redeploying the contract |
| **Transfer Admin** | Admin | Hands off contract ownership securely |

Every action emits a **structured on-chain event**, enabling real-time indexing, analytics dashboards, and audit trails without relying on a centralised database.

---

## ✨ Features

### 🪙 Tradeable Loyalty Tokens
Points are **real SEP-41 tokens** — not database entries. Members can transfer them to friends, sell them on a DEX, or pool them with family members. True ownership, no walled garden.

### 🏗️ Built on Soroban (Stellar Smart Contracts)
Leverages Stellar's **Soroban** execution environment for:
- Near-zero transaction fees (fractions of a cent)
- 5-second finality
- Carbon-neutral consensus (Stellar Proof of Agreement)
- Native cross-border settlement

### 🔐 Role-Based Access Control
- **Admin** — deploys and governs the contract
- **Issuer** — the airline or hotel operator; the only address permitted to mint new points
- **Member** — any Stellar address that holds and controls their own points

Auth is enforced on-chain via `require_auth()` — no off-chain trust required.

### 🔄 Full Lifecycle Management
Award → Hold → Transfer → Redeem. The contract handles every stage:
- Minting on qualifying events (flights booked, nights stayed)
- Burning on redemption (upgrades, free nights, vouchers)
- Peer transfer between any two Stellar wallets

### 📣 On-Chain Events
Every state change (`award`, `redeem`, `transfer`, `new_issuer`, `new_admin`) emits a **structured event** that any indexer or frontend can subscribe to — making it trivial to build a real-time points dashboard or loyalty history feed.

### 🛡️ Safety Guards
- Re-initialisation protection (panics if called twice)
- Positive-amount enforcement on all mints, burns, and transfers
- Insufficient-balance check before redemptions
- All sensitive operations gated by Stellar's cryptographic auth

### 🔧 Configurable Per Programme
- Custom token name & symbol (e.g. `StellarAir Miles / SAM`, `GrandHotel Points / GHP`)
- Configurable decimal places (e.g. `2` for fractional points)
- Issuer address rotatable without contract redeployment

### 🧪 Unit Tested
Ships with an embedded Rust test suite using `soroban-sdk/testutils`:
- Happy-path initialisation
- Double-init panic protection
- (Extendable with award/redeem/transfer coverage)

---

## 🗂️ Project Structure

```
loyalty-points/
├── Cargo.toml                          # Workspace manifest
└── contracts/
    └── loyalty_token/
        ├── Cargo.toml                  # Contract crate config
        └── src/
            └── lib.rs                  # Smart contract source
```

---

## 🛠️ Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
cd loyalty-points
stellar contract build
```

The compiled `.wasm` binary lands at:
```
target/wasm32-unknown-unknown/release/loyalty_token.wasm
```

### Run Tests

```bash
cargo test --features testutils
```

### Deploy to Testnet

```bash
# Fund a testnet account
stellar keys generate --global alice --network testnet --fund

# Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/loyalty_token.wasm \
  --source alice \
  --network testnet
```

### Invoke — Award Points

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- \
  award_points \
  --token_address <TOKEN_ADDRESS> \
  --member <MEMBER_ADDRESS> \
  --amount 5000 \
  --memo "Flight JFK→LAX"
```

---

## 🔭 Roadmap

- [ ] Expiry / time-locked points (TTL-based storage)
- [ ] Tiered status tiers (Silver / Gold / Platinum) stored on-chain
- [ ] Cross-programme points swap via Stellar DEX
- [ ] Partner network: earn Hotel points on Airline bookings
- [ ] Frontend dashboard (React + Stellar Wallets Kit)
- [ ] Testnet demo deployment

---

## 📄 Licence

MIT — free to use, fork, and build upon.

---

> Built with ❤️ on [Stellar](https://stellar.org) · Powered by [Soroban](https://soroban.stellar.org)


wallet address:GASZSSNU2TRFZJIQNOE3UUXP2246DQ7V6JRJLIS7P43NTDDHYAZ2Q6PR


contract address:CA7C7UB4OGHC7JGO4BERH46O3YFMQ2KLZNA6EVVTUZ36C4DNXUKWOWID

<img width="1868" height="880" alt="image" src="https://github.com/user-attachments/assets/91904b05-566f-487b-914e-33b63a8b2680" />
