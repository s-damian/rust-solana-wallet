use crate::config::wallet_config::WalletConfig;
use crate::solana::address::SolanaAddress;
use solana_sdk::signer::Signer;

pub struct KeypairManager {
    config: WalletConfig,
}

impl KeypairManager {
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    /// Récupère et affiche la clé publique à partir d'une paire de clés stockée localement dans un fichier.
    pub fn get_pubkey_from_keypair_file(&self) {
        // Obtient le chemin d'accès au fichier où la paire de clés est stockée.
        let keypair_path = &self.config.keypair_path;

        // Tente de lire la paire de clés à partir du fichier spécifié (en utilisant le chemin obtenu précédemment).
        // La fonction "read_keypair_from_file" gère le chargement et la désérialisation de la paire de clés à partir du fichier.
        // Ok: En cas de succès, extrait la clé publique de la paire de clés et l'affiche.
        // Err: En cas d'échec, affiche une erreur indiquant que la lecture a échoué.
        match SolanaAddress::read_keypair_from_file(keypair_path) {
            Ok(keypair) => println!("Clé publique : {}", keypair.pubkey()),
            Err(e) => println!("Failed to read key pair from file: {}", e),
        }
    }
}
