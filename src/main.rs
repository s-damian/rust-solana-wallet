use clap::{Arg, Command};
mod bip;
mod solana;

use bip::mnemonic::{generate_mnemonic, get_mnemonic_from_phrase, get_mnemonic_to_str};
use bip::seed::{generate_seed, get_seed_bytes};
use solana::address::{generate_keypair, read_keypair_from_file, write_keypair};
use solana_sdk::signer::Signer;

fn main() {
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
            Command::new("get_pubkey_from_file")
                .about("Displays the public key from the keypair stored in file"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate_seed", _sub_matches)) => {
            println!("-------------- START generate_and_print_random_mnemonic --------------");
            generate_and_print_random_mnemonic();
            println!("-------------- END generate_and_print_random_mnemonic --------------");
        }
        Some(("from_mnemonic", sub_matches)) => {
            if let Some(phrase) = sub_matches.get_one::<String>("PHRASE") {
                println!(
                    "-------------- START generate_and_print_mnemonic_from_phrase --------------"
                );
                generate_and_print_mnemonic_from_phrase(phrase);
                println!(
                    "-------------- END generate_and_print_mnemonic_from_phrase --------------"
                );
            }
        }
        Some(("get_pubkey_from_file", _sub_matches)) => {
            get_pubkey_from_file();
        }
        _ => println!("Commande inconnue."),
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

    println!("--- Phrase (aléatoire) : {}", phrase);

    process_mnemonic(&mnemonic);
}

/// Générer un mnémonique à partir d'une phrase donnée (12 mots) et afficher la clé publique Solana correspondante.
fn generate_and_print_mnemonic_from_phrase(phrase: &str) {
    println!("--- Phrase (donnée) : {}", phrase);

    let mnemonic = get_mnemonic_from_phrase(phrase);
    process_mnemonic(&mnemonic);
}

fn process_mnemonic(mnemonic: &bip39::Mnemonic) {
    // Générer la seed de portefeuille HD (Hiérarchiquement Déterministe) à partir de la mnémonique.
    // Cette seed peut être utilisée pour générer des clés déterministes pour un portefeuille de cryptomonnaie.
    let seed = generate_seed(mnemonic, "");
    println!("--- Seed SANS passphrase (format hexadécimal) : {:X}", seed);

    let seed_with_passphrase = generate_seed(mnemonic, "stephen");
    println!(
        "--- Seed AVEC passphrase (format hexadécimal) : {:X}",
        seed_with_passphrase
    );

    // Récupérer la seed du portefeuille HD sous forme de bytes bruts.
    // Ce tableau de bytes représente la seed sous sa forme binaire la plus fondamentale.
    let seed_bytes = get_seed_bytes(&seed);

    let keypair = generate_keypair(seed_bytes);

    write_keypair(&keypair, "./storage/keypair/id.json");

    // Clé public Solana (qui dans le cas de Solana, est également utilisée comme adresse publique du wallet).
    println!("--- Public key: {}", keypair.pubkey());
}

fn get_pubkey_from_file() {
    let file_path = "./storage/keypair/id.json";

    match read_keypair_from_file(file_path) {
        Ok(keypair) => println!("--- Public key: {}", keypair.pubkey()),
        Err(e) => println!("Failed to read the keypair from file: {}", e),
    }
}
