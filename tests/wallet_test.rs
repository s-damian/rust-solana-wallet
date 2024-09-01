mod common;
use rust_solana_wallet::app::wallet_manager::WalletManager;
use rust_solana_wallet::config::wallet_config::WalletConfig;

#[test]
fn test_get_balance_by_pubkey() {
    common::setup();

    let config = WalletConfig {
        nb_derivations: 1,
        keypair_path: "./storage/tests/keypair/id.json".to_string(),
        keypair_dir: "./storage/tests/keypair/derived".to_string(),
        rpc_url: "https://api.testnet.solana.com".to_string(),
    };
    let wallet_manager = WalletManager::new(config);
    let pubkey = "FTGJPL5hia749v3jhNWJA7uE2VoVGyofB7BBL2cLwoPc";

    match wallet_manager.get_balance_by_pubkey(pubkey) {
        Ok(balance) => assert!(balance == 0, "Balance should be zero"),
        Err(e) => panic!("Failed to retrieve balance: {}", e),
    }
}
