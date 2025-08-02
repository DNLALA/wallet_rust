use thiserror::Error;
use wallet_core_rs::tw_coin_registry;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Mnemonic generation failed")]
    MnemonicError,
    #[error("Wallet derivation failed for coin {0}")]
    DerivationError(String),
}

pub struct WalletService;

impl WalletService {
    pub fn generate_mnemonic(word_count: usize) -> Result<String, WalletError> {
        Mnemonic::generate(word_count)
            .map(|m| m.to_string())
            .map_err(|_| WalletError::MnemonicError)
    }

    pub fn derive_wallet(mnemonic: &str, coin: CoinType) -> Result<(String, String), WalletError> {
        let wallet = HDWallet::new(mnemonic, "")
            .map_err(|_| WalletError::DerivationError(coin.symbol().to_string()))?;

        let private_key = wallet
            .get_key(coin)
            .map_err(|_| WalletError::DerivationError(coin.symbol().to_string()))?;

        let address = private_key
            .get_address(coin)
            .map_err(|_| WalletError::DerivationError(coin.symbol().to_string()))?;

        Ok((private_key.to_string(), address))
    }
}
