use std::io;

/// Suggest that the user add a passprase to the seed (leave empty to not use any).
pub fn prompt_for_passphrase() -> String {
    println!("Enter your passphrase:");

    let mut passphrase = String::new();
    io::stdin()
        .read_line(&mut passphrase)
        .expect("Failed to read passphrase");

    passphrase.trim().to_string() // Return the passphrase.
}
