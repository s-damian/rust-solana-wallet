use crate::common;
use std::process::Command;
use std::str;

#[test]
fn test_pubkey_command() {
    common::setup();

    /*
    |--------------------------------------------------------------------------
    | Étape 1/2 : Génération de la clé publique avec "recover_seed"
    |--------------------------------------------------------------------------
    */

    // Étape 1 : Exécute la commande "recover_seed" pour générer une clé publique et stocker la keypair dans le fichier.
    let mnemonic = "shed scorpion manual wheat monster phone winter toe dream kitchen salad column";
    let recover_output = Command::new("cargo")
        .args(["run", "--", "recover_seed", mnemonic])
        .output()
        .expect("Failed to execute 'recover_seed' command");

    // Vérifie que la commande "recover_seed" s'est exécutée avec succès.
    assert!(
        recover_output.status.success(),
        "Error: 'recover_seed' command failed to execute"
    );

    // Extraire la public key générée.
    let recover_pubkey_output_str =
        str::from_utf8(&recover_output.stdout).expect("Invalid UTF-8 output");
    let recovered_pubkey_line = recover_pubkey_output_str
        .lines()
        .find(|line| line.starts_with("Solana Public Key"))
        .expect("Public key line not found");
    let recovered_pubkey = recovered_pubkey_line.split(':').nth(1).unwrap().trim();

    assert!(
        (32..=44).contains(&recovered_pubkey.len()),
        "Solana public key should be between 32 and 44 characters long, but it's {} characters long",
        recovered_pubkey.len()
    );

    /*
    |--------------------------------------------------------------------------
    | Étape 2/2 : Récupération de la clé publique avec "pubkey"
    |--------------------------------------------------------------------------
    */

    // Étape 2 : Exécute la commande "pubkey" pour récupérer la clé publique depuis le fichier keypair.
    let pubkey_output = Command::new("cargo")
        .args(["run", "--", "pubkey"])
        .output()
        .expect("Failed to execute 'pubkey' command");

    // Vérifie que la commande "pubkey" s'est exécutée avec succès.
    assert!(
        pubkey_output.status.success(),
        "Error: 'pubkey' command failed to execute"
    );

    // Convertit la sortie de la commande en chaîne de caractères.
    let show_pubkey_output_str =
        str::from_utf8(&pubkey_output.stdout).expect("Invalid UTF-8 output");
    let show_pubkey_line = show_pubkey_output_str
        .lines()
        .find(|line| line.starts_with("Solana Public Key"))
        .expect("Public key line not found");
    let show_pubkey = show_pubkey_line.split(':').nth(1).unwrap().trim();

    assert!(
        (32..=44).contains(&show_pubkey.len()),
        "Solana public key should be between 32 and 44 characters long, but it's {} characters long",
        show_pubkey.len()
    );

    /*
    |--------------------------------------------------------------------------
    | Vérifications finales
    |--------------------------------------------------------------------------
    */

    // Vérifier que la public key extraite correspond à celle générée précédemment.
    assert_eq!(
        show_pubkey, recovered_pubkey,
        "The public key from pubkey command doesn't match the one generated"
    );
}
