use crate::types::{Bytes, H160, H256, H64, U256, U64};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

// The consensus status returned from RPC calls.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConsensusStatus {
    state: State,
    validator: bool,
}

/// The view state
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct State {
    view: View,
    #[serde(rename = "highestCommitBlock")]
    committed: QCBlock,
    #[serde(rename = "highestLockBlock")]
    locked: QCBlock,
    #[serde(rename = "highestQCBlock")]
    qc: QCBlock,
}

/// View
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct View {
    epoch: u64,
    #[serde(rename = "viewNumber")]
    view: u64,
}

/// QC Block
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct QCBlock {
    hash: H256,
    number: u64,
}
