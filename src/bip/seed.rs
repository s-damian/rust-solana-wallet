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
        // Si l'index est 0, retourne directement les octets de la seed originale sans modification.
        // Cela garantit que la clé principale reste inchangée si aucune dérivation n'est requise.
        if index == 0 {
            return Ok(seed_bytes.to_vec());
        }

        // Construit le chemin de dérivation complet en utilisant le standard BIP44 pour Solana.
        // Le format est "m/44'/501'/0'/0/{index}", où {index} représente le numéro de la dérivation.
        // SLIP44: 501 = Solana Coin (SOL Symbol).
        //
        // Le chemin de dérivation utilisé dans BIP44 suit cette structure :
        // m / purpose' / coin_type' / account' / change / address_index
        // # Résumé :
        // - m:
        //   Est la "master key", la racine de toutes les dérivations.
        // - purpose:
        //   Est une constante fixée à 44' (pour indiquer qu'on suit la norme BIP44).
        // - coin_type:
        //   Est une constante (integer), définie pour chaque crypto-monnaie.
        // - account:
        //   Les users peuvent utiliser ces comptes pour organiser les fonds de la même manière que les comptes bancaires.
        //   Ce numéro est utilisé comme index enfant dans la dérivation BIP32.
        // - change: 0 ou 1.
        //   La constante 0 est utilisée pour la chaîne externe. La constante 1 pour la chaîne interne (également appelée adresse de changement).
        // - address_index:
        //   Représente le numéro séquentiel d’une adresse à l’intérieur de cet "account".
        let path = format!("m/44'/501'/0'/0/{}", index);
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
