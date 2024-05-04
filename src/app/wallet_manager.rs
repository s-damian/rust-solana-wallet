use crate::app::mnemonic_manager::MnemonicManager;
use crate::bip::mnemonic::BipMnemonic;
use crate::config::wallet_config::WalletConfig;
use crate::solana::balance::SolanaBalance;

pub struct WalletManager {
    config: WalletConfig,
}

impl WalletManager {
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    /// Génère une phrase mnémonique aléatoire de 12 mots et affiche la clé publique Solana correspondante.
    /// Cette fonction est typiquement utilisée pour la création initiale d'un portefeuille.
    pub fn generate_and_print_random_mnemonic(&self) {
        let mnemonic_manager = MnemonicManager::new(self.config.clone());

        // Créer une nouvelle phrase mnémonique générée aléatoirement (en suivant le standard BIP39).
        // Il s'agit d'une mnémonique de 12 mots, ce qui est un standard commun pour de nombreux portefeuilles.
        let mnemonic = BipMnemonic::generate_mnemonic();

        // Convertit la mnémonique en une chaîne de caractères (pour pouvoir l'afficher et l'utiliser ultérieurement).
        // Cette phrase est utilisée pour générer une seed et peut être utilisée pour la récupération d'un portefeuille.
        let phrase = BipMnemonic::get_mnemonic_to_str(&mnemonic);

        println!("BIP39 Mnemonic (random phrase): {}", phrase);

        mnemonic_manager.process_mnemonic(&mnemonic);
    }

    /// Génère une phrase mnémonique à partir d'une phrase donnée (12/24/Etc. mots) et affiche la clé publique Solana correspondante.
    /// Cette méthode permet d'utiliser une phrase existante pour récupérer ou accéder à un portefeuille.
    pub fn generate_and_print_mnemonic_from_phrase(&self, phrase: &str) {
        let mnemonic_manager = MnemonicManager::new(self.config.clone());

        println!("BIP39 Mnemonic (given phrase): {}", phrase);

        // Convertit la chaîne de caractères fournie en une structure mnémonique valide.
        // Cette étape vérifie que la phrase correspond aux critères du standard BIP39 et qu'elle peut être utilisée pour générer une seed.
        let mnemonic = BipMnemonic::get_mnemonic_from_phrase(phrase);

        mnemonic_manager.process_mnemonic(&mnemonic);
    }

    pub fn get_balance_by_pubkey(&self, pubkey: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let solana_balance = SolanaBalance::new(self.config.clone());
        solana_balance.get_balance_by_pubkey(pubkey)
    }
}
