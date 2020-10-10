//! `PlatON` namespace, subscriptions

use std::marker::PhantomData;
use std::pin::Pin;

use crate::api::Namespace;
use crate::helpers::{self, CallFuture};
use crate::types::{BlockHeader, H256};
use crate::{error, DuplexTransport};
use futures::{
    task::{Context, Poll},
    Future, FutureExt, Stream, StreamExt,
};
use serde;
use serde_json;

/// `PlatON` namespace, subscriptions
#[derive(Debug, Clone)]
pub struct PlatONSubscribe<T> {
    transport: T,
}

impl<T: DuplexTransport> Namespace<T> for PlatONSubscribe<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        PlatONSubscribe { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

/// ID of subscription returned from `platon_subscribe`
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct SubscriptionId(String);

impl From<String> for SubscriptionId {
    fn from(s: String) -> Self {
        SubscriptionId(s)
    }
}

/// Stream of notifications from a subscription
/// Given a type deserialized from rpc::Value and a subscription id, yields items of that type as
/// notifications are derived.
#[derive(Debug)]
pub struct SubscriptionStream<T: DuplexTransport, I> {
    transport: T,
    id: SubscriptionId,
    rx: T::NotificationStream,
    _marker: PhantomData<I>,
}

impl<T: DuplexTransport, I> SubscriptionStream<T, I> {
    fn new(transport: T, id: SubscriptionId) -> error::Result<Self> {
        let rx = transport.subscribe(id.clone())?;
        Ok(SubscriptionStream {
            transport,
            id,
            rx,
            _marker: PhantomData,
        })
    }

    /// Return the ID of this subscription
    pub fn id(&self) -> &SubscriptionId {
        &self.id
    }

    /// Unsubscribe from the event represented by this stream
    pub fn unsubscribe(self) -> CallFuture<bool, T::Out> {
        let &SubscriptionId(ref id) = &self.id;
        let id = helpers::serialize(&id);
        CallFuture::new(self.transport.execute("platon_unsubscribe", vec![id]))
    }
}

impl<T, I> Stream for SubscriptionStream<T, I>
where
    T: DuplexTransport,
    T::NotificationStream: Unpin,
    I: serde::de::DeserializeOwned + Unpin,
{
    type Item = error::Result<I>;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
        let x = ready!(self.rx.poll_next_unpin(ctx));
        Poll::Ready(x.map(|result| serde_json::from_value(result).map_err(Into::into)))
    }
}

impl<T: DuplexTransport, I> Drop for SubscriptionStream<T, I> {
    fn drop(&mut self) {
        self.transport.unsubscribe(self.id().clone());
    }
}

/// A result of calling a subscription.
#[derive(Debug)]
pub struct SubscriptionResult<T: DuplexTransport, I> {
    transport: T,
    inner: CallFuture<String, T::Out>,
    _marker: PhantomData<I>,
}

impl<T: DuplexTransport, I> SubscriptionResult<T, I> {
    /// New `SubscriptionResult`.
    pub fn new(transport: T, id_future: CallFuture<String, T::Out>) -> Self {
        SubscriptionResult {
            transport,
            inner: id_future,
            _marker: PhantomData,
        }
    }
}

impl<T, I> Future for SubscriptionResult<T, I>
where
    T: DuplexTransport,
    I: serde::de::DeserializeOwned + Unpin,
{
    type Output = error::Result<SubscriptionStream<T, I>>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let id = ready!(self.inner.poll_unpin(ctx))?;
        Poll::Ready(SubscriptionStream::new(self.transport.clone(), SubscriptionId(id)))
    }
}

impl<T: DuplexTransport> PlatONSubscribe<T> {
    /// Create new heads subscription
    pub fn subscribe_new_heads(&self) -> SubscriptionResult<T, BlockHeader> {
        let subscription = helpers::serialize(&&"newHeads");
        let id_future = CallFuture::new(self.transport.execute("platon_subscribe", vec![subscription]));
        SubscriptionResult::new(self.transport().clone(), id_future)
    }
}
