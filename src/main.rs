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

    // Charge la configuration du wallet (selon les données du fichier .env).
    let wallet_config = WalletConfig::new();

    let matches = setup_cli().get_matches();

    handle_matches(matches, &wallet_config);
}

fn setup_cli() -> Command {
    Command::new("Solana Wallet")
        .version("1.0.0")
        .about("Example of a Solana Wallet in Rust")
        .subcommand(Command::new("generate_seed").about("Generates a new random mnemonic"))
        .subcommand(
            Command::new("from_mnemonic")
                .about("Generates a mnemonic from a specified phrase")
                .arg(
                    Arg::new("PHRASE")
                        .help("A mnemonic phrase")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("get_pubkey_from_keypair_file")
                .about("Displays the public key from the keypair stored in file"),
        )
}

fn handle_matches(matches: clap::ArgMatches, config: &WalletConfig) {
    match matches.subcommand() {
        Some(("generate_seed", _)) => {
            generate_and_print_random_mnemonic(config);
        }
        Some(("from_mnemonic", sub_matches)) => {
            if let Some(phrase) = sub_matches.get_one::<String>("PHRASE") {
                generate_and_print_mnemonic_from_phrase(config, phrase);
            }
        }
        Some(("get_pubkey_from_keypair_file", _)) => {
            get_pubkey_from_keypair_file(config);
        }
        _ => println!("Unknown command."),
    }
}
