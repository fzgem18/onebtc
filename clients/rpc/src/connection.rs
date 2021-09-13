use crate::error::{Error, Web3Error};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use web3::{transports::WebSocket, Web3};

const RETRY_TIMEOUT: Duration = Duration::from_millis(1000);

pub(crate) async fn new_websocket_client(url: &str) -> Result<Web3<WebSocket>, Error> {
    let transport = WebSocket::new(url).await?;
    let ws_client = Web3::new(transport);
    Ok(ws_client)
}

pub(crate) async fn new_websocket_client_with_retry(
    url: &str,
    connection_timeout: Duration,
) -> Result<Web3<WebSocket>, Error> {
    log::info!("Connecting to the oneBTC...");
    timeout(connection_timeout, async move {
        loop {
            match new_websocket_client(url).await {
                Err(Error::Web3Error(Web3Error::Transport(err))) => {
                    log::trace!("could not connect to oneBTC: {}", err);
                    sleep(RETRY_TIMEOUT).await;
                    continue;
                }
                Ok(rpc) => {
                    log::info!("Connected!");
                    return Ok(rpc);
                }
                Err(err) => return Err(err),
            }
        }
    })
    .await?
}
