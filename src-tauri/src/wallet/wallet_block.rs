use neptune_cash::api::export::Digest;
use neptune_cash::protocol::consensus::block::block_kernel::BlockKernel;
use neptune_cash::application::json_rpc::core::model::wallet::block::RpcWalletBlock;
use serde::Serialize;
use serde::Deserialize;

/// A block tailored for this program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct WalletBlock {
    pub(crate) kernel: BlockKernel,
    pub(crate) hash: Digest,
}

impl From<RpcWalletBlock> for WalletBlock {
    fn from(value: RpcWalletBlock) -> Self {
        let hash = value.hash();
        Self {
            kernel: value.kernel.into(),
            hash,
        }
    }
}