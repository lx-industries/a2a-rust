// crates/a2a-transport-wasi/src/poll.rs
//! Poll-to-async bridge for WASI pollables.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasi::io::poll::{poll, Pollable};

/// A future that wraps a WASI `Pollable` and yields when it becomes ready.
///
/// This allows WASI poll-based async to integrate with Rust's `Future` trait.
pub struct WasiPollFuture<'a> {
    pollable: &'a Pollable,
}

impl<'a> WasiPollFuture<'a> {
    /// Create a new poll future from a WASI pollable.
    pub fn new(pollable: &'a Pollable) -> Self {
        Self { pollable }
    }
}

impl Future for WasiPollFuture<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if already ready without blocking
        if self.pollable.ready() {
            return Poll::Ready(());
        }
        // Block on the pollable - in WASM, this yields to the host
        poll(&[self.pollable]);
        Poll::Ready(())
    }
}

/// Extension trait for WASI Pollable to convert to a Future.
pub trait PollableExt {
    /// Wait for this pollable to become ready.
    fn wait(&self) -> WasiPollFuture<'_>;
}

impl PollableExt for Pollable {
    fn wait(&self) -> WasiPollFuture<'_> {
        WasiPollFuture::new(self)
    }
}
