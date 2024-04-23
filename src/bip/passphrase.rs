use std::io;

/// Suggérez à l'utilisateur d'ajouter une passphrase à la seed (laisser vide pour ne pas en utiliser).
pub fn prompt_for_passphrase() -> String {
    println!("Enter your passphrase:");

    let mut passphrase = String::new();
    io::stdin()
        .read_line(&mut passphrase)
        .expect("Failed to read passphrase");

    // Retourner la passphrase.
    passphrase.trim().to_string()
}
