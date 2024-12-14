use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a blockchain account with an `id` and a `balance`.
#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,    // Unique identifier for the account
    pub balance: i64,  // Account balance
}

/// Represents a transaction in the blockchain.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: Option<String>, // ID of the sender account, or `None` for system transactions
    pub to: String,           // ID of the recipient account
    pub amount: i64,          // Amount to transfer
}

/// Represents a single block in the blockchain.
#[derive(Debug, Clone)]
pub struct Block {
    pub transactions: Vec<Transaction>, // List of transactions included in the block
    pub timestamp: u64,                 // Unix timestamp of when the block was created
}

/// Represents the entire blockchain.
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub accounts: HashMap<String, Account>,    // Mapping of account IDs to their data
    pub chain: Vec<Block>,                     // List of blocks in the blockchain
    pub pending_transactions: Vec<Transaction>,// Transactions waiting to be included in a block
    pub block_count: usize,                    // Number of blocks minted
}

impl Blockchain {
    /// Creates a new, empty blockchain.
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            block_count: 0,
        }
    }

    /// Creates a new account with the given `id` and `balance`.
    /// Returns a success message or an error message if the account already exists.
    pub fn create_account(&mut self, id: String, balance: i64) -> String {
        if self.accounts.contains_key(&id) {
            return format!("Account with ID '{}' already exists!", id);
        }
        self.accounts.insert(id.clone(), Account { id, balance });
        String::from("Account created successfully!")
    }

    /// Transfers `amount` from the `from` account to the `to` account.
    /// Returns a success message or an error message if the transfer fails.
    pub fn transfer(&mut self, from: String, to: String, amount: i64) -> String {
        // Ensure both accounts exist
        if !self.accounts.contains_key(&from) || !self.accounts.contains_key(&to) {
            return String::from("One or both accounts do not exist!");
        }

        // Attempt to perform the transfer
        if let Some(mut from_acc) = self.accounts.remove(&from) {
            if from_acc.balance < amount {
                // Re-add the account if insufficient funds
                self.accounts.insert(from.clone(), from_acc);
                return String::from("Insufficient funds!");
            }

            // Deduct from sender's account and credit to recipient's account
            from_acc.balance -= amount;
            if let Some(to_acc) = self.accounts.get_mut(&to) {
                to_acc.balance += amount;
            }
            self.accounts.insert(from.clone(), from_acc);

            // Add the transaction to the pending list
            self.pending_transactions.push(Transaction {
                from: Some(from),
                to,
                amount,
            });

            String::from("Transaction queued successfully!")
        } else {
            String::from("Error processing the transaction!")
        }
    }

    /// Retrieves the balance of the account with the given `id`.
    /// Returns the balance as a string or an error message if the account is not found.
    pub fn get_balance(&self, id: &String) -> String {
        if let Some(account) = self.accounts.get(id) {
            format!("Balance of {}: {}", id, account.balance)
        } else {
            String::from("Account not found!")
        }
    }

    /// Mints a new block containing all pending transactions.
    /// Resets the list of pending transactions after minting the block.
    pub fn mint_block(&mut self) {
        if !self.pending_transactions.is_empty() {
            let block = Block {
                transactions: self.pending_transactions.drain(..).collect(), // Collect all pending transactions
                timestamp: current_timestamp(), // Generate the current timestamp
            };
            self.chain.push(block);          // Add the new block to the chain
            self.block_count += 1;           // Increment the block count
        }
    }

    /// Starts an automated process to mint a new block every 10 seconds.
    /// Runs in a separate thread.
    pub fn start_auto_minting(blockchain: Arc<Mutex<Self>>) {
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(10)); // Wait for 10 seconds
                let mut chain = blockchain.lock().unwrap(); // Safely access the blockchain
                chain.mint_block();                         // Mint a new block
            }
        });
    }
}

/// Retrieves the current Unix timestamp in seconds.
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
