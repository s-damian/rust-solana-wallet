use std::env;

#[cfg(test)]
use serial_test::serial;

#[derive(Clone)] // Cette ligne est utile pour implémenter automatiquement Clone.
pub struct WalletConfig {
    pub keypair_path: String,
    pub keypair_dir: String,
    pub nb_derivations: usize,
    pub rpc_url: String,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// # Assignments:
///
/// - nb_derivations:
///   Récupère le nombre de dérivations à effectuer à partir de la variable d'environnement `NB_DERIVATIONS`.
///   Retourne 1 par défaut si la variable n'est pas définie ou si sa valeur n'est pas un entier valide.
///
/// - keypair_path:
///   Récupère le chemin d'accès au fichier où la paire de clés principale est stockée à partir de la variable d'environnement `KEYPAIR_PATH`.
///   Retourne un chemin par défaut si la variable d'environnement n'est pas définie.
///
/// - keypair_dir:
///   Récupère le chemin d'accès au dossier où les paires de clés dérivées sont stockées, spécifié par la variable d'environnement `KEYPAIR_DIR`.
///   Retourne un chemin par défaut si la variable d'environnement n'est pas définie.
///
/// - rpc_url:
///   Récupère l'URL du serveur RPC pour l'accès au réseau Solana à partir de la variable d'environnement `RPC_URL`.
///   Retourne "https://api.testnet.solana.com" par défaut, indiquant que le réseau Testnet est utilisé si la variable d'environnement n'est pas définie.

impl WalletConfig {
    /// Charge la configuration depuis les variables d'environnement ou utilise les valeurs par défaut.
    pub fn new() -> Self {
        Self {
            nb_derivations: env::var("NB_DERIVATIONS")
                .unwrap_or_else(|_| "0".to_string()) // Utilise "1" comme valeur par défaut si la variable n'est pas définie.
                .parse::<usize>() // Tente de convertir la chaîne de caractères en un entier de type usize.
                .unwrap_or(0), // Retourne 1 si la conversion échoue ou si la valeur convertie n'est pas un nombre.
            keypair_path: env::var("KEYPAIR_PATH")
                .unwrap_or_else(|_| "./storage/keypair/id.json".to_string()),
            keypair_dir: env::var("KEYPAIR_DIR")
                .unwrap_or_else(|_| "./storage/keypair/derived".to_string()),
            rpc_url:
                env::var("RPC_URL") // Lire l'URL RPC de l'environnement
                    .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string()), // URL par défaut pointant vers le Testnet de Solana.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn setup() {
        env::remove_var("NB_DERIVATIONS");
        env::remove_var("KEYPAIR_PATH");
        env::remove_var("KEYPAIR_DIR");
        env::remove_var("RPC_URL");
    }

    fn teardown() {
        dotenv::from_filename(".env.testing").ok();
    }

    #[test]
    #[serial]
    fn test_default_values() {
        setup();

        let config = WalletConfig::new();
        assert_eq!(config.nb_derivations, 0);
        assert_eq!(config.keypair_path, "./storage/keypair/id.json");
        assert_eq!(config.keypair_dir, "./storage/keypair/derived");
        assert_eq!(config.rpc_url, "https://api.devnet.solana.com");

        teardown();
    }

    #[test]
    #[serial]
    fn test_custom_values() {
        setup();

        env::set_var("NB_DERIVATIONS", "5");
        env::set_var("KEYPAIR_PATH", "./storage/custom/keypair/id.json");
        env::set_var("KEYPAIR_DIR", "./storage/custom/keypair/derived");
        env::set_var("RPC_URL", "https://custom.rpc.url");

        let config = WalletConfig::new();
        assert_eq!(config.nb_derivations, 5);
        assert_eq!(config.keypair_path, "./storage/custom/keypair/id.json");
        assert_eq!(config.keypair_dir, "./storage/custom/keypair/derived");
        assert_eq!(config.rpc_url, "https://custom.rpc.url");

        teardown();
    }

    #[test]
    #[serial]
    fn test_invalid_nb_derivations() {
        setup();

        env::set_var("NB_DERIVATIONS", "not_a_number");

        let config = WalletConfig::new();
        assert_eq!(config.nb_derivations, 0); // Should default to 0 if invalid

        teardown();
    }
}
