// crates/a2a-server/src/lib.rs
//! A2A protocol server.

pub mod error;
pub mod handler;
pub mod store;

pub use error::{Error, Result};
pub use handler::{AgentHandler, RequestContext};
pub use store::{InMemoryTaskStore, TaskFilter, TaskStore};
