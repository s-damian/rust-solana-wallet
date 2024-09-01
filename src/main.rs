mod app;
mod bip;
mod config;
mod solana;

use app::cli::AppCli;
use config::wallet_config::WalletConfig;

#[cfg(not(test))]
use dotenv::dotenv;

fn main() {
    // Charger les variables d'environnement depuis le fichier ".env" (mais seulement si nous ne sommes pas en mode test).
    #[cfg(not(test))]
    dotenv().ok();

    // Charger la configuration du wallet (selon les données du fichier .env).
    let config = WalletConfig::new();

    let app_cli = AppCli::new(config.clone());

    let matches = app_cli.setup_cli().get_matches();

    app_cli.handle_matches(matches);
}
