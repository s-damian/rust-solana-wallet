use bip32::{DerivationPath, Error as Bip32Error, XPrv};
use bip39::{Mnemonic, Seed};
use std::str::FromStr;

/// Générer la seed à partir de la phrase mnémonique.
pub fn generate_seed(mnemonic: &Mnemonic, passphrase: &str) -> Seed {
    Seed::new(mnemonic, passphrase)
}

/// Récupérer la seed sous forme de bytes bruts.
pub fn get_seed_bytes(seed: &Seed) -> &[u8] {
    seed.as_bytes()
}

/// Dériver la seed pour générer différentes clés en fonction de l'index (index param) spécifié.
/// Cette fonction prend en compte l'index de dérivation pour générer des clés dérivées
/// en utilisant le standard BIP44 spécifique à Solana.
pub fn derive_seed_bytes(seed: &[u8], index: usize) -> Result<Vec<u8>, Bip32Error> {
    // Si l'index est 0, on retourne directement la seed originale sans modification.
    // Cela garantit que la clé principale reste inchangée si aucune dérivation n'est requise.
    if index == 0 {
        return Ok(seed.to_vec());
    }

    // Construire le path de dérivation complet en utilisant le standard BIP44 pour Solana.
    // Le format est "m/44'/501'/0'/0/{index}", où {index} représente le numéro de la dérivation.
    let path = format!("m/44'/501'/0'/0/{}", index);
    let derivation_path = DerivationPath::from_str(&path)?;

    // Créer une clé privée étendue à partir de la seed.
    let mut master_xprv = XPrv::new(seed)?;

    // Itérer sur chaque segment du path de dérivation.
    // Chaque segment correspond à un niveau de dérivation dans la hiérarchie BIP32.
    for child_number in derivation_path {
        master_xprv = master_xprv.derive_child(child_number)?;
    }

    // Retourner les bytes de la clé privée dérivée.
    // Cette clé privée peut ensuite être utilisée pour générer une paire de clés (publique et privée).
    Ok(master_xprv.private_key().to_bytes().to_vec())
}
