use crate::app::mnemonic_manager::process_mnemonic;
use crate::bip::mnemonic::{generate_mnemonic, get_mnemonic_from_phrase, get_mnemonic_to_str};
use crate::config::wallet_config::WalletConfig;

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
        // Créer une nouvelle phrase mnémonique générée aléatoirement (en suivant le standard BIP39).
        // Il s'agit d'une mnémonique de 12 mots, ce qui est un standard commun pour de nombreux portefeuilles.
        let mnemonic = generate_mnemonic();

        // Convertit la mnémonique en une chaîne de caractères (pour pouvoir l'afficher et l'utiliser ultérieurement).
        // Cette phrase est utilisée pour générer une seed et peut être utilisée pour la récupération d'un portefeuille.
        let phrase = get_mnemonic_to_str(&mnemonic);

        println!("BIP39 Mnemonic (random phrase): {}", phrase);

        process_mnemonic(&mnemonic, &self.config);
    }

    /// Génère une phrase mnémonique à partir d'une phrase donnée (12/24/Etc. mots) et affiche la clé publique Solana correspondante.
    /// Cette méthode permet d'utiliser une phrase existante pour récupérer ou accéder à un portefeuille.
    pub fn generate_and_print_mnemonic_from_phrase(&self, phrase: &str) {
        println!("BIP39 Mnemonic (given phrase): {}", phrase);

        // Convertit la chaîne de caractères fournie en une structure mnémonique valide.
        // Cette étape vérifie que la phrase correspond aux critères du standard BIP39 et qu'elle peut être utilisée pour générer une seed.
        let mnemonic = get_mnemonic_from_phrase(phrase);

        process_mnemonic(&mnemonic, &self.config);
    }
}
