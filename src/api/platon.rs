//! `PlatON` namespace

use crate::api::Namespace;
use crate::helpers::{self, CallFuture};
use crate::types::{
    Address, Block, BlockId, BlockNumber, Bytes, CallRequest, Index, Log, SyncState, Transaction, TransactionId,
    TransactionReceipt, TransactionRequest, Work, H256, H520, H64, U256, U64,
};
use crate::Transaction;

/// `PlatON` namespace
#[derive(Debug, Clone)]
pub struct PlatON<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for PlatON<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        PlatON { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> PlatON<T> {
    /// Get list of available accounts.
    pub fn accounts(&self) -> CallFuture<Vec<Address>, T::Out> {
        CallFuture::new(self.transpport.execute("platon_accounts", vec![]))
    }

    /// Get current block number.
    pub fn block_number(&self) -> CallFuture<U64, T::Out> {
        CallFuture::new(self.transport.execute("platon_blockNumber", vec![]))
    }

    /// Call a constant method of contract without changing the state of the blockchain.
    pub fn call(&self, req: CallRequest, block: Option<BlockId>) -> CallFuture<Bytes, T::Out> {
        let req = helpers::serialize(&req);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest.into()));

        CallFuture::new(self.transport.execute("platon_call", vec![req, block]))
    }

    /// Get coinbase address.
    pub fn coinbase(&self) -> CallFuture<Address, T::Out> {
        CallFuture::new(self.transport.execute("platon_coinbase", vec![]))
    }

    /// Call a contract without changing the state of the blockchain to estimate gas usage.
    pub fn estimate_gas(&self, req: CallRequest, block: Option<BlockNumber>) -> CallFuture<U256, T::Out> {
        let req = helpers::serialize(&req);

        let args = match block {
            Some(block) => vec![req, helpers::serialize(&block)],
            None => vec![req],
        };

        CallFuture::new(self.transport.execute("platon_estimateGas", args))
    }

    /// Get current recommended gas price.
    pub fn gas_price(&self) -> CallFuture<U256, T::Out> {
        CallFuture::new(self.transport.execute("platon_gasPrice", vec![]))
    }

    /// Get balance of given address
    pub fn balance(&self, address: Address, block: Option<BlockNumber>) -> CallFuture<U256, T::Out> {
        let address = helpers::serialize(&address);
        let block = helpers::serialize(block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(self.transport.execute("platon_balance", vec![address, block]))
    }

    /// Get block detail with transaction hashes.
    pub fn block(&self, block: BlockId) -> CallFuture<Option<Block>, T::Out> {
        let include_txs = helpers::serialize(&false);

        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport.execute("platon_getBlockByHash", vec![hash, include_txs])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("platon_getBlockByNumber", vec![num, include_txs])
            }
        };

        CallFuture::new(result)
    }

    /// Get block details with full transaction objects.
    pub fn block_with_txs(&self, block: BlockId) -> CallFuture<Option<Block>, T::Out> {
        let include_txs = helpers::serialize(&true);

        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport.execute("platon_getBlockByHash", vec![hash, include_txs])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("platon_getBlockByNumber", vec![num, include_txs])
            }
        };

        CallFuture::new(result)
    }

    /// Get number of transactions in block.
    pub fn block_transaction_count(&self, block: BlockId) -> CallFuture<Option<U256>, T::Out> {
        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport
                    .execute("platon_getBlockTransactionCountByHash", vec![hash])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("platon_getBlockTransactionCountByNumber", vec![num])
            }
        };

        CallFuture::new(result)
    }

    /// Get code under given address.
    pub fn code(&self, address: Address, block: Option<BlockNumber>) -> CallFuture<Bytes, T::out> {
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(self.transport.execute("platon_getCode", vec![address, block]))
    }

    /// Get nonce.
    pub fn transaction_count(&self, address: Address, block: Option<BlockNumber>) -> CallFuture<U256, T::Out> {
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(
            self.transport
                .execute("platon_getTransactionCount", vec![address, block]),
        )
    }

    /// Get transaction.
    pub fn transaction(&self, id: TransactionId) -> CallFuture<Option<Transaction>, T::Out> {
        let result = match id {
            TransactionId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport.execute("platon_getTransactionByHash", vec![hash])
            }
            TransactionId::Block(BlockId::Hash(hash), index) => {
                let hash = helpers::serialize(&hash);
                let idx = helpers::serialize(&index);
                self.transport
                    .execute("platon_getTransactionByBlockHashAndIndex", vec![hash, idx])
            }
            TransactionId::Block(BlockId::Number(num), index) => {
                let num = helpers::serialize(&num);
                let idx = helpers::serialize(&index);
                self.transport
                    .execute("platon_getTransactionByBlockNumberAndIndex", vec![num, idx])
            }
        };

        CallFuture::new(result)
    }

    /// Get transaction receipt.
    pub fn transaction_receipt(&self, hash: H256) -> CallFuture<Option<TransactionReceipt>, T::Out> {
        let hash = helpers::serialize(&hash);

        CallFuture::new(self.transport.execute("platon_getTransactionReceipt", vec![hash]))
    }

    /// Sends a rlp-encoded signed transaction.
    pub fn send_raw_transaction(&self, rlp: Bytes) -> CallFuture<H256, T::Out> {
        let rlp = helpers::serialize(&rlp);
        CallFuture::new(self.transport.execute("platon_sendRawTransaction", vec![rlp]))
    }

    /// Sends a transaction.
    pub fn send_transaction(&self, tx: TransactionRequest) -> CallFuture<H256, T::Out> {
        let tx = helpers::serialize(&tx);
        CallFuture::new(self.transport.execute("platon_sendTransaction", vec![tx]))
    }

    /// Signs a hash of given data.
    pub fn sign(&self, address: Address, data: Bytes) -> CallFuture<H520, T::Out> {
        let address = helpers::serialize(&address);
        let data = helpers::serialize(&data);
        CallFuture::new(self.transport.execute("platon_sign", vec![address, data]))
    }

    /// Get syncing status.
    pub fn syncing(&self) -> CallFuture<SyncState, T::Out> {
        CallFuture::new(self.transport.execute("platon_syncing", vec![]))
    }
}
