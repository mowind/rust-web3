use crate::types::{Bytes, H160, H256, H64, U256, U64};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

// The consensus status returned from RPC calls.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConsensusStatus {
    pub state: State,
    pub validator: bool,
}

/// The view state
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub view: View,
    #[serde(rename = "highestCommitBlock")]
    pub committed: QCBlock,
    #[serde(rename = "highestLockBlock")]
    pub locked: QCBlock,
    #[serde(rename = "highestQCBlock")]
    pub qc: QCBlock,
}

/// View
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct View {
    pub epoch: u64,
    #[serde(rename = "viewNumber")]
    pub view: u64,
}

/// QC Block
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct QCBlock {
    pub hash: H256,
    pub number: u64,
}
