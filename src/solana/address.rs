use solana_sdk::signature::{keypair_from_seed, write_keypair_file};
use solana_sdk::signer::keypair::Keypair;

// Fénerer une paire de clés Solana (clé publique et clé privée) à partir d'une seed.
pub fn generate_keypair(seed_bytes: &[u8]) -> Keypair {
    keypair_from_seed(seed_bytes).unwrap()
}

// Elle écrit la paire de clés générée dans un fichier.
// Ce fichier peut être utilisé pour stocker la clé de manière sécurisée ou pour l'importer dans d'autres applications ou services compatibles avec Solana.
pub fn write_keypair(keypair: &Keypair, file_path: &str) {
    write_keypair_file(keypair, file_path).unwrap();
}
