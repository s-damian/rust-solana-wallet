use crate::config::wallet_config::WalletConfig;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub struct SolanaBalance {
    config: WalletConfig,
}

impl SolanaBalance {
    /// Crée une nouvelle instance de `SolanaBalance` avec une URL RPC spécifiée.
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    /// Retourne la balance en SOL pour une clé publique donnée.
    ///
    /// Arguments:
    /// - pubkey_str - La clé publique sous forme de chaîne de caractères.
    ///
    /// # Returns:
    /// - Ok(balance) - Si la requête est réussie.
    /// - Err(e) - Si une erreur se produit lors de la récupération de la balance.
    pub fn get_balance_by_pubkey(&self, pubkey: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let pubkey = Pubkey::from_str(pubkey)?;
        let client = RpcClient::new(&self.config.rpc_url);
        client.get_balance(&pubkey).map_err(Into::into)
    }
}
