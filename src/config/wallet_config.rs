use std::env;

pub struct WalletConfig {
    pub keypair_path: String,
    pub keypair_dir: String,
    pub nb_derivations: usize,
}

/// - nb_derivations :
/// Récupère le nombre de dérivations à effectuer à partir de la variable d'environnement `NB_DERIVATIONS`.
/// Retourne 1 par défaut si la variable n'est pas définie ou si sa valeur n'est pas un entier valide.
///
/// - keypair_path :
/// Récupère le chemin d'accès au fichier où la paire de clés principale est stockée à partir de la variable d'environnement `KEYPAIR_PATH`.
/// Retourne un chemin par défaut si la variable d'environnement n'est pas définie.
///
/// - keypair_dir :
/// Récupère le chemin d'accès au dossier où les paires de clés dérivées sont stockées, spécifié par la variable d'environnement `KEYPAIR_DIR`.
/// Retourne un chemin par défaut si la variable d'environnement n'est pas définie.
impl WalletConfig {
    /// Charge la configuration depuis les variables d'environnement ou utilise les valeurs par défaut.
    pub fn new() -> Self {
        Self {
            nb_derivations: env::var("NB_DERIVATIONS")
                .unwrap_or_else(|_| "1".to_string()) // Utilise "1" comme valeur par défaut si la variable n'est pas définie.
                .parse::<usize>() // Tente de convertir la chaîne de caractères en un entier de type usize.
                .unwrap_or(1), // Retourne 1 si la conversion échoue ou si la valeur convertie n'est pas un nombre.
            keypair_path: env::var("KEYPAIR_PATH")
                .unwrap_or_else(|_| "./storage/keypair/id.json".to_string()),
            keypair_dir: env::var("KEYPAIR_DIR")
                .unwrap_or_else(|_| "./storage/keypair/derived".to_string()),
        }
    }
}
