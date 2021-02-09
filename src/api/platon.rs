//! `PlatON` namespace

use crate::api::Namespace;
use crate::helpers::{self, CallFuture};
use crate::types::{
    Address, Block, BlockId, BlockNumber, Bytes, CallRequest, Index, Log, SyncState, Transaction, TransactionId,
    TransactionReceipt, TransactionRequest, Work, H256, H520, H64, U256, U64,
};
use crate::Transport;

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
        CallFuture::new(self.transport.execute("platon_accounts", vec![]))
    }

    /// Get current block number.
    pub fn block_number(&self, ledger_name: &String) -> CallFuture<U64, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        CallFuture::new(self.transport.execute("platon_blockNumber", vec![ledger_name]))
    }

    /// Call a constant method of contract without changing the state of the blockchain.
    pub fn call(&self, ledger_name: &String, req: CallRequest, block: Option<BlockId>) -> CallFuture<Bytes, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let req = helpers::serialize(&req);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest.into()));

        CallFuture::new(self.transport.execute("platon_call", vec![ledger_name, req, block]))
    }

    /// Get coinbase address.
    pub fn coinbase(&self) -> CallFuture<Address, T::Out> {
        CallFuture::new(self.transport.execute("platon_coinbase", vec![]))
    }

    /// Call a contract without changing the state of the blockchain to estimate gas usage.
    pub fn estimate_gas(
        &self,
        ledger_name: &String,
        req: CallRequest,
        block: Option<BlockNumber>,
    ) -> CallFuture<U256, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let req = helpers::serialize(&req);

        let args = match block {
            Some(block) => vec![ledger_name, req, helpers::serialize(&block)],
            None => vec![ledger_name, req],
        };

        CallFuture::new(self.transport.execute("platon_estimateGas", args))
    }

    /// Get current recommended gas price.
    pub fn gas_price(&self, ledger_name: &String) -> CallFuture<U256, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        CallFuture::new(self.transport.execute("platon_gasPrice", vec![ledger_name]))
    }

    /// Get balance of given address
    pub fn balance(
        &self,
        ledger_name: &String,
        address: Address,
        block: Option<BlockNumber>,
    ) -> CallFuture<U256, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(
            self.transport
                .execute("platon_balance", vec![ledger_name, address, block]),
        )
    }

    /// Get block detail with transaction hashes.
    pub fn block(&self, ledger_name: &String, block: BlockId) -> CallFuture<Option<Block<H256>>, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let include_txs = helpers::serialize(&false);

        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport
                    .execute("platon_getBlockByHash", vec![ledger_name, hash, include_txs])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("platon_getBlockByNumber", vec![ledger_name, num, include_txs])
            }
        };

        CallFuture::new(result)
    }

    /// Get block details with full transaction objects.
    pub fn block_with_txs(
        &self,
        ledger_name: &String,
        block: BlockId,
    ) -> CallFuture<Option<Block<Transaction>>, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let include_txs = helpers::serialize(&true);

        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport
                    .execute("platon_getBlockByHash", vec![ledger_name, hash, include_txs])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("platon_getBlockByNumber", vec![ledger_name, num, include_txs])
            }
        };

        CallFuture::new(result)
    }

    /// Get number of transactions in block.
    pub fn block_transaction_count(&self, ledger_name: &String, block: BlockId) -> CallFuture<Option<U256>, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport
                    .execute("platon_getBlockTransactionCountByHash", vec![ledger_name, hash])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("platon_getBlockTransactionCountByNumber", vec![ledger_name, num])
            }
        };

        CallFuture::new(result)
    }

    /// Get code under given address.
    pub fn code(
        &self,
        ledger_name: &String,
        address: Address,
        block: Option<BlockNumber>,
    ) -> CallFuture<Bytes, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(
            self.transport
                .execute("platon_getCode", vec![ledger_name, address, block]),
        )
    }

    /// Get nonce.
    pub fn transaction_count(
        &self,
        ledger_name: &String,
        address: Address,
        block: Option<BlockNumber>,
    ) -> CallFuture<U256, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallFuture::new(
            self.transport
                .execute("platon_getTransactionCount", vec![ledger_name, address, block]),
        )
    }

    /// Get transaction.
    pub fn transaction(&self, ledger_name: &String, id: TransactionId) -> CallFuture<Option<Transaction>, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let result = match id {
            TransactionId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport
                    .execute("platon_getTransactionByHash", vec![ledger_name, hash])
            }
            TransactionId::Block(BlockId::Hash(hash), index) => {
                let hash = helpers::serialize(&hash);
                let idx = helpers::serialize(&index);
                self.transport
                    .execute("platon_getTransactionByBlockHashAndIndex", vec![ledger_name, hash, idx])
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
    pub fn transaction_receipt(&self, ledger_name: &String,hash: H256) -> CallFuture<Option<TransactionReceipt>, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let hash = helpers::serialize(&hash);

        CallFuture::new(self.transport.execute("platon_getTransactionReceipt", vec![ledger_name,hash]))
    }

    /// Sends a rlp-encoded signed transaction.
    pub fn send_raw_transaction(&self, ledger_name: &String,rlp: Bytes) -> CallFuture<H256, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let rlp = helpers::serialize(&rlp);
        CallFuture::new(self.transport.execute("platon_sendRawTransaction", vec![ledger_name,rlp]))
    }

    /// Sends a transaction.
    pub fn send_transaction(&self, ledger_name: &String,tx: TransactionRequest) -> CallFuture<H256, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let tx = helpers::serialize(&tx);
        CallFuture::new(self.transport.execute("platon_sendTransaction", vec![ledger_name,tx]))
    }

    /// Signs a hash of given data.
    pub fn sign(&self, ledger_name: &String,address: Address, data: Bytes) -> CallFuture<H520, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        let address = helpers::serialize(&address);
        let data = helpers::serialize(&data);
        CallFuture::new(self.transport.execute("platon_sign", vec![ledger_name,address, data]))
    }

    /// Get syncing status.
    pub fn syncing(&self, ledger_name: &String) -> CallFuture<SyncState, T::Out> {
        let ledger_name = helpers::serialize(ledger_name);
        CallFuture::new(self.transport.execute("platon_syncing", vec![ledger_name]))
    }
}
