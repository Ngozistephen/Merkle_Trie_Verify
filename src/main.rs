
#![doc = include_str!("../README.md")]

use std::fs;
use sha2::{Digest, Sha256};  
use hex;                       


#[derive(Debug, Clone)]
struct Entry {
    address: String,
    amount: u64, 
}


struct MerkleTrie {
    tree: Vec<Vec<u8>>,
    leaf_count: usize,
}

/// Hash a single CSV entry into a leaf node.

fn hash_entry(entry: &Entry) -> Vec<u8> {
    let data = format!("{},{}", entry.address, entry.amount);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());  // feed bytes into the hasher
    hasher.finalize().to_vec()       
}

/// Hash two child hashes together to form a parent node.
fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(left);   // first child
    hasher.update(right);  // second child
    hasher.finalize().to_vec()
}


impl MerkleTrie {
    /// Build a Merkle Trie from a slice of entries.
    fn build(entries: &[Entry]) -> Self {
        // --- a) Hash each entry to create leaves ---
        let mut current_level: Vec<Vec<u8>> = entries
            .iter()
            .map(|e| hash_entry(e))
            .collect();

        let leaf_count = current_level.len();

        //  collect all levels here (leaves first, root last)
        let mut all_levels: Vec<Vec<Vec<u8>>> = Vec::new();
        all_levels.push(current_level.clone());

        //  Walk up the tree 
        while current_level.len() > 1 {
            // If odd number of nodes, duplicate the last one
            // so we always have pairs to hash.
            if current_level.len() % 2 != 0 {
                let last = current_level.last().unwrap().clone();
                current_level.push(last);
            }

            // Build the next level up
            let mut next_level: Vec<Vec<u8>> = Vec::new();
            for i in (0..current_level.len()).step_by(2) {
                // Pair node i with node i+1
                let parent = hash_pair(&current_level[i], &current_level[i + 1]);
                next_level.push(parent);
            }

            all_levels.push(next_level.clone());
            current_level = next_level;
        }

        // current_level now has exactly one element: the root
        let mut tree: Vec<Vec<u8>> = Vec::new();

        // Reverse so root (last level) comes first at index 0
        for level in all_levels.into_iter().rev() {
            for node in level {
                tree.push(node);
            }
        }

        MerkleTrie { tree, leaf_count }
    }

    
    /// Index 0 is always the root after building.
    fn root_hex(&self) -> String {
        hex::encode(&self.tree[0])
    }

   

    fn verify(&self, entry: &Entry) -> bool {
        let target_hash = hash_entry(entry);

        // Check every node in the tree
        self.tree.iter().any(|node| node == &target_hash)
    }
}


fn parse_csv(csv: &str) -> Vec<Entry> {
    let mut entries = Vec::new();

    for line in csv.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("address") {
            continue;
        }

        // Split on the comma
        let parts: Vec<&str> = line.splitn(2, ',').collect();
        if parts.len() != 2 {
            eprintln!("Skipping malformed line: {}", line);
            continue;
        }

        let address = parts[0].trim().to_string();

        let amount: u64 = parts[1].trim().parse().expect("Invalid amount");

        entries.push(Entry { address, amount });
    }

    entries
}


fn main() {

    let csv_data = match fs::read_to_string("addresses.csv") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
  
    let entries = parse_csv(&csv_data);
    println!("=== Parsed {} entries ===", entries.len());
    for (i, e) in entries.iter().enumerate() {
        println!("  [{}] address={} amount={}", i, e.address, e.amount);
    }

   
    let trie = MerkleTrie::build(&entries);
    println!("\n=== Merkle Trie built ===");
    println!("  Root hash : {}", trie.root_hex());
    println!("  Total nodes in tree: {}", trie.tree.len());

    // Print all nodes 
    println!("\n=== All tree nodes (root first) ===");
    for (i, node) in trie.tree.iter().enumerate() {
        println!("  node[{}] = {}", i, hex::encode(node));
    }

    
    println!("\n=== Verifiy ===");

    
    let test1 = Entry {
        address: "0x5C88C720556f41B96885CfCa84458a3492b4839d".to_string(),
        amount: 80,
    };
    println!(
        "  {:?} → amount={}  : {}",
        &test1.address,
        test1.amount,
        if trie.verify(&test1) { " VERIFIED (found in trie)" } else { " NOT FOUND" }
    );

  
    let test2 = Entry {
        address: "0x4B20993Bc481177ec7E8f571ceCaE8A9e22C02db".to_string(),
        amount: 99,
    };
    println!(
        "  {:?} → amount={}  : {}",
        &test2.address,
        test2.amount,
        if trie.verify(&test2) { "✅ VERIFIED (found in trie)" } else { "❌ NOT FOUND" }
    );

    
    let test3 = Entry {
        address: "0x5C88C720556f41B96885CfCa84458a3492b4839d".to_string(),
        amount: 9999,  
    };
    println!(
        "  {:?} → amount={} : {}",
        &test3.address,
        test3.amount,
        if trie.verify(&test3) { "VERIFIED (found in trie)" } else { " NOT FOUND (tampered amount detected!)" }
    );


    let test4 = Entry {
        address: "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
        amount: 50,
    };
    println!(
        "  {:?} → amount={}  : {}",
        &test4.address,
        test4.amount,
        if trie.verify(&test4) { " VERIFIED (found in trie)" } else { " NOT FOUND (unknown address)" }
    );

    println!("\nDone! The root hash summarises ALL four entries.");
    println!("If any single byte of any entry changes, the root changes too.");
}