use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use neptune_cash::prelude::tasm_lib::prelude::Digest;
use tracing::debug;

use crate::rpc_client;
use crate::wallet::wallet_block::WalletBlock;

impl super::WalletState {
    /// Return (height, digest) of latest ancestor that's canonical.
    ///
    /// Assume this wallet's tip is a block of height 12, and a reorganization
    /// of depth 2 has just happened, making the known blocks with height 12
    /// and 11 orphanes. Then this function will return (10, block_10.hash()).
    pub async fn check_fork(&self, block: &WalletBlock) -> Result<Option<(u64, Digest)>> {
        let (mut parent_hash, mut parent_height) = (block.kernel.header.prev_block_digest, block.kernel.header.height.previous().unwrap());

        loop {
            let is_canonical = rpc_client::node_rpc_client().is_block_canonical(parent_hash).await.context("try is_block_canonical")?;
            if is_canonical {
                return Ok(Some((parent_height.value(), parent_hash)));
            }



        }
        // if block.kernel.header.height.value() <= 1 {
        //     return Ok(None);
        // }

        // debug!(
        //     "Checking fork for block {} {} {}",
        //     block.kernel.header.height,
        //     block.hash().to_hex(),
        //     block.kernel.header.prev_block_digest.to_hex()
        // );

        // if let Some((prev_height, prev_digest)) = self.get_tip().await.context("get tip")? {
        //     debug!("prev digest: {} {:?}", prev_height, prev_digest.to_hex());
        //     //prev is forked
        //     if block.kernel.header.prev_block_digest != prev_digest {
        //         let mut prev_digest = block.kernel.header.prev_block_digest;
        //         loop {
        //             let prev = rpc_client::node_rpc_client()
        //                 .get_block_info(&prev_digest.to_hex())
        //                 .await
        //                 .context("try get_prev_block_info")?;

        //             match prev {
        //                 Some(prev) => {
        //                     if prev.is_canonical {
        //                         let blk_before_fork = rpc_client::node_rpc_client()
        //                             .get_block_info(&prev.prev_block_digest.to_hex())
        //                             .await?
        //                             .context("try get_block_info before fork")?;
        //                         return Ok(Some((
        //                             blk_before_fork.height.into(),
        //                             blk_before_fork.digest,
        //                         )));
        //                     } else {
        //                         prev_digest = prev.prev_block_digest;
        //                     }
        //                 }
        //                 None => return Err(anyhow!("Block not found")),
        //             }
        //         }
        //     }
        // }
        // Ok(None)
    }
}
