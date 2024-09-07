use crate::common;
use std::process::Command;
use std::str;

#[test]
fn test_send_command() {
    common::setup();

    let recipient = "EMLY3VvNZ41yMWyPQy2AiEfJTPpZdzeGNG5zaaq3Lihb";
    let amount = "2000000";

    let output = Command::new("cargo")
        .args(["run", "--", "send", recipient, amount])
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");

    // Vérifiez si la sortie contient soit le message de simulation, soit le message d'erreur attendu
    assert!(
        output_str.contains("Simulating transaction:")
            || output_str.contains("Failed to send transaction:"),
        "Unexpected output: {}",
        output_str
    );

    // Si la transaction a échoué à cause d'un manque de fonds, c'est un résultat acceptable pour un test
    if output_str.contains("Failed to send transaction:") {
        assert!(
            output_str
                .contains("Attempt to debit an account but found no record of a prior credit."),
            "Unexpected error message: {}",
            output_str
        );
    } else {
        // Si la simulation a réussi, vérifiez le message de succès
        assert!(
            output_str.contains("Transaction sent successfully!"),
            "Expected successful transaction, got: {}",
            output_str
        );
    }
}
