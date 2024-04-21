use solana_sdk::signature::{keypair_from_seed, write_keypair_file};
use solana_sdk::signer::keypair::Keypair;
use std::fs;
use std::io::{self, Error, ErrorKind};

// Fénerer une paire de clés Solana (clé publique et clé privée) à partir d'une seed.
pub fn generate_keypair(seed_bytes: &[u8]) -> Keypair {
    keypair_from_seed(seed_bytes).unwrap()
}

// Écrire la paire de clés générée dans un fichier.
// Ce fichier peut être utilisé pour stocker la clé de manière sécurisée ou pour l'importer dans d'autres applications ou services compatibles avec Solana.
pub fn write_keypair(keypair: &Keypair, file_path: &str) {
    write_keypair_file(keypair, file_path).unwrap();
}

pub fn read_keypair_from_file(file_path: &str) -> io::Result<Keypair> {
    let content = fs::read_to_string(file_path)?;
    // Retirer les crochets et espaces blancs, puis diviser par les virgules
    let cleaned_content = content.trim_matches(|c: char| c == '[' || c == ']' || c.is_whitespace());
    let bytes: Result<Vec<u8>, _> = cleaned_content.split(',')
        .map(|s| s.trim().parse::<u8>())
        .collect();

    match bytes {
        Ok(bytes) => Keypair::from_bytes(&bytes)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to parse keypair")),
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Invalid byte format in file")),
    }
}
