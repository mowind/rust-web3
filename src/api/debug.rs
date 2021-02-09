//! `debug` namespace

use crate::api::Namespace;
use crate::helpers::{self, CallFuture};
use crate::types::ConsensusStatus;
use crate::Transport;

/// `debug` namespace
#[derive(Debug, Clone)]
pub struct Debug<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Debug<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Debug { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Debug<T> {
    /// Get CBFT consensus status
    pub fn consensus_status(&self, ledger_name: &String) -> CallFuture<ConsensusStatus, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        CallFuture::new(self.transport.execute("debug_consensusStatus", vec![ledger_name]))
    }
}
