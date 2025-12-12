# Blockchain Simulator

A simple yet functional blockchain implementation in Rust that demonstrates core blockchain concepts including block mining, proof-of-work, and transaction chains.

## 🎯 Overview

This project implements a basic blockchain simulator that allows you to:
- Create a genesis block
- Mine new blocks with proof-of-work
- Simulate transactions between traders
- Visualize the mining process
- Display the complete blockchain

## 🚀 Features

### Core Blockchain Functionality
- **Block Structure**: Each block contains an index, previous hash, timestamp, data, nonce, and hash
- **Proof-of-Work Mining**: Implements a configurable difficulty level for mining blocks
- **SHA-256 Hashing**: Uses cryptographic hashing to secure the blockchain
- **Chain Validation**: Ensures each block is properly linked to the previous block
- **Genesis Block**: Automatically creates the first block in the chain

### Interactive Simulation
- **Custom Miner Name**: Enter your name as the blockchain miner
- **Automated Transactions**: Simulates 10 transactions between predefined traders
- **Mining Visualization**: Shows real-time mining progress and calculated hashes
- **Blockchain Display**: Prints the complete blockchain with timestamps
- **NovaCoin Trading Statistics**: Calculates total NovaCoin traded (10 per block)
- **Simulation Metrics**: Displays simulation end time and completion status

## 📋 Prerequisites

- **Rust**: Version 1.70 or higher
- **Cargo**: Rust's package manager (comes with Rust)

## 🛠️ Installation

1. **Clone or navigate to the project directory:**
   ```bash
   cd /Users/subh/Desktop/code-playground/blockchain
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run the blockchain simulator:**
   ```bash
   cargo run
   ```

## 📦 Dependencies

The project uses the following Rust crates:

- **sha2** (v0.10.9): For SHA-256 cryptographic hashing
- **chrono** (v0.4.42): For timestamp handling and formatting

These dependencies are automatically downloaded and installed by Cargo.

## 🎮 Usage

When you run the program, you'll be prompted to enter your miner name:

```
Welcome to the Blockchain stimulator!
Enter your miner name: 
```

After entering your name, the simulator will:
1. Create a genesis block
2. Mine 10 new blocks, each containing a transaction
3. Display mining progress for each block
4. Show the total number of blocks
5. Print the complete blockchain with timestamps

### Example Output

```
Welcome to the Blockchain stimulator!
Enter your miner name: 
Alice

 Let's start mining and stimulating transactions!
Mining Block 1 ...
Block mined : 1
Transaction added : Send Alice to Neha

Mining Block 2 ...
Block mined : 2
Transaction added : Send Neha to Subh

...

Total number of blocks : 11

Blockchain contents:
Block 0 : Genesis Block at 2025-12-12 12:47:50
Block 1 : Send Alice to Neha at 2025-12-12 12:47:53
Block 2 : Send Neha to Subh at 2025-12-12 12:47:56
...

💰 Total NovaCoin traded: 110 NovaCoin
⏰ Simulation ended at: 2025-12-12 12:48:15 UTC
🎉 Congrats! Mining operation completed successfully!
```

## 🔧 Technical Details

### Block Structure

Each block in the blockchain contains:

```rust
struct Block {
    index: u32,           // Position in the blockchain
    previous_hash: String, // Hash of the previous block
    timestamp: u64,        // Unix timestamp
    data: String,          // Transaction data
    nonce: u64,            // Proof-of-work nonce
    hash: String,          // Block's hash
}
```

### Mining Algorithm

The mining process uses a proof-of-work algorithm:

1. Calculate the hash of the block (index + previous_hash + timestamp + data + nonce)
2. Check if the hash starts with the required number of zeros (difficulty)
3. If not, increment the nonce and try again
4. Repeat until a valid hash is found

**Difficulty Level**: Currently set to `2`, meaning the hash must start with "00"

### Hash Calculation

The hash is calculated using SHA-256:

```rust
fn calculate_hash(&self) -> String {
    let data = format!(
        "{}{}{}{}{}",
        self.index, self.previous_hash, self.timestamp, self.data, self.nonce
    );
    
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    
    format!("{:x}", result)
}
```

### Blockchain Structure

The blockchain is a vector of blocks:

```rust
struct Blockchain {
    chain: Vec<Block>,
}
```

## 🎓 Learning Concepts

This project demonstrates several important blockchain concepts:

1. **Immutability**: Each block contains the hash of the previous block, making it impossible to alter past blocks without invalidating the entire chain

2. **Proof-of-Work**: Mining requires computational effort to find a valid hash, making it expensive to create fraudulent blocks

3. **Cryptographic Hashing**: SHA-256 ensures that any change to block data results in a completely different hash

4. **Chain Integrity**: The blockchain maintains integrity through the linked hash structure

5. **Timestamps**: Each block is timestamped, creating a chronological record of transactions

## 🔍 Code Structure

```
blockchain/
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock          # Locked dependency versions
├── errors.md           # Documentation of fixed syntax errors
├── README.md           # This file
└── src/
    └── main.rs         # Main blockchain implementation
```

## 🐛 Troubleshooting

### Build Errors

If you encounter build errors, ensure:
- You're using Rust edition 2021 or later
- All dependencies are properly installed (`cargo update`)
- You have the latest stable version of Rust (`rustup update`)

### Runtime Issues

If the mining process seems stuck:
- The difficulty is set to 2, which should mine blocks quickly
- If mining takes too long, you can reduce the `DIFFICULTY` constant in `main.rs`

## 🎨 Customization

### Adjusting Mining Difficulty

Change the `DIFFICULTY` constant at the top of `main.rs`:

```rust
const DIFFICULTY: usize = 2;  // Increase for harder mining, decrease for easier
```

### Adding More Traders

Modify the `trader_names` vector in the `main()` function:

```rust
let trader_names = vec!["Neha", "Subh", "Your Name Here", ...];
```

### Changing Transaction Format

Modify the transaction format in the loop:

```rust
let transaction = format!("Your custom format: {} -> {}", sender, recipient);
```

### Adjusting NovaCoin Rewards

Change the `novacoin_per_block` value:

```rust
let novacoin_per_block: usize = 10;  // Change to your desired reward amount
```

## 📝 License

This is an educational project created for learning blockchain concepts.

## 🤝 Contributing

Feel free to fork this project and experiment with:
- Adding transaction validation
- Implementing a merkle tree
- Adding wallet functionality
- Creating a network layer for distributed nodes
- Implementing smart contracts

## 📚 Further Reading

To learn more about blockchain technology:
- [Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf)
- [Ethereum Documentation](https://ethereum.org/en/developers/docs/)
- [Rust Book](https://doc.rust-lang.org/book/)

## ✨ Acknowledgments

This project uses:
- The Rust programming language
- SHA-256 cryptographic hashing
- Chrono for time handling

---

**Happy Mining! ⛏️**
