use crate::{connection::*, error::Error};
use ethereum_types::Address;
use std::time::Duration;
use web3::{transports::WebSocket, Web3};

#[derive(Clone)]
pub struct OneBtcRpc {
    web3_client: Web3<WebSocket>,
    account_id: Address,
}

impl OneBtcRpc {
    pub async fn new(web3_client: Web3<WebSocket>, account_id: Address) -> Result<Self, Error> {
        let onebtc_rpc = Self {
            web3_client,
            account_id,
        };
        Ok(onebtc_rpc)
    }

    pub async fn from_url(url: &str, account_id: Address) -> Result<Self, Error> {
        let web3_client = new_websocket_client(url).await?;
        Self::new(web3_client, account_id).await
    }

    pub async fn from_url_with_retry(
        url: &str,
        account_id: Address,
        connection_timeout: Duration,
    ) -> Result<Self, Error> {
        let web3_client = new_websocket_client_with_retry(url, connection_timeout).await?;
        Self::new(web3_client, account_id).await
    }
}
