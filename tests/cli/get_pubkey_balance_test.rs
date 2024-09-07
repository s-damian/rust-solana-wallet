use crate::common;
use std::process::Command;
use std::str;

#[test]
fn test_get_pubkey_balance_command() {
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

    // Vérifie que la sortie contient le mot "Balance".
    assert!(
        output_str.contains("Balance:"),
        "Error: Balance not found in output"
    );
    // Vérifie que la sortie contient le mot "SOL".
    assert!(output_str.contains("SOL"), "Error: SOL not found in output");
    // Vérifie que la sortie contient le mot "lamports".
    assert!(
        output_str.contains("lamports"),
        "Error: lamports not found in output"
    );

    // Extrait les informations de solde.
    let balance_line = output_str
        .lines()
        .find(|line| line.starts_with("Balance:"))
        .expect("Balance line not found");

    // Vérifie le format du solde (SOL et lamports).
    let balance_parts: Vec<&str> = balance_line.split_whitespace().collect();

    // Vérifie que le solde en SOL est un nombre à virgule flottante valide.
    let sol_balance = balance_parts[1]
        .parse::<f64>()
        .expect("Failed to parse SOL balance as a float");
    assert!(sol_balance >= 0.0, "SOL balance should be non-negative");

    // Vérifie que le solde en lamports est un entier valide.
    let lamports = balance_parts[3]
        .trim_end_matches(')')
        .trim_start_matches('(');
    let lamports_balance = lamports
        .parse::<u64>()
        .expect("Failed to parse lamports balance as an integer");

    // Vérifie la cohérence entre le solde en SOL et en lamports.
    assert_eq!(
        (sol_balance * 1_000_000_000.0).round() as u64,
        lamports_balance,
        "SOL and lamports balance are inconsistent"
    );

    println!("Test passed successfully. Balance output: {}", balance_line);
}
