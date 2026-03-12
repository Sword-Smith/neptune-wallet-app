use std::sync::Arc;

use axum::extract::Path;
use axum_extra::response::ErasedJson;
use serde::Serialize;

use super::error::RestError;
use crate::rpc::WalletRpcImpl;
use crate::service::get_state;
use crate::wallet::sync::SyncState;

#[derive(Debug, Serialize)]
pub struct TransactionStatus {
    tx_id: String,
    status: TransactionStatusEnum,
}

#[derive(Debug, Serialize)]
pub enum TransactionStatusEnum {
    Pending,
    // Proving,
    // Composing,
}
pub async fn get_pending_transaction() -> Result<ErasedJson, RestError> {
    Ok(ErasedJson::pretty(
        WalletRpcImpl::pending_transactions().await?,
    ))
}

pub async fn forget_tx(Path(id): Path<String>) -> Result<ErasedJson, RestError> {
    WalletRpcImpl::forget_tx(id).await?;
    Ok(ErasedJson::pretty(true))
}

pub trait TransactionStatusRpc {
    async fn pending_transactions() -> Result<Vec<TransactionStatus>, RestError> {
        let wallet = &get_state::<Arc<SyncState>>().wallet;
        let txs = wallet.get_pending_transactions().await?;
        let mut result = vec![];
        for tx in txs {
            let status = TransactionStatus {
                tx_id: tx,
                status: TransactionStatusEnum::Pending,
            };
            result.push(status);
        }
        Ok(result)
    }
    async fn forget_tx(txid: String) -> Result<(), RestError> {
        let wallet = &get_state::<Arc<SyncState>>().wallet;
        wallet.forget_tx(&txid).await?;
        Ok(())
    }
}

impl TransactionStatusRpc for WalletRpcImpl {}
