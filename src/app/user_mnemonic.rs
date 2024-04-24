use crate::bip::passphrase::prompt_for_passphrase;
use crate::bip::seed::{derive_seed_bytes, generate_seed, get_seed_bytes};
use crate::solana::address::{generate_keypair, write_keypair};
use crate::utils::env::{get_keypair_dir, get_keypair_path, get_nb_derivations};
use solana_sdk::signer::Signer;

/// Traite une mnémonique pour générer et afficher la clé publique correspondante, en prenant en compte les dérivations spécifiées.
/// Cette fonction sert de point central pour la création de clés Solana à partir d'une phrase mnémonique.
pub fn process_mnemonic(mnemonic: &bip39::Mnemonic) {
    // Demande à l'utilisateur d'entrer une passphrase optionnelle qui sera utilisée lors de la génération de la seed.
    // (laisser vide pour ne pas utiliser de passphrase)
    let passphrase = prompt_for_passphrase();

    // Génère une seed en format hexadécimal à partir de la phrase mnémonique et de la passphrase.
    // Cette seed de portefeuille HD (Hiérarchiquement Déterministe) permettra de produire une suite cohérente de clés dérivées.
    let seed = generate_seed(mnemonic, &passphrase);
    println!("BIP39 Seed: {:X}", seed);

    // Convertit la seed en un tableau de bytes bruts, qui servira de base pour la génération de clés dérivées.
    let seed_bytes = get_seed_bytes(&seed);

    // Récupère le nombre de dérivations souhaitées (est de 1 par défaut).
    let nb_derivations = get_nb_derivations();
    if nb_derivations == 1 {
        // Génerer une paire de clés (clé publique et clé privée) à partir de la seed en bytes.
        // Puis écrire cette paire de clés dans un fichier JSON.
        let keypair = generate_keypair(seed_bytes);
        let keypair_path = get_keypair_path();
        write_keypair(&keypair, &keypair_path);

        // Affiche la clé publique (qui dans le cas de Solana, est également utilisée comme adresse publique du wallet).
        println!("Solana Public Key: {}", keypair.pubkey());
    } else {
        // Gère les dérivations multiples pour générer plusieurs paires de clés.
        for index in 0..nb_derivations {
            // Dériver la seed pour chaque index spécifié (sauf pour l'index 0 qui utilise la seed originale).
            match derive_seed_bytes(seed_bytes, index) {
                Ok(derived_seed_bytes) => {
                    // Génerer une paire de clés (clé publique et clé privée) à partir de la seed en bytes.
                    // Puis écrire cette paire de clés dans un fichier JSON.
                    let keypair = generate_keypair(&derived_seed_bytes);
                    let keypair_path = if index == 0 {
                        get_keypair_path()
                    } else {
                        format!("{}/keypair-{}.json", get_keypair_dir(), index)
                    };
                    write_keypair(&keypair, &keypair_path);

                    // Affiche la clé publique (qui dans le cas de Solana, est également utilisée comme adresse publique du wallet).
                    println!("Solana Public Key {}: {}", index, keypair.pubkey());
                }
                Err(e) => println!("Error deriving seed bytes: {}", e),
            }
        }
    }
}
