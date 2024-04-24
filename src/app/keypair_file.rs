use crate::solana::address::read_keypair_from_file;
use crate::utils::env::get_keypair_path;
use solana_sdk::signer::Signer;

/// Récupère et affiche la clé publique à partir d'une paire de clés stockée localement dans un fichier.
pub fn get_pubkey_from_keypair_file() {
    // Obtient le chemin d'accès au fichier où la paire de clés est stockée.
    let keypair_path = get_keypair_path();

    // Tente de lire la paire de clés à partir du fichier spécifié (en utilisant le chemin obtenu précédemment).
    // La fonction "read_keypair_from_file" gère le chargement et la désérialisation de la paire de clés à partir du fichier.
    // Ok: En cas de succès, extrait la clé publique de la paire de clés et l'affiche.
    // Err: En cas d'échec, affiche une erreur indiquant que la lecture a échoué.
    match read_keypair_from_file(&keypair_path) {
        Ok(keypair) => println!("Clé publique : {}", keypair.pubkey()),
        Err(e) => println!(
            "Échec de la lecture de la paire de clés depuis le fichier : {}",
            e
        ),
    }
}
