// Import the necessary dependencies
use core::hash;
use sha2::{Digest, Sha256};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use std::thread;
use std::time::Duration;

// Define the difficulty of the mining
const DIFFICULTY: usize = 2;

// Define the structure of a block in the blockchain
// struct and impl

struct Block {
    index: u32,
    previous_hash: String,
    timestamp: u64,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    // calculate the hash of the block
    fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index, self.previous_hash, self.timestamp, self.data, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let hash_str = format!("{:x}", result);
        hash_str
    }

    fn mine_block_with_visulisation(&mut self) {
        let mut iterations = 0;
        loop {
            self.hash = self.calculate_hash();

            iterations += 1;
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                println!("Block mined : {}", self.index);
                break;
            }

            if iterations > 100 {
                println!("Mining in progress...");
                thread::sleep(Duration::from_millis(3000));
                println!("calculated_hash: {}", self.hash);
                break;
            }
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(f, "Block {} : {} at {}", self.index, self.data, datetime)
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }
}
