use std::env;

/// Retourne le nombre de dérivations.
pub fn get_nb_derivations() -> usize {
    env::var("NB_DERIVATIONS")
        .unwrap_or_else(|_| "1".to_string()) // Utilise "1" si la variable n'est pas définie.
        .parse::<usize>() // Tente de convertir la chaîne en usize.
        .unwrap_or(1) // Utilise 1 si la conversion échoue ou si la valeur n'est pas un entier.
}

/// Retourne le path du fichier de paire de clés.
pub fn get_keypair_path() -> String {
    env::var("KEYPAIR_PATH").unwrap_or_else(|_| "./storage/keypair/id.json".to_string())
}

/// Retourne le path vers le dossier de stockage des paires de clés.
pub fn get_keypair_dir() -> String {
    env::var("KEYPAIR_DIR").unwrap_or_else(|_| "./storage/keypair/derived".to_string())
}
