use crate::config::wallet_config::WalletConfig;
use crate::solana::transaction::SolanaTransaction;
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

    pub fn send_transaction(&self, matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        let sender_keypair = read_keypair_file(&self.config.keypair_path)
            .map_err(|_| "Failed to read keypair from file")?;
        let recipient_pubkey = self.get_pubkey_from_matches(matches)?;
        let amount = self.get_amount_from_matches(matches)?;

        SolanaTransaction::send_lamports(
            &self.config.rpc_url,
            &sender_keypair,
            &recipient_pubkey,
            amount,
        )
        .map_err(Into::into)
    }

    fn get_pubkey_from_matches(
        &self,
        matches: &ArgMatches,
    ) -> Result<Pubkey, Box<dyn std::error::Error>> {
        let recipient = matches
            .get_one::<String>("RECIPIENT")
            .ok_or("Recipient required")?;
        Pubkey::from_str(recipient).map_err(|_| "Invalid public key format".into())
    }

    fn get_amount_from_matches(
        &self,
        matches: &ArgMatches,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let amount_str = matches
            .get_one::<String>("AMOUNT")
            .ok_or("Amount required")?;
        amount_str
            .parse::<u64>()
            .map_err(|_| "Amount needs to be a number".into())
    }
}
