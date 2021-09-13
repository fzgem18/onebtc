mod connection;
mod error;
mod retry;
mod rpc;

pub use error::Error;
pub use retry::{notify_retry, RetryPolicy};

pub use rpc::OneBtcRpc;

pub const BTC_RELAY_MODULE: &str = "BTCRelay";
pub const ISSUE_MODULE: &str = "Issue";
pub const REDEEM_MODULE: &str = "Redeem";

pub const DUPLICATE_BLOCK_ERROR: &str = "DuplicateBlock";
pub const INVALID_CHAIN_ID_ERROR: &str = "InvalidChainID";
pub const ISSUE_COMPLETED_ERROR: &str = "IssueCompleted";
pub const COMMIT_PERIOD_EXPIRED_ERROR: &str = "CommitPeriodExpired";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
