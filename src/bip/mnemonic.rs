use bip39::{Language, Mnemonic, MnemonicType};

/// Générer une phrase mnémonique (12 mots aléatoires).
pub fn generate_mnemonic() -> Mnemonic {
    Mnemonic::new(MnemonicType::Words12, Language::English)
}

/// Récupérer la phrase mnémonique sous forme de chaîne de caractères.
pub fn get_mnemonic_to_str(mnemonic: &Mnemonic) -> &str {
    mnemonic.phrase()
}

/// Créer une phrase mnémonique à partir d'une phrase (12/24/Etc. mots) donnée.
pub fn get_mnemonic_from_phrase(phrase: &str) -> Mnemonic {
    Mnemonic::from_phrase(phrase, Language::English).unwrap()
}
