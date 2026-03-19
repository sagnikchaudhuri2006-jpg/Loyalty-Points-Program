# ⭐ Loyalty Points Program (Soroban Smart Contract)

## 📌 Project Description

The **Loyalty Points Program** is a decentralized application built using Soroban smart contracts on the Stellar network. It enables businesses, platforms, or communities to reward users with loyalty points that are securely stored and managed on-chain.

By leveraging blockchain technology, this system ensures transparency, immutability, and trust—eliminating the need for centralized loyalty management systems.

---

## 🚀 What It Does

This smart contract provides a simple and efficient way to manage loyalty points:

* Businesses (admin) can assign points to users
* Users can securely redeem their points
* Anyone can check a user’s point balance
* All transactions are recorded on-chain for transparency

---

## ✨ Features

* 🔐 **Admin Authorization**
  Only the contract admin can distribute loyalty points

* 👤 **User-Based Accounting**
  Each user has an individual on-chain points balance

* 💸 **Secure Redemption**
  Users can redeem points with proper balance validation

* 📊 **On-Chain Transparency**
  All balances and updates are verifiable on the blockchain

* ⚡ **Lightweight & Efficient**
  Built using Soroban SDK for high performance

* 🧩 **Extensible Design**
  Easily expandable for advanced features like rewards, tiers, or tokenization

---

## 🛠️ Smart Contract Functions

| Function        | Description                                    |
| --------------- | ---------------------------------------------- |
| `init`          | Initializes the contract with an admin address |
| `add_points`    | Adds loyalty points to a user (admin only)     |
| `redeem_points` | Allows a user to redeem their points           |
| `get_points`    | Returns the current balance of a user          |

---

## ⚙️ How It Works

1. The contract is deployed to the Stellar Soroban network
2. An admin account initializes the contract
3. The admin distributes loyalty points to users
4. Users redeem points when needed
5. Balances are stored and updated on-chain

---

## 🧪 Example Use Cases

* 🛍️ E-commerce reward systems
* ☕ Café / restaurant loyalty programs
* 🎮 Gaming reward points
* 🎟️ Event participation incentives
* 🌐 Web3 community engagement rewards

---

## 🔗 Deployed Smart Contract Link

https://stellar.expert/explorer/testnet/contract/CDSTFJEALE34N7RDETLRXMWLE2MK655GYIX4PBNYSL2CKKDXERB4HYXU

<img width="1919" height="966" alt="Screenshot 2026-03-19 143402" src="https://github.com/user-attachments/assets/33fbf527-dbfc-4b03-a3e5-e0042c0fcb1b" />

---

🌐 Deployed Smart Contract

Contract Address: CDSTFJEALE34N7RDETLRXMWLE2MK655GYIX4PBNYSL2CKKDXERB4HYXU

Network: Stellar Testnet / Mainnet (choose one)

Explorer Link: https://stellar.expert/explorer/testnet/contract/CDSTFJEALE34N7RDETLRXMWLE2MK655GYIX4PBNYSL2CKKDXERB4HYXU

## 🧰 Tech Stack

* **Rust** (Smart Contract Language)
* **Soroban SDK**
* **Stellar Network (Testnet/Mainnet)**

---

## 🚧 Future Improvements

* 🎁 Reward redemption catalog
* 🪙 Tokenized loyalty points (SAC token integration)
* 🧾 Transaction history & event logs
* 🏆 Tier-based reward levels
* 🌐 Frontend dApp (React + Wallet integration)
* ⏳ Expirable points system

---

## 📦 How to Deploy

```bash
# Build contract
cargo build --target wasm32v1-none --release

# Deploy contract
stellar contract deploy --wasm target/wasm32v1-none/release/your_contract.wasm --source-account alice --network testnet --alias loyalty_program
```

---

## 👨‍💻 Author

Name: SAGNIK CHAUDHURI

Email: sagnikchaudhuri2006@gmail.com

---

## ⭐ Acknowledgements

* Stellar Development Foundation
* Soroban Documentation
