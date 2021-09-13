// use crate::{
//     BTC_RELAY_MODULE, COMMIT_PERIOD_EXPIRED_ERROR, DUPLICATE_BLOCK_ERROR, INVALID_CHAIN_ID_ERROR,
//     ISSUE_COMPLETED_ERROR, ISSUE_MODULE, REDEEM_MODULE,
// };
use jsonrpc_core::error::Error as RPCError;
use serde_json::Error as SerdeJsonError;
use std::{array::TryFromSliceError, io::Error as IoError, num::TryFromIntError};
use thiserror::Error;
use tokio::time::error::Elapsed;
use url::ParseError as UrlParseError;
pub use web3::{error::Error as Web3Error, signing::RecoveryError};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not get exchange rate info")]
    ExchangeRateInfo,
    #[error("Could not get issue id")]
    RequestIssueIDNotFound,
    #[error("Could not get redeem id")]
    RequestRedeemIDNotFound,
    #[error("Could not get replace id")]
    RequestReplaceIDNotFound,
    #[error("Could not get block")]
    BlockNotFound,
    #[error("Could not get vault")]
    VaultNotFound,
    #[error("Vault has been liquidated")]
    VaultLiquidated,
    #[error("Vault has stolen BTC")]
    VaultCommittedTheft,
    #[error("Channel closed unexpectedly")]
    ChannelClosed,
    #[error("Transaction is invalid")]
    InvalidTransaction,
    #[error("Request has timed out")]
    Timeout,
    #[error("Block is not in the relay main chain")]
    BlockNotInRelayMainChain,

    #[error("Failed to load credentials from file: {0}")]
    KeyLoadingFailure(#[from] KeyLoadingError),
    #[error("Error serializing: {0}")]
    Serialize(#[from] TryFromSliceError),
    #[error("Error converting: {0}")]
    Convert(#[from] TryFromIntError),
    #[error("Error communicating with ethereum: {0}")]
    Web3Error(#[from] Web3Error),
    #[error("Error encoding json data: {0}")]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("Error getting json-rpc data: {0}")]
    JsonRpcError(#[from] RPCError),
    #[error("Timeout: {0}")]
    TimeElapsed(#[from] Elapsed),
    #[error("UrlParseError: {0}")]
    UrlParseError(#[from] UrlParseError),
}

#[derive(Error, Debug)]
pub enum KeyLoadingError {
    #[error("Key not found in file")]
    KeyNotFound,
    #[error("Json parsing error: {0}")]
    JsonError(#[from] SerdeJsonError),
    #[error("Io error: {0}")]
    IoError(#[from] IoError),
}

impl Error {
    pub fn is_rpc_error(&self) -> bool {
        matches!(self, Error::Web3Error(Web3Error::Rpc(_)))
    }

    pub fn is_recovery_error(&self) -> bool {
        matches!(
            self,
            Error::Web3Error(Web3Error::Recovery(RecoveryError::InvalidSignature))
        )
    }
}
