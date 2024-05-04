use std::io;

pub struct BipPassphrase {}

impl BipPassphrase {
    /// Demande à l'utilisateur d'entrer une passphrase optionnelle pour renforcer la sécurité de la seed.
    /// Laisser vide si aucune passphrase n'est souhaitée.
    pub fn prompt_for_passphrase() -> String {
        println!("Enter passphrase (optional):");

        let mut passphrase = String::new();
        io::stdin()
            .read_line(&mut passphrase)
            .expect("Failed to read passphrase");

        // Nettoyer et retourner la passphrase saisie, en éliminant les espaces avant et après.
        passphrase.trim().to_string()
    }
}
