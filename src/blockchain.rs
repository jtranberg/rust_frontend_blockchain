use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub balance: i64,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: Option<String>,
    pub to: String,
    pub amount: i64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub accounts: HashMap<String, Account>,
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub block_count: usize, // Track the number of blocks minted
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            block_count: 0,
        }
    }

    pub fn create_account(&mut self, id: String, balance: i64) -> String {
        if self.accounts.contains_key(&id) {
            return format!("Account with ID '{}' already exists!", id);
        }
        self.accounts.insert(id.clone(), Account { id, balance });
        String::from("Account created successfully!")
    }

    pub fn transfer(&mut self, from: String, to: String, amount: i64) -> String {
        if !self.accounts.contains_key(&from) || !self.accounts.contains_key(&to) {
            return String::from("One or both accounts do not exist!");
        }

        if let Some(mut from_acc) = self.accounts.remove(&from) {
            if from_acc.balance < amount {
                self.accounts.insert(from.clone(), from_acc);
                return String::from("Insufficient funds!");
            }

            from_acc.balance -= amount;
            if let Some(to_acc) = self.accounts.get_mut(&to) {
                to_acc.balance += amount;
            }
            self.accounts.insert(from.clone(), from_acc);

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

    pub fn get_balance(&self, id: &String) -> String {
        if let Some(account) = self.accounts.get(id) {
            format!("Balance of {}: {}", id, account.balance)
        } else {
            String::from("Account not found!")
        }
    }

    pub fn mint_block(&mut self) {
        if !self.pending_transactions.is_empty() {
            let block = Block {
                transactions: self.pending_transactions.drain(..).collect(),
                timestamp: current_timestamp(),
            };
            self.chain.push(block);
            self.block_count += 1; // Increment block count
        }
    }

    pub fn start_auto_minting(blockchain: Arc<Mutex<Self>>) {
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(10));
                let mut chain = blockchain.lock().unwrap();
                chain.mint_block();
            }
        });
    }
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
