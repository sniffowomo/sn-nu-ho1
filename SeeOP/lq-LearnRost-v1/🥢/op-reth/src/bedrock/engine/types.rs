use reth_primitives::Bytes;
use reth_rpc_types::engine::PayloadAttributes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BedrockPayloadAttributes {
    #[serde(flatten)]
    inner: PayloadAttributes,
    /// Transactions is a field for rollups: the transactions list is forced into the block
    #[serde(skip_serializing_if = "Option::is_none")]
    transactions: Option<Vec<Bytes>>,
    /// NoTxPool is a field for rollups: if true, the no transactions are taken out of the tx-pool,
    /// only transactions from the above Transactions list will be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    no_tx_pool: Option<bool>,
    /// GasLimit is a field for rollups: if set, this sets the exact gas limit the block produced
    /// with.
    #[serde(skip_serializing_if = "Option::is_none")]
    gas_limit: Option<u64>,
}
