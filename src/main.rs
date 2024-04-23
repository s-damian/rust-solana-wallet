mod bip;
mod solana;
mod utils;

use bip::mnemonic::{generate_mnemonic, get_mnemonic_from_phrase, get_mnemonic_to_str};
use bip::passphrase::prompt_for_passphrase;
use bip::seed::{derive_seed_bytes, generate_seed, get_seed_bytes};
use clap::{Arg, Command};
use dotenv::dotenv;
use solana::address::{generate_keypair, read_keypair_from_file, write_keypair};
use solana_sdk::signer::Signer;
use utils::env::{get_keypair_dir, get_keypair_path, get_nb_derivations};

fn main() {
    dotenv().ok(); // Charge les variables d'environnement du fichier .env

    let matches = Command::new("Solana Wallet")
        .version("1.0.0")
        .about("Example of a Solana Wallet in Rust")
        .subcommand(Command::new("generate_seed").about("Generates a new random mnemonic"))
        .subcommand(
            Command::new("from_mnemonic")
                .about("Generates a mnemonic from a specified phrase")
                .arg(
                    Arg::new("PHRASE")
                        .help("A 12-word mnemonic phrase")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("get_pubkey_from_keypair_file")
                .about("Displays the public key from the keypair stored in file"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate_seed", _sub_matches)) => {
            generate_and_print_random_mnemonic();
        }
        Some(("from_mnemonic", sub_matches)) => {
            if let Some(phrase) = sub_matches.get_one::<String>("PHRASE") {
                generate_and_print_mnemonic_from_phrase(phrase);
            }
        }
        Some(("get_pubkey_from_keypair_file", _sub_matches)) => {
            get_pubkey_from_keypair_file();
        }
        _ => println!("Unknown command."),
    }
}

/// Générer un mnémonique (12 mots) aléatoire et afficher la clé publique Solana correspondante.
fn generate_and_print_random_mnemonic() {
    // Créer une nouvelle phrase mnémonique générée aléatoirement.
    // Il s'agit d'une mnémonique de 12 mots, ce qui est un standard commun pour de nombreux portefeuilles.
    let mnemonic = generate_mnemonic();

    // Récupérer la chaîne de caractères de la phrase mnémonique.
    // Cette phrase est utilisée pour générer une seed et peut être utilisée pour la récupération d'un portefeuille.
    let phrase = get_mnemonic_to_str(&mnemonic);

    println!("BIP39 Mnemonic (random phrase): {}", phrase);

    process_mnemonic(&mnemonic);
}

/// Générer un mnémonique à partir d'une phrase donnée (12/24/Etc. mots) et afficher la clé publique Solana correspondante.
fn generate_and_print_mnemonic_from_phrase(phrase: &str) {
    println!("BIP39 Mnemonic (given phrase): {}", phrase);

    let mnemonic = get_mnemonic_from_phrase(phrase);
    process_mnemonic(&mnemonic);
}

fn process_mnemonic(mnemonic: &bip39::Mnemonic) {
    // Passphrase optionnelle (laisser vide pour ne pas utiliser de passphrase).
    let passphrase = prompt_for_passphrase();

    // Générer la seed (format hexadécimal) de portefeuille HD (Hiérarchiquement Déterministe) à partir de la mnémonique.
    // Cette seed peut être utilisée pour générer des clés déterministes pour un portefeuille de cryptomonnaie.
    let seed = generate_seed(mnemonic, &passphrase);
    //let seed = generate_seed(mnemonic, "");
    println!("BIP39 Seed: {:X}", seed);

    // Récupérer la seed du portefeuille HD sous forme de bytes bruts.
    // Ce tableau de bytes représente la seed sous sa forme binaire la plus fondamentale.
    let seed_bytes = get_seed_bytes(&seed);

    let nb_derivations = get_nb_derivations();
    if nb_derivations == 1 {
        // Génerer une paire de clés (clé publique et clé privée) à partir de la seed en bytes.
        // Puis écrire cette paire de clés dans un fichier JSON.
        let keypair = generate_keypair(seed_bytes);
        let keypair_path = get_keypair_path();
        write_keypair(&keypair, &keypair_path);

        // Clé public Solana (qui dans le cas de Solana, est également utilisée comme adresse publique du wallet).
        println!("Public Key: {}", keypair.pubkey());
    } else {
        for index in 0..nb_derivations {
            match derive_seed_bytes(seed_bytes, index) {
                Ok(derived_seed_bytes) => {
                    // Génerer une paire de clés (clé publique et clé privée) à partir de la seed en bytes.
                    // Puis écrire cette paire de clés dans un fichier JSON.
                    let keypair = generate_keypair(&derived_seed_bytes);
                    let keypair_path = format!("{}/keypair-{}.json", get_keypair_dir(), index);
                    write_keypair(&keypair, &keypair_path);

                    // Clé public Solana (qui dans le cas de Solana, est également utilisée comme adresse publique du wallet).
                    println!("Public Key {}: {}", index, keypair.pubkey());
                }
                Err(e) => println!("Error deriving seed bytes: {}", e),
            }
        }
    }
}

/// Récuperer la clé publique à partir d'une paire de clés stockée dans un fichier.
fn get_pubkey_from_keypair_file() {
    // Chemin vers le fichier où la paire de clés est stockée.
    let keypair_path = get_keypair_path();

    // Tentative de lecture de la paire de clés à partir du fichier spécifié.
    match read_keypair_from_file(&keypair_path) {
        Ok(keypair) => println!("Public Key: {}", keypair.pubkey()),
        Err(e) => println!("Failed to read the keypair from file: {}", e),
    }
}
