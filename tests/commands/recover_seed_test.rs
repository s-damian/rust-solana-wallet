use crate::common;
use std::process::Command;
use std::str;

#[test]
fn test_recover_seed_command() {
    common::setup();

    let mnemonic = "shed scorpion manual wheat monster phone winter toe dream kitchen salad column";

    // Exécute la commande "recover_seed" avec la mnémonic donnée.
    let output = Command::new("cargo")
        .args(["run", "--", "recover_seed", mnemonic])
        .output()
        .expect("Failed to execute command");

    // Vérifie que la commande s'est exécutée avec succès.
    assert!(output.status.success(), "Error: Command failed to execute");

    // Convertit la sortie de la commande en chaîne de caractères.
    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");

    // Vérifie que la sortie contient les éléments attendus.
    assert!(
        output_str.contains(&format!("BIP39 Mnemonic (given phrase): {}", mnemonic)),
        "Error: Mnemonic phrase not found in output"
    );
    assert!(
        output_str.contains("Enter passphrase (optional)"),
        "Error: passphrase prompt not found in output"
    );
    assert!(
        output_str.contains("Seed:"),
        "Error: Seed not found in output"
    );
    // Vérifie que la sortie contient les clés public (dans ".env.testing" nous avons : NB_DERIVATIONS=2).
    assert!(
        output_str.contains("Solana Public Key:"),
        "Error: Public key not found in output"
    );

    // Vérifie que la sortie contient la mnémonique donnée exacte.
    let mnemonic_line = output_str
        .lines()
        .find(|line| line.starts_with("BIP39 Mnemonic (given phrase):"))
        .expect("Mnemonic line not found");
    assert!(
        mnemonic_line.contains(mnemonic),
        "The output does not contain the exact given mnemonic"
    );

    // Vérifie le format de la seed (64 caractères hexadécimaux).
    let seed_line = output_str
        .lines()
        .find(|line| line.starts_with("Seed:"))
        .expect("Seed line not found");
    let seed = seed_line.split(':').nth(1).unwrap().trim();
    assert_eq!(seed.len(), 128, "Seed should be 128 characters long");
    assert!(
        seed.chars().all(|c| c.is_ascii_hexdigit()),
        "Seed should only contain hexadecimal characters"
    );

    // Vérifie le format de la clé publique Solana (adresse entre 32 et 44 caractères base58).
    let pubkey_line = output_str
        .lines()
        .find(|line| line.starts_with("Solana Public Key"))
        .expect("Public key line not found");
    let pubkey = pubkey_line.split(':').nth(1).unwrap().trim();
    assert!(
        pubkey.len() >= 32 && pubkey.len() <= 44,
        "Solana public key should be between 32 and 44 characters long, but it's {} characters long",
        pubkey.len()
    );
    assert!(
        pubkey.chars().all(|c| c.is_ascii_alphanumeric()),
        "Public key should only contain alphanumeric characters"
    );
}
