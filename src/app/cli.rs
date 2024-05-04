use crate::app::keypair_manager::KeypairManager;
use crate::app::transaction_manager::TransactionManager;
use crate::app::wallet_manager::WalletManager;
use crate::config::wallet_config::WalletConfig;
use clap::{Arg, ArgMatches, Command};

pub struct AppCli {
    config: WalletConfig,
}

impl AppCli {
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    pub fn setup_cli(&self) -> Command {
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
                Command::new("get_pubkey_balance")
                    .about("Displays the balance for the public key")
                    .arg(
                        Arg::new("PUBKEY")
                            .help("A public key")
                            .required(true)
                            .index(1),
                    ),
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

    pub fn handle_matches(&self, matches: ArgMatches) {
        let wallet_manager = WalletManager::new(self.config.clone());

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
                let keypair_manager = KeypairManager::new(self.config.clone());
                keypair_manager.get_pubkey_from_keypair_file();
            }
            Some(("get_pubkey_balance", sub_matches)) => {
                if let Some(pubkey) = sub_matches.get_one::<String>("PUBKEY") {
                    match wallet_manager.get_balance_by_pubkey(pubkey) {
                        Ok(balance) => {
                            let sol_value = balance as f64 / 1_000_000_000_f64; // Convertir les lamports en SOL.
                            println!("Balance: {:.9} SOL ({} lamports)", sol_value, balance);
                        }
                        Err(e) => println!("Failed to retrieve balance: {}", e),
                    }
                }
            }
            Some(("send", sub_matches)) => {
                let transaction_manager = TransactionManager::new(self.config.clone());
                match transaction_manager.send_transaction(sub_matches) {
                    Ok(_) => println!("Transaction sent successfully!"),
                    Err(e) => println!("Failed to send transaction: {}", e),
                }
            }
            _ => println!("Unknown command."),
        }
    }
}
