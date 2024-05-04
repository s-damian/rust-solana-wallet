use crate::app::create_wallet::{
    generate_and_print_mnemonic_from_phrase, generate_and_print_random_mnemonic,
};
use crate::app::keypair_file::get_pubkey_from_keypair_file;
use crate::app::transaction::send_lamports;
use crate::config::wallet_config::WalletConfig;
use clap::{Arg, ArgMatches, Command};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use std::str::FromStr;

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
    match matches.subcommand() {
        Some(("generate_seed", _)) => {
            generate_and_print_random_mnemonic(wallet_config);
        }
        Some(("from_mnemonic", sub_matches)) => {
            if let Some(phrase) = sub_matches.get_one::<String>("PHRASE") {
                generate_and_print_mnemonic_from_phrase(phrase, wallet_config);
            }
        }
        Some(("get_pubkey_from_keypair_file", _)) => {
            get_pubkey_from_keypair_file(wallet_config);
        }
        Some(("send", sub_matches)) => {
            let recipient = sub_matches
                .get_one::<String>("RECIPIENT")
                .expect("Recipient required");
            let amount = sub_matches
                .get_one::<String>("AMOUNT")
                .expect("Amount required")
                .parse::<u64>()
                .expect("Amount needs to be a number");

            let recipient_pubkey = Pubkey::from_str(recipient).expect("Invalid public key format");
            let sender_keypair_path = &wallet_config.keypair_path;

            let sender_keypair =
                read_keypair_file(sender_keypair_path).expect("Failed to read keypair from file");

            match send_lamports(
                &wallet_config.rpc_url,
                &sender_keypair,
                &recipient_pubkey,
                amount,
            ) {
                Ok(_) => println!("Transaction sent successfully!"),
                Err(e) => println!("Failed to send transaction: {}", e),
            }
        }
        _ => println!("Unknown command."),
    }
}
