use std::env;
use dotenv::dotenv;
use clap::{Arg, Command};
mod bip;
mod solana;

use bip::mnemonic::{generate_mnemonic, get_mnemonic_from_phrase, get_mnemonic_to_str};
use bip::seed::{generate_seed, get_seed_bytes};
use solana::address::{generate_keypair, read_keypair_from_file, write_keypair};
use solana_sdk::signer::Signer;

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

    println!("BIP39 Mnemonic (random phrase) : {}", phrase);

    process_mnemonic(&mnemonic);
}

/// Générer un mnémonique à partir d'une phrase donnée (12/24/Etc. mots) et afficher la clé publique Solana correspondante.
fn generate_and_print_mnemonic_from_phrase(phrase: &str) {
    println!("BIP39 Mnemonic (given phrase) : {}", phrase);

    let mnemonic = get_mnemonic_from_phrase(phrase);
    process_mnemonic(&mnemonic);
}

fn process_mnemonic(mnemonic: &bip39::Mnemonic) {
    // Générer la seed (format hexadécimal) de portefeuille HD (Hiérarchiquement Déterministe) à partir de la mnémonique.
    // Cette seed peut être utilisée pour générer des clés déterministes pour un portefeuille de cryptomonnaie.
    let seed = generate_seed(mnemonic, "");
    println!("BIP39 Seed : {:X}", seed);

    // Récupérer la seed du portefeuille HD sous forme de bytes bruts.
    // Ce tableau de bytes représente la seed sous sa forme binaire la plus fondamentale.
    let seed_bytes = get_seed_bytes(&seed);

    // Génerer une paire de clés (clé publique et clé privée) à partir de la seed en bytes.
    // Puis écrire cette paire de clés dans un fichier JSON.
    let keypair = generate_keypair(seed_bytes);
    let keypair_path = get_keypair_path();
    write_keypair(&keypair, &keypair_path);

    // Clé public Solana (qui dans le cas de Solana, est également utilisée comme adresse publique du wallet).
    println!("Public Key: {}", keypair.pubkey());
}

/// Récuperer la clé publique à partir d'une paire de clés stockée dans un fichier.
fn get_pubkey_from_keypair_file() {
    // Chemin vers le fichier où la paire de clés est stockée.
    //let file_path = "./storage/keypair/id.json";
    let keypair_path = get_keypair_path();

    // Tentative de lecture de la paire de clés à partir du fichier spécifié.
    match read_keypair_from_file(&keypair_path) {
        Ok(keypair) => println!("Public Key: {}", keypair.pubkey()),
        Err(e) => println!("Failed to read the keypair from file: {}", e),
    }
}

/// Récupère le path dy fichier de paire de clés à partir des variables d'environnement ou utilise une valeur par défaut si elle n'est pas définie.
fn get_keypair_path() -> String {
    env::var("KEYPAIR_PATH").unwrap_or_else(|_| "./storage/keypair/id.json".to_string())
}
