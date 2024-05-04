mod app;
mod bip;
mod config;
mod solana;

use app::cli::AppCli;
use config::wallet_config::WalletConfig;
use dotenv::dotenv;

fn main() {
    // Charge les variables d'environnement depuis le fichier ".env".
    dotenv().ok();

    // Charge la configuration du wallet (selon les données du fichier .env).
    let config = WalletConfig::new();

    let app_cli = AppCli::new(config.clone());

    let matches = app_cli.setup_cli().get_matches();

    app_cli.handle_matches(matches);
}
