use crate::config::wallet_config::WalletConfig;
use clap::ArgMatches;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use std::str::FromStr;

pub struct TransactionManager {
    config: WalletConfig,
}

impl TransactionManager {
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    pub fn send_transaction(&self, matches: &ArgMatches) {
        let wallet_config = &self.config;

        let recipient = matches
            .get_one::<String>("RECIPIENT")
            .expect("Recipient required");
        let amount = matches
            .get_one::<String>("AMOUNT")
            .expect("Amount required")
            .parse::<u64>()
            .expect("Amount needs to be a number");

        let recipient_pubkey = Pubkey::from_str(recipient).expect("Invalid public key format");
        let sender_keypair_path = &wallet_config.keypair_path;

        let sender_keypair =
            read_keypair_file(sender_keypair_path).expect("Failed to read keypair from file");

        match crate::solana::transaction::send_lamports(
            &wallet_config.rpc_url,
            &sender_keypair,
            &recipient_pubkey,
            amount,
        ) {
            Ok(_) => println!("Transaction sent successfully!"),
            Err(e) => println!("Failed to send transaction: {}", e),
        }
    }
}
