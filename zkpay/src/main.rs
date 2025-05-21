mod wallet;

use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use wallet::WalletManager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new wallet
    CreateWallet {
        /// Name of the wallet
        name: String,
    },
    /// Mint tokens to a wallet
    Mint {
        /// Recipient wallet name
        #[arg(short, long)]
        to: String,
        /// Amount to mint
        #[arg(short, long)]
        amount: u64,
    },
    /// Send tokens to another wallet
    Send {
        /// Sender wallet name
        #[arg(short, long)]
        from: String,
        /// Recipient wallet name
        #[arg(short, long)]
        to: String,
        /// Amount to send
        #[arg(short, long)]
        amount: u64,
    },
    /// Check wallet balance
    CheckBalance {
        /// Wallet name
        #[arg(short, long)]
        wallet: String,
    },
    /// View the ledger
    Ledger,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wallet {
    public_key: String,
    viewing_key: String,
    secret_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    value: u64,
    owner_pubkey: String,
    secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Commitment {
    id: String,
    data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ledger {
    commitments: Vec<Commitment>,
    nullifiers: Vec<String>,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    proof: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_dir = PathBuf::from(".zkpay");
    let mut wallet_manager = WalletManager::new(data_dir)?;

    match cli.command {
        Commands::CreateWallet { name } => {
            wallet_manager.create_wallet(&name)?;
        }
        Commands::Mint { to, amount } => {
            wallet_manager.mint(&to, amount)?;
        }
        Commands::Send { from, to, amount } => {
            println!("[!] Sending not implemented yet - ZK proofs coming soon!");
            println!("Would send {} units from {} to {}", amount, from, to);
        }
        Commands::CheckBalance { wallet } => {
            wallet_manager.check_balance(&wallet)?;
        }
        Commands::Ledger => {
            wallet_manager.view_ledger()?;
        }
    }

    Ok(())
}
