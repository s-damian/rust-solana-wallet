mod app;
mod bip;
mod config;
mod solana;

use app::create_wallet::{
    generate_and_print_mnemonic_from_phrase, generate_and_print_random_mnemonic,
};
use app::keypair_file::get_pubkey_from_keypair_file;
use clap::{Arg, Command};
use config::wallet_config::WalletConfig;
use dotenv::dotenv;

fn main() {
    // Charge les variables d'environnement depuis le fichier ".env".
    dotenv().ok();

    let wallet_config = WalletConfig::new();

    let matches = Command::new("Solana Wallet")
        .version("1.0.0")
        .about("Example of a Solana Wallet in Rust")
        .subcommand(Command::new("generate_seed").about("Generates a new random mnemonic"))
        .subcommand(
            Command::new("from_mnemonic")
                .about("Generates a mnemonic from a specified phrase")
                .arg(
                    Arg::new("PHRASE")
                        .help("A 12-word mnemonic phrase")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("get_pubkey_from_keypair_file")
                .about("Displays the public key from the keypair stored in file"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate_seed", _sub_matches)) => {
            generate_and_print_random_mnemonic(&wallet_config);
        }
        Some(("from_mnemonic", sub_matches)) => {
            if let Some(phrase) = sub_matches.get_one::<String>("PHRASE") {
                generate_and_print_mnemonic_from_phrase(&wallet_config, phrase);
            }
        }
        Some(("get_pubkey_from_keypair_file", _sub_matches)) => {
            get_pubkey_from_keypair_file(&wallet_config);
        }
        _ => println!("Unknown command."),
    }
}
