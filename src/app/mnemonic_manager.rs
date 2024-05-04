use crate::bip::passphrase::BipPassphrase;
use crate::bip::seed::BipSeed;
use crate::config::wallet_config::WalletConfig;
use crate::solana::address::SolanaAddress;
use bip39::Mnemonic;
use solana_sdk::signer::Signer;

pub struct MnemonicManager {
    config: WalletConfig,
}

impl MnemonicManager {
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    /// Traite une mnémonique pour générer et afficher la clé publique correspondante, en prenant en compte les dérivations spécifiées.
    /// Cette fonction sert de point central pour la création de clés Solana à partir d'une phrase mnémonique.
    pub fn process_mnemonic(&self, mnemonic: &Mnemonic) {
        // Demande à l'utilisateur d'entrer une passphrase optionnelle qui sera utilisée lors de la génération de la seed.
        // (laisser vide pour ne pas utiliser de passphrase)
        let passphrase = BipPassphrase::prompt_for_passphrase();

        // Génère une seed en format hexadécimal à partir de la phrase mnémonique et de la passphrase.
        // Cette seed de portefeuille HD (Hiérarchiquement Déterministe) permettra de produire une suite cohérente de clés dérivées.
        let seed = BipSeed::generate_seed(mnemonic, &passphrase);
        println!("Seed: {:X}", seed);

        // Convertit la seed en un tableau de bytes bruts, qui servira de base pour la génération de clés dérivées.
        let seed_bytes = BipSeed::get_seed_bytes(&seed);
        self.handle_key_derivation(seed_bytes);
    }

    /// Gère la dérivation de clés et leur enregistrement.
    fn handle_key_derivation(&self, seed_bytes: &[u8]) {
        // Récupère le nombre de dérivations souhaitées (est de 1 par défaut).
        let nb_derivations = self.config.nb_derivations;
        // Gère les dérivations multiples pour générer plusieurs paires de clés.
        for index in 0..nb_derivations {
            self.derive_and_store_keypair(seed_bytes, index);
        }
    }

    /// Dérive et stocke une paire de clés.
    fn derive_and_store_keypair(&self, seed_bytes: &[u8], index: usize) {
        // Dériver la seed pour chaque index spécifié (sauf pour l'index 0 qui utilise la seed originale).
        match BipSeed::derive_seed_bytes(seed_bytes, index) {
            Ok(derived_seed_bytes) => {
                // Génerer une paire de clés (clé publique et clé privée) à partir de la seed en bytes.
                // Puis écrire cette paire de clés dans un fichier JSON.
                let keypair = SolanaAddress::generate_keypair(&derived_seed_bytes);

                let keypair_path = self.derive_keypair_path(index);
                SolanaAddress::write_keypair(&keypair, &keypair_path);

                // Affiche la clé publique (qui dans le cas de Solana, est également utilisée comme adresse publique du wallet).
                println!("Solana Public Key {}: {}", index, keypair.pubkey());
            }
            Err(e) => println!("Error deriving seed bytes: {}", e),
        }
    }

    /// Construit le chemin du fichier de la paire de clés en fonction de l'index de dérivation.
    fn derive_keypair_path(&self, index: usize) -> String {
        if index == 0 {
            self.config.keypair_path.clone()
        } else {
            format!("{}/keypair-{}.json", self.config.keypair_dir, index)
        }
    }
}
