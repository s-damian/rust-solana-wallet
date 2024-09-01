use crate::common;
use std::process::Command;
use std::str;

#[test]
fn test_generate_seed() {
    common::setup();

    // Exécute la commande.
    let output = Command::new("cargo")
        .args(&["run", "--", "generate_seed"])
        .output()
        .expect("Failed to execute command");

    // Convertit la sortie en chaîne de caractères.
    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");

    // Vérifie que la commande s'est exécutée avec succès.
    assert!(output.status.success(), "Error: Command failed to execute");

    // Vérifie que la sortie contient les éléments attendus.
    assert!(
        output_str.contains("BIP39 Mnemonic (random phrase):"),
        "Error: Mnemonic phrase not found in output"
    );
    assert!(
        output_str.contains("Enter passphrase (optional)"),
        "Error: passphrase not found in output"
    );
    assert!(
        output_str.contains("Seed:"),
        "Error: Seed not found in output"
    );
    assert!(
        output_str.contains("Solana Public Key 0:"),
        "Public key not found in output"
    );

    // Vérifie le format de la phrase mnémonique (12 mots séparés par des espaces).
    let mnemonic_line = output_str
        .lines()
        .find(|line| line.starts_with("BIP39 Mnemonic"))
        .expect("Mnemonic line not found");
    let mnemonic_words: Vec<&str> = mnemonic_line
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();
    assert_eq!(mnemonic_words.len(), 12, "Mnemonic should contain 12 words");

    // Vérifie le format de la seed (64 caractères hexadécimaux).
    let seed_line = output_str
        .lines()
        .find(|line| line.starts_with("Seed:"))
        .expect("Seed line not found");
    let seed = seed_line.split(":").nth(1).unwrap().trim();
    assert_eq!(seed.len(), 128, "Seed should be 128 characters long");
    assert!(
        seed.chars().all(|c| c.is_ascii_hexdigit()),
        "Seed should only contain hexadecimal characters"
    );

    // Vérifie le format de la clé publique Solana (adresse de 32 caractères base58).
    let pubkey_line = output_str
        .lines()
        .find(|line| line.starts_with("Solana Public Key"))
        .expect("Public key line not found");
    let pubkey = pubkey_line.split(":").nth(1).unwrap().trim();
    assert_eq!(
        pubkey.len(),
        44,
        "Solana public key should be 44 characters long"
    );
    assert!(
        pubkey.chars().all(|c| c.is_ascii_alphanumeric()),
        "Public key should only contain alphanumeric characters"
    );
}
