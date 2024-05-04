use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

pub struct SolanaTransaction {}

impl SolanaTransaction {
    /// Envoyer des SOL à une adresse spécifique.
    ///
    /// # Arguments:
    /// * rpc_url - L'URL du point de terminaison RPC pour se connecter au réseau Solana.
    /// * sender_keypair - La paire de clés du compte expéditeur utilisée pour signer la transaction.
    /// * recipient_pubkey - La clé publique du destinataire qui recevra les SOL.
    /// * lamports - Le montant en lamports à envoyer (1 SOL = 1_000_000_000 lamports).
    ///
    /// # Returns:
    /// Retourne un "Result" qui est Ok si la transaction est réussie, ou une erreur en cas d'échec.
    pub fn send_lamports(
        rpc_url: &str,
        sender_keypair: &Keypair,
        recipient_pubkey: &Pubkey,
        lamports: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Crée un client pour interagir avec le réseau Solana via RPC.
        let client = RpcClient::new(String::from(rpc_url));

        // Récupère le dernier blockhash utilisé comme référence de frais pour la transaction.
        let recent_blockhash = client.get_latest_blockhash()?;

        // Crée une instruction pour transférer des lamports du compte expéditeur au destinataire.
        let instruction =
            system_instruction::transfer(&sender_keypair.pubkey(), recipient_pubkey, lamports);

        // Emballe l'instruction dans un message, en spécifiant le compte expéditeur comme compte de frais.
        let message = Message::new(&[instruction], Some(&sender_keypair.pubkey()));

        // Crée la transaction en utilisant la paire de clés de l'expéditeur, le message et le blockhash récent.
        // La transaction est automatiquement signée par la paire de clés de l'expéditeur lors de la création.
        let transaction = Transaction::new(&[sender_keypair], message, recent_blockhash);

        // Envoie la transaction signée au réseau Solana et attend la confirmation.
        client.send_and_confirm_transaction(&transaction)?;

        // Retourne Ok si tout s'est bien passé.
        Ok(())
    }
}
