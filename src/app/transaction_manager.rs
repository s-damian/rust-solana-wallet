use crate::config::wallet_config::WalletConfig;
use crate::solana::transaction::SolanaTransaction;
use clap::ArgMatches;
use solana_sdk::signature::Signer;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use std::env;
use std::str::FromStr;

pub struct TransactionManager {
    config: WalletConfig,
}

impl TransactionManager {
    /// Crée une nouvelle instance de TransactionManager.
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    /// Envoie des lamports (unité de SOL) à une adresse spécifiée en utilisant la clé privée du wallet.
    ///
    /// # Arguments:
    /// * matches - Arguments de ligne de commande traités, fournissant le destinataire et le montant.
    ///
    /// # Returns:
    /// * Ok(()) - Si la transaction est envoyée avec succès.
    /// * Err(e) - Si une erreur se produit lors de la lecture de la clé, la conversion des arguments, ou l'envoi de la transaction.
    pub fn send_transaction(&self, matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        // Lecture du fichier contenant la clé privée du sender.
        let sender_keypair = read_keypair_file(&self.config.keypair_path)
            .map_err(|_| "Failed to read keypair from file")?;

        // Extraction et validation de l'adresse publique du destinataire.
        let recipient_pubkey = self.get_pubkey_from_matches(matches)?;

        // Extraction et validation du montant à envoyer.
        let amount = self.get_amount_from_matches(matches)?;

        // Vérifier si nous sommes en mode test.
        if env::var("TEST_MODE").unwrap_or_default() == "true" {
            // Simulation de la transaction.
            println!(
                "Simulating transaction: {} lamports from {} to {}",
                amount,
                sender_keypair.pubkey(),
                recipient_pubkey
            );
            println!("Transaction sent successfully!");
            Ok(())
        } else {
            // Envoi réel de la transaction via le réseau Solana.
            SolanaTransaction::send_lamports(
                &self.config.rpc_url,
                &sender_keypair,
                &recipient_pubkey,
                amount,
            )
            .map_err(Into::into)
        }
    }

    /// Extrait l'adresse publique du destinataire à partir des arguments de ligne de commande.
    ///
    /// Arguments:
    /// * matches - Arguments de ligne de commande pour l'opération de transaction.
    ///
    /// # Returns:
    /// * Result<Pubkey, Box<dyn std::error::Error>> - Sui est l'adresse publique du destinataire si l'extraction est réussie.
    fn get_pubkey_from_matches(
        &self,
        matches: &ArgMatches,
    ) -> Result<Pubkey, Box<dyn std::error::Error>> {
        let recipient = matches
            .get_one::<String>("RECIPIENT")
            .ok_or("Recipient required")?;
        Pubkey::from_str(recipient).map_err(|_| "Invalid public key format".into())
    }

    /// Extrait le montant des lamports à envoyer à partir des arguments de ligne de commande.
    ///
    /// Arguments:
    /// * matches - Arguments de ligne de commande pour l'opération de transaction.
    ///
    /// # Returns:
    /// * Result<u64, Box<dyn std::error::Error>> - Qui est le montant des lamports si l'extraction est réussie.
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
