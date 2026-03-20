
# 🌳 Merkle Trie in Rust

A simple implementation of a **Merkle Trie (Merkle Tree)** in Rust that:
- Reads address + amount data from a CSV file
- Hashes entries using SHA-256
- Builds a Merkle Tree
- Generates a root hash
- Verifies whether an entry exists in the tree

---

## 🚀 Features

- ✅ CSV file input (`addresses.csv`)
- ✅ SHA-256 hashing (`sha2`)
- ✅ Deterministic Merkle root generation
- ✅ Entry verification
- ✅ Handles odd number of nodes (auto-duplicates last node)

---

## 📁 Project Structure
.
├── src/
│ └── main.rs
├── addresses.csv
├── Cargo.toml
└── README.md



---

## Installation
Make sure you have Rust installed:

rustc --version
cargo --version


Clone the project:
git clone https://github.com/your-username/your-repo.git
cd your-repo


1. Parse CSV

Reads and converts CSV rows into:

Entry {
  address: String,
  amount: u64
}

2. Hash Leaves

Each entry is hashed:

hash = SHA256("address,amount")
