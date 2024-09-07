use crate::common;
use std::process::Command;
use std::str;

#[test]
fn test_get_pubkey_balance() {
    common::setup();

    // Clé publique Solana à utiliser dans le test.
    let pubkey = "FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc";

    // Exécute la commande "get_pubkey_balance" avec la clé publique donnée.
    let output = Command::new("cargo")
        .args(["run", "--", "get_pubkey_balance", pubkey])
        .output()
        .expect("Failed to execute command");

    // Vérifie que la commande s'est exécutée avec succès.
    assert!(output.status.success(), "Error: Command failed to execute");

    // Convertit la sortie de la commande en chaîne de caractères.
    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");



    // Vérifie que la sortie contient le mot "Balance"
    assert!(
        output_str.contains("Balance:"),
        "Error: Balance not found in output"
    );

    // Vérifie que la sortie contient le mot "SOL"
    assert!(output_str.contains("SOL"), "Error: SOL not found in output");

    // Vérifie que la sortie contient le mot "lamports"
    assert!(
        output_str.contains("lamports"),
        "Error: lamports not found in output"
    );



    // Vérifie le format de la balance (par exemple: 0.000000000 SOL)
    let balance_line = output_str
        .lines()
        .find(|line| line.starts_with("Balance:"))
        .expect("Balance line not found");


    
    /*let balance_parts: Vec<&str> = balance_line.split_whitespace().collect();
    let sol_balance = balance_parts[1].trim(); // Extraction de la balance en SOL
    let lamports_balance = balance_parts[3].trim(); // Extraction de la balance en lamports

    // Vérifie que la balance en SOL est au format numérique
    assert!(
        sol_balance.parse::<f64>().is_ok(),
        "SOL balance is not a valid number"
    );

    // Vérifie que la balance en lamports est un nombre entier
    assert!(
        lamports_balance.parse::<u64>().is_ok(),
        "Lamports balance is not a valid integer"
    );*/


    
}
