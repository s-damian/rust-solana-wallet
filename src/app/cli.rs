use crate::app::keypair_manager::KeypairManager;
use crate::app::transaction_manager::TransactionManager;
use crate::app::wallet_manager::WalletManager;
use crate::config::wallet_config::WalletConfig;
use clap::{Arg, ArgMatches, Command};

pub fn setup_cli() -> Command {
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
        .subcommand(
            Command::new("send")
                .about("Send SOL to a specific address")
                .arg(
                    Arg::new("RECIPIENT")
                        .help("The recipient's public key")
                        .required(true),
                )
                .arg(
                    Arg::new("AMOUNT")
                        .help("The amount of SOL to send")
                        .required(true),
                ),
        )
}

pub fn handle_matches(matches: ArgMatches, wallet_config: &WalletConfig) {
    let wallet_manager = WalletManager::new(wallet_config.clone());
    let keypair_manager = KeypairManager::new(wallet_config.clone());

    match matches.subcommand() {
        Some(("generate_seed", _)) => {
            wallet_manager.generate_and_print_random_mnemonic();
        }
        Some(("from_mnemonic", sub_matches)) => {
            if let Some(phrase) = sub_matches.get_one::<String>("PHRASE") {
                wallet_manager.generate_and_print_mnemonic_from_phrase(phrase);
            }
        }
        Some(("get_pubkey_from_keypair_file", _)) => {
            keypair_manager.get_pubkey_from_keypair_file();
        }
        Some(("send", sub_matches)) => {
            TransactionManager::send_transaction(sub_matches, wallet_config);
        }
        _ => println!("Unknown command."),
    }
}
