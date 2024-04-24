use std::env;

/// Récupère le nombre de dérivations à effectuer à partir de la variable d'environnement `NB_DERIVATIONS`.
/// Retourne 1 par défaut si la variable n'est pas définie ou si sa valeur n'est pas un entier valide.
pub fn get_nb_derivations() -> usize {
    env::var("NB_DERIVATIONS")
        .unwrap_or_else(|_| "1".to_string()) // Utilise "1" comme valeur par défaut si la variable n'est pas définie.
        .parse::<usize>() // Tente de convertir la chaîne de caractères en un entier de type usize.
        .unwrap_or(1) // Retourne 1 si la conversion échoue ou si la valeur convertie n'est pas un nombre.
}

/// Récupère le chemin d'accès au fichier où la paire de clés principale est stockée à partir de la variable d'environnement `KEYPAIR_PATH`.
/// Retourne un chemin par défaut si la variable d'environnement n'est pas définie.
pub fn get_keypair_path() -> String {
    env::var("KEYPAIR_PATH").unwrap_or_else(|_| "./storage/keypair/id.json".to_string())
}

/// Récupère le chemin d'accès au dossier où les paires de clés dérivées sont stockées, spécifié par la variable d'environnement `KEYPAIR_DIR`.
/// Retourne un chemin par défaut si la variable d'environnement n'est pas définie.
pub fn get_keypair_dir() -> String {
    env::var("KEYPAIR_DIR").unwrap_or_else(|_| "./storage/keypair/derived".to_string())
}
