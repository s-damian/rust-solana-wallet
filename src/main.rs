mod app;
mod bip;
mod config;
mod solana;

use app::cli::{handle_matches, setup_cli};
use config::wallet_config::WalletConfig;
use dotenv::dotenv;

fn main() {
    // Charge les variables d'environnement depuis le fichier ".env".
    dotenv().ok();

    // Charge la configuration du wallet (selon les données du fichier .env).
    let config = WalletConfig::new();

    let matches = setup_cli().get_matches();

    handle_matches(matches, &config);
}
