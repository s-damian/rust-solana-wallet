use rust_solana_wallet::app::wallet_manager::WalletManager;
use rust_solana_wallet::config::wallet_config::WalletConfig;

#[test]
fn test_get_balance_by_pubkey() {
    let config = WalletConfig {
        nb_derivations: 1,
        keypair_path: "./storage/tests/keypair/id.json".to_string(),
        keypair_dir: "./storage/tests/keypair/derived".to_string(),
        rpc_url: "https://api.testnet.solana.com".to_string(),
    };
    let wallet_manager = WalletManager::new(config);
    let pubkey = "FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc";

    match wallet_manager.get_balance_by_pubkey(pubkey) {
        Ok(balance) => {
            println!("Retrieved balance: {}", balance); // Ajout de cette ligne pour le debugging
            assert!(balance > 0, "Balance should be positive");
        }
        Err(e) => panic!("Failed to retrieve balance: {}", e),
    }
}
