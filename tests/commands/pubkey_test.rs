use crate::common;
use serial_test::serial;
use std::env;
use std::fs;
use std::process::Command;
use std::str;

// cargo test --test mod -- commands::pubkey_test --nocapture

#[test]
#[serial]
fn test_pubkey_command() {
    common::setup();

    /*
    |--------------------------------------------------------------------------
    | Préparation d'un "KEYPAIR_PATH" temporaire.
    |--------------------------------------------------------------------------
    */

    // Utiliser un chemin de fichier temporaire unique pour ce test.
    let temp_keypair_path = "./storage/tests/keypair/id_temp_test_pubkey_command.json";

    // Sauvegarder l'ancienne valeur de "KEYPAIR_PATH".
    let old_keypair_path = env::var("KEYPAIR_PATH").unwrap_or_default();

    // Définir la nouvelle valeur de KEYPAIR_PATH pour ce test.
    env::set_var("KEYPAIR_PATH", temp_keypair_path);

    /*
    |--------------------------------------------------------------------------
    | Étape 1/2 : Génération de la clé publique avec "recover_seed"
    |--------------------------------------------------------------------------
    */

    let recover_output = Command::new("cargo")
        .args(["run", "--", "generate_seed"])
        .output()
        .expect("Failed to 'recover_output' execute command");

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
    println!("DEBUG recovered_pubkey: {}", recovered_pubkey);

    common::verify_pubkey(recovered_pubkey);

    /*
    |--------------------------------------------------------------------------
    | Étape 2/2 : Récupération de la clé publique avec "pubkey"
    |--------------------------------------------------------------------------
    */

    // Exécute la commande "pubkey" pour récupérer la clé publique depuis le fichier keypair.
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
    println!("DEBUG show_pubkey: {}", show_pubkey);

    common::verify_pubkey(show_pubkey);

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

    /*
    |--------------------------------------------------------------------------
    | Nettoyage de "KEYPAIR_PATH" : reset env var comme avant
    |--------------------------------------------------------------------------
    */

    // Nettoyage : supprimer le fichier temporaire et restaurer l'ancienne valeur de "KEYPAIR_PATH".
    fs::remove_file(temp_keypair_path).expect("Failed to remove temporary keypair file");
    env::set_var("KEYPAIR_PATH", old_keypair_path);
}
