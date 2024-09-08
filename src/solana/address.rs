use solana_sdk::signature::{keypair_from_seed, write_keypair_file};
use solana_sdk::signer::keypair::Keypair;
use std::fs;
use std::io::{self, Error, ErrorKind};

pub struct SolanaAddress {}

impl SolanaAddress {
    /// Génère une paire de clés Solana (clé publique et clé privée) à partir d'octets de seed.
    ///
    /// # Arguments:
    /// - seed_bytes - Les octets de la seed à partir de laquelle la paire de clés est générée.
    ///
    /// # Returns:
    /// Retourne la paire de clés générée ou interrompt l'exécution en cas d'erreur.
    pub fn generate_keypair(seed_bytes: &[u8]) -> Keypair {
        keypair_from_seed(seed_bytes).expect("Failed to generate keypair to file")
    }

    /// Écrit la paire de clés générée dans un fichier.
    /// La paire de clés, contenant la clé publique et la clé privée, est encodée en JSON.
    /// Le fichier créé possède des permissions restreintes (0o600) sur les systèmes Unix, assurant une sécurité accrue.
    ///
    /// # Arguments:
    /// - keypair - La paire de clés à écrire.
    /// - file_path - Le chemin du fichier où écrire la paire de clés.
    ///   @note Ce fichier peut être utilisé pour stocker de manière sécurisée la paire de clés ou pour l'importer dans d'autres applications ou services compatibles avec Solana.
    pub fn write_keypair(keypair: &Keypair, file_path: &str) {
        write_keypair_file(keypair, file_path).expect("Failed to write keypair to file");
    }

    /// Lit une paire de clés à partir d'un fichier et la retourne.
    ///
    /// # Arguments:
    /// @param file_path Le chemin du fichier contenant la paire de clés.
    ///
    /// # Returns:
    /// Retourne une paire de clés si la lecture et l'interprétation des données sont réussies, sinon une erreur.
    pub fn read_keypair_from_file(file_path: &str) -> io::Result<Keypair> {
        let content = fs::read_to_string(file_path)?;
        // Nettoie le contenu du fichier en retirant les crochets, espaces et en séparant les éléments par virgules.
        let cleaned_content =
            content.trim_matches(|c: char| c == '[' || c == ']' || c.is_whitespace());
        let bytes: Result<Vec<u8>, _> = cleaned_content
            .split(',')
            .map(|s| s.trim().parse::<u8>())
            .collect();

        match bytes {
            Ok(bytes) => Keypair::from_bytes(&bytes)
                .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to parse keypair")),
            Err(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid byte format in file",
            )),
        }
    }
}
