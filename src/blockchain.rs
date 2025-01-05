use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    pub block_count: usize,
}

impl Blockchain {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            accounts: HashMap::new(),
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            block_count: 0,
        }))
    }

    pub fn create_account(&mut self, id: String, balance: i64) -> String {
        if self.accounts.contains_key(&id) {
            return format!("Account '{}' already exists!", id);
        }
        self.accounts.insert(id.clone(), Account { id, balance });
        "Account created successfully!".to_string()
    }

    pub fn transfer(&mut self, from: String, to: String, amount: i64) -> String {
        if let Some(from_account) = self.accounts.get_mut(&from) {
            if from_account.balance < amount {
                return "Insufficient funds!".to_string();
            }
            from_account.balance -= amount;
            if let Some(to_account) = self.accounts.get_mut(&to) {
                to_account.balance += amount;
                self.pending_transactions.push(Transaction {
                    from: Some(from),
                    to,
                    amount,
                });
                return "Transaction successful!".to_string();
            }
        }
        "Transfer failed!".to_string()
    }

    pub fn get_balance(&self, id: &String) -> String {
        if let Some(account) = self.accounts.get(id) {
            format!("Balance of {}: {}", id, account.balance)
        } else {
            "Account not found!".to_string()
        }
    }

    pub fn mint_block(&mut self) {
        if self.pending_transactions.is_empty() {
            self.pending_transactions.push(Transaction {
                from: None,
                to: "No Transactions".to_string(),
                amount: current_timestamp() as i64,
            });
        }
        let block = Block {
            transactions: self.pending_transactions.drain(..).collect(),
            timestamp: current_timestamp(),
        };
        self.chain.push(block);
        self.block_count += 1;
        println!(
            "Minted block #{} with {} transactions.",
            self.block_count,
            self.chain.last().unwrap().transactions.len()
        );
    }

    pub fn start_auto_minting(blockchain: Arc<Mutex<Self>>) {
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(10));
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
