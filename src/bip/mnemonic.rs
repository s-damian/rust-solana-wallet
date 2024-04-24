use bip39::{Language, Mnemonic, MnemonicType};

/// Génère une phrase mnémonique (12 mots aléatoires) selon le standard BIP39.
/// Cette phrase est utilisée pour sécuriser et restaurer des portefeuilles cryptographiques.
pub fn generate_mnemonic() -> Mnemonic {
    Mnemonic::new(MnemonicType::Words12, Language::English)
}

/// Récupère et retourne la phrase mnémonique sous forme de chaîne de caractères.
/// La phrase est la représentation humainement lisible de la seed cryptographique.
pub fn get_mnemonic_to_str(mnemonic: &Mnemonic) -> &str {
    mnemonic.phrase()
}

/// Crée une phrase mnémonique à partir d'une phrase spécifique donnée (12/24/Etc. mots).
/// La phrase doit correspondre aux critères du standard BIP39 et être en anglais.
/// Retourne un objet Mnemonic si la phrase est valide selon BIP39, sinon une erreur est déclenchée.
pub fn get_mnemonic_from_phrase(phrase: &str) -> Mnemonic {
    Mnemonic::from_phrase(phrase, Language::English).expect("Failed to get mnemonic from phrase")
}
