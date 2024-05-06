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
            .subcommand(self.configure_generate_seed())
            .subcommand(self.configure_from_mnemonic())
            .subcommand(self.configure_get_pubkey_from_keypair_file())
            .subcommand(self.configure_get_pubkey_balance())
            .subcommand(self.configure_send())
    }

    fn configure_generate_seed(&self) -> Command {
        Command::new("generate_seed").about("Generates a new random mnemonic")
    }

    fn configure_from_mnemonic(&self) -> Command {
        Command::new("from_mnemonic")
            .about("Generates a mnemonic from a specified phrase")
            .arg(
                Arg::new("PHRASE")
                    .help("A mnemonic phrase")
                    .required(true)
                    .index(1),
            )
    }

    fn configure_get_pubkey_from_keypair_file(&self) -> Command {
        Command::new("get_pubkey_from_keypair_file")
            .about("Displays the public key from the keypair stored in file")
    }

    fn configure_get_pubkey_balance(&self) -> Command {
        Command::new("get_pubkey_balance")
            .about("Displays the balance for the public key")
            .arg(
                Arg::new("PUBKEY")
                    .help("A public key")
                    .required(true)
                    .index(1),
            )
    }

    fn configure_send(&self) -> Command {
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
            )
    }

    pub fn handle_matches(&self, matches: ArgMatches) {
        match matches.subcommand() {
            Some(("generate_seed", _)) => self.handle_generate_seed(),
            Some(("from_mnemonic", sub_matches)) => self.handle_from_mnemonic(sub_matches),
            Some(("get_pubkey_from_keypair_file", _)) => self.get_pubkey_from_keypair_file(),
            Some(("get_pubkey_balance", sub_matches)) => {
                self.handle_get_pubkey_balance(sub_matches)
            }
            Some(("send", sub_matches)) => self.handle_send(sub_matches),
            _ => println!("Unknown command."),
        }
    }

    fn handle_generate_seed(&self) {
        let wallet_manager = WalletManager::new(self.config.clone());
        wallet_manager.generate_and_print_random_mnemonic();
    }

    fn handle_from_mnemonic(&self, sub_matches: &ArgMatches) {
        if let Some(phrase) = sub_matches.get_one::<String>("PHRASE") {
            let wallet_manager = WalletManager::new(self.config.clone());
            wallet_manager.generate_and_print_mnemonic_from_phrase(phrase);
        }
    }

    fn get_pubkey_from_keypair_file(&self) {
        let keypair_manager = KeypairManager::new(self.config.clone());
        keypair_manager.get_pubkey_from_keypair_file();
    }

    fn handle_get_pubkey_balance(&self, sub_matches: &ArgMatches) {
        if let Some(pubkey) = sub_matches.get_one::<String>("PUBKEY") {
            let wallet_manager = WalletManager::new(self.config.clone());
            match wallet_manager.get_balance_by_pubkey(pubkey) {
                Ok(balance) => {
                    let sol_value = balance as f64 / 1_000_000_000_f64; // Convertir les lamports en SOL.
                    println!("Balance: {:.9} SOL ({} lamports)", sol_value, balance);
                }
                Err(e) => println!("Failed to retrieve balance: {}", e),
            }
        }
    }

    fn handle_send(&self, sub_matches: &ArgMatches) {
        let transaction_manager = TransactionManager::new(self.config.clone());
        match transaction_manager.send_transaction(sub_matches) {
            Ok(_) => println!("Transaction sent successfully!"),
            Err(e) => println!("Failed to send transaction: {}", e),
        }
    }
}
