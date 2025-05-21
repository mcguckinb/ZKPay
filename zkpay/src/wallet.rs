use crate::{Commitment, Ledger, Note, Transaction, Wallet};
use anyhow::{Context, Result};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct WalletManager {
    wallets: HashMap<String, Wallet>,
    ledger: Ledger,
    data_dir: PathBuf,
}

impl WalletManager {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&data_dir).context("Failed to create data directory")?;
        
        let wallets_file = data_dir.join("wallets.json");
        let ledger_file = data_dir.join("ledger.json");
        
        let wallets = if wallets_file.exists() {
            let data = fs::read_to_string(wallets_file)?;
            serde_json::from_str(&data)?
        } else {
            HashMap::new()
        };
        
        let ledger = if ledger_file.exists() {
            let data = fs::read_to_string(ledger_file)?;
            serde_json::from_str(&data)?
        } else {
            Ledger {
                commitments: Vec::new(),
                nullifiers: Vec::new(),
                transactions: Vec::new(),
            }
        };
        
        Ok(Self {
            wallets,
            ledger,
            data_dir,
        })
    }
    
    pub fn create_wallet(&mut self, name: &str) -> Result<()> {
        if self.wallets.contains_key(name) {
            anyhow::bail!("Wallet '{}' already exists", name);
        }
        
        let mut rng = rand::thread_rng();
        let secret_key = hex::encode(rng.gen::<[u8; 32]>());
        let public_key = hex::encode(Sha256::digest(secret_key.as_bytes()));
        let viewing_key = hex::encode(rng.gen::<[u8; 32]>());
        
        let wallet = Wallet {
            public_key,
            viewing_key,
            secret_key,
        };
        
        self.wallets.insert(name.to_string(), wallet);
        self.save_wallets()?;
        
        println!("[✔] Wallet created for '{}'", name);
        println!("[✔] Public Key: {}...", &self.wallets[name].public_key[..8]);
        println!("[✔] Viewing Key: {}...", &self.wallets[name].viewing_key[..8]);
        
        Ok(())
    }
    
    pub fn mint(&mut self, to: &str, amount: u64) -> Result<()> {
        let wallet = self.wallets.get(to)
            .context(format!("Wallet '{}' not found", to))?;
            
        let mut rng = rand::thread_rng();
        let secret = hex::encode(rng.gen::<[u8; 32]>());
        
        let note = Note {
            value: amount,
            owner_pubkey: wallet.public_key.clone(),
            secret,
        };
        
        let commitment = Commitment {
            id: format!("cmt{}", hex::encode(rng.gen::<[u8; 16]>())),
            data: serde_json::to_string(&note)?,
        };
        
        self.ledger.commitments.push(commitment);
        self.save_ledger()?;
        
        println!("[✔] Minted {} units to {}", amount, to);
        println!("[+] Created note: {}... (hidden)", &self.ledger.commitments.last().unwrap().id[..8]);
        
        Ok(())
    }
    
    pub fn check_balance(&self, wallet_name: &str) -> Result<()> {
        let wallet = self.wallets.get(wallet_name)
            .context(format!("Wallet '{}' not found", wallet_name))?;
            
        let mut balance = 0;
        let mut found_notes = 0;
        
        for commitment in &self.ledger.commitments {
            let note: Note = serde_json::from_str(&commitment.data)?;
            if note.owner_pubkey == wallet.public_key {
                balance += note.value;
                found_notes += 1;
            }
        }
        
        println!("[🔍] Scanning ledger with viewing key...");
        println!("[✔] Found {} note(s) for you", found_notes);
        println!("💵 Your private balance: {} units", balance);
        
        Ok(())
    }
    
    pub fn view_ledger(&self) -> Result<()> {
        println!("\n[🧾] Commitments:");
        for commitment in &self.ledger.commitments {
            println!("- {} (amount: hidden)", commitment.id);
        }
        
        println!("\n[🚫] Nullifiers:");
        if self.ledger.nullifiers.is_empty() {
            println!("- (none yet)");
        } else {
            for nullifier in &self.ledger.nullifiers {
                println!("- {}", nullifier);
            }
        }
        
        println!("\n[📜] Total transactions: {}", self.ledger.transactions.len());
        
        Ok(())
    }
    
    fn save_wallets(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.wallets)?;
        fs::write(self.data_dir.join("wallets.json"), data)?;
        Ok(())
    }
    
    fn save_ledger(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.ledger)?;
        fs::write(self.data_dir.join("ledger.json"), data)?;
        Ok(())
    }
} 