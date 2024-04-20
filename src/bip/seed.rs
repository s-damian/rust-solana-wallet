use bip39::{Mnemonic, Seed};

// Générer la seed à partir de la phrase mnémonique.
pub fn generate_seed(mnemonic: &Mnemonic, passphrase: &str) -> Seed {
    Seed::new(mnemonic, passphrase)
}

// Récupérer la seed sous forme de bytes bruts.
pub fn get_seed_bytes(seed: &Seed) -> &[u8] {
    seed.as_bytes()
}
