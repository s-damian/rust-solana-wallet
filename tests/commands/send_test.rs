use crate::common;
use serial_test::serial;
use std::process::Command;
use std::str;

#[test]
#[serial]
fn test_send_command() {
    common::setup();

    // Clé publique Solana du récepteur qui recevra  les lamports.
    let recipient = "EMLY3VvNZ41yMWyPQy2AiEfJTPpZdzeGNG5zaaq3Lihb";
    let amount = "2000000";

    // Exécute la commande "send" avec le récepteur ("recipient") et le montant en lamports ("amount").
    let output = Command::new("cargo")
        .args(["run", "--", "send", recipient, amount])
        .output()
        .expect("Failed to execute command");

    // Convertit la sortie de la commande en chaîne de caractères.
    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");

    // Vérifiez si la sortie contient soit le message de simulation.
    assert!(
        output_str.contains("Simulating transaction:"),
        "Unexpected output: {}",
        output_str
    );
    assert!(
        output_str.contains(&format!("Simulating transaction: {} lamports", amount)),
        "Output should contain the correct amount: {}",
        output_str
    );
    assert!(
        output_str.contains(recipient),
        "Output should contain the recipient address: {}",
        output_str
    );
    // Si la simulation a réussi, vérifiez le message de succès.
    assert!(
        output_str.contains("Transaction sent successfully!"),
        "Expected successful transaction, got: {}",
        output_str
    );
}
