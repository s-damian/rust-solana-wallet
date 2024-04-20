mod bip;
mod solana;

use bip::mnemonic::{generate_mnemonic, get_mnemonic_from_phrase, get_mnemonic_to_str};
use bip::seed::{generate_seed, get_seed_bytes};
use solana::address::{generate_keypair, write_keypair};
use solana_sdk::signer::Signer;

fn main() {
    println!("-------------- START generate_and_print_random_mnemonic --------------");
    generate_and_print_random_mnemonic();
    println!("-------------- END generate_and_print_random_mnemonic --------------");

    let given_phrase =
        "fit refuse hotel collect tortoise race rail weasel little medal couch remember";
    println!("-------------- START generate_and_print_mnemonic_from_phrase --------------");
    generate_and_print_mnemonic_from_phrase(given_phrase);
    println!("-------------- END generate_and_print_mnemonic_from_phrase --------------");
}

/// Générer un mnémonique (12 mots) aléatoire et afficher l'adresse publique Solana correspondante.
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

/// Générer un mnémonique à partir d'une phrase donnée (12 mots) et afficher l'adresse publique Solana correspondante.
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
    let seed2 = generate_seed(mnemonic, "");
    println!("--- Seed ENCORE SANS passphrase (format hexadécimal) : {:X}", seed2);
    let seed3 = generate_seed(mnemonic, "stephen");
    println!("--- Seed AVEC passphrase (format hexadécimal) : {:X}", seed3);

    // Récupérer la seed du portefeuille HD sous forme de bytes bruts.
    // Ce tableau de bytes représente la seed sous sa forme binaire la plus fondamentale.
    let seed_bytes = get_seed_bytes(&seed);

    let keypair = generate_keypair(seed_bytes);

    write_keypair(&keypair, "./storage/keypair.txt");

    // Adresse public solana.
    println!("--- Public key: {}", keypair.pubkey());
}
