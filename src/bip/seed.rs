use bip32::{DerivationPath, Error as Bip32Error, XPrv};
use bip39::{Mnemonic, Seed};
use std::str::FromStr;

pub struct BipSeed {}

impl BipSeed {
    /// Génère la seed à partir de la phrase mnémonique (mnemonic param).
    ///
    /// # Arguments:
    /// - mnemonic - La mnémonique à partir de laquelle la seed est générée.
    /// - passphrase - La passphrase optionnelle pour ajouter une couche de sécurité supplémentaire à la seed.
    ///
    /// # Returns:
    /// Retourne la seed générée.
    pub fn generate_seed(mnemonic: &Mnemonic, passphrase: &str) -> Seed {
        Seed::new(mnemonic, passphrase)
    }

    /// Récupère la seed sous forme d'octets bruts.
    ///
    /// # Arguments:
    /// - seed - La seed dont les octets bruts doivent être récupérés.
    ///
    /// # Returns:
    /// Retourne une référence vers un tableau d'octets.
    pub fn get_seed_bytes(seed: &Seed) -> &[u8] {
        seed.as_bytes()
    }

    /// Dérive la seed (seed_bytes param) pour générer différentes clés privées en fonction de l'index spécifié (index param).
    /// Utilise le chemin de dérivation BIP44 spécifique à Solana.
    ///
    /// # Arguments:
    /// - seed_bytes - Les octets de la seed à partir de laquelle les clés seront dérivées.
    /// - index - L'index de dérivation utilisé pour générer différentes clés privées.
    ///
    /// # Returns:
    /// Retourne un vecteur contenant les octets de la clé privée dérivée. Ou retourne une erreur en cas de problème lors de la dérivation.
    pub fn derive_seed_bytes(seed_bytes: &[u8], index: usize) -> Result<Vec<u8>, Bip32Error> {
        // SLIP44: 501 = Solana Coin (SOL Symbol).
        // Format: "m/44'/501'/{index}'/0'" (Style Trezor)
        //
        // Le chemin de dérivation utilisé dans BIP44 suit cette structure :
        // m / purpose' / coin_type' / account' / change'
        // # Résumé :
        // - m : Est la "master key", la racine de toutes les dérivations.
        // - purpose : Est une constante fixée à 44' (pour indiquer qu'on suit la norme BIP44).
        // - coin_type : Est une constante (integer), définie pour chaque crypto-monnaie.
        // - account : Account index (incrémenté pour chaque nouveau compte).
        // - change (0 ou 1) : 0' pour sous-niveau hardened fixe (structure HD Trezor).
        //
        // Note :
        // Solana utilise Ed25519 (et non secp256k1 comme BTC/ETH).
        // Ed25519 (SLIP-0010) exige que tous les niveaux soient hardened (').
        // Les niveaux non-hardened (change/address_index classiques de BIP44) ne sont pas supportés avec Solana,
        // d'où l'utilisation de /0' au lieu de /0/0.

        //let path = format!("m/44'/501'/0'/0/{}", index); // Deprecated derivation paths (Non-hardened invalide pour Solana).
        //let path = format!("m/44'/501'/{}'", index); // bip44 grouping : Style Ledger (simple).
        let path = format!("m/44'/501'/{}'/0'", index); // bip44Change grouping : Style Trezor (avec sous-niveau).
        let derivation_path = DerivationPath::from_str(&path)?;

        // Créer une clé privée étendue à partir des octets de la seed.
        let mut master_xprv = XPrv::new(seed_bytes)?;

        // Itère sur chaque segment du chemin de dérivation pour dériver la clé.
        // Chaque segment correspond à un niveau de dérivation dans la hiérarchie BIP32.
        for child_number in derivation_path {
            master_xprv = master_xprv.derive_child(child_number)?;
        }

        // Retourne les octets de la clé privée dérivée, utilisable pour générer des paires de clés.
        Ok(master_xprv.private_key().to_bytes().to_vec())
    }
}
