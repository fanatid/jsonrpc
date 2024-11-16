//! ### Transport agnostic jsonrpc library.
//!
//! Right now it supports only server side handling requests.
//!
//! ```rust
//! use jsonrpc_core::IoHandler;
//! use jsonrpc_core::Value;
//! let mut io = IoHandler::new();
//! io.add_sync_method("say_hello", |_| {
//!     Ok(Value::String("Hello World!".into()))
//! });
//!
//! let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#;
//! let response = r#"{"jsonrpc":"2.0","result":"Hello World!","id":1}"#;
//!
//! assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
//! ```

#![deny(missing_docs)]

use std::pin::Pin;

#[macro_use]
extern crate log;

#[cfg(feature = "futures")]
pub use futures;
#[cfg(feature = "futures-executor")]
pub use futures_executor;
pub use futures_util;

pub use jsonrpc_core_types as types;
pub use types::*;

#[doc(hidden)]
pub extern crate serde;
#[doc(hidden)]
pub extern crate serde_json;

mod calls;
mod io;

pub mod delegates;
pub mod middleware;

/// A `Future` trait object.
pub type BoxFuture<T> = Pin<Box<dyn std::future::Future<Output = T> + Send>>;

pub use crate::calls::{
	Metadata, RemoteProcedure, RpcMethod, RpcMethodSimple, RpcMethodSync, RpcNotification, RpcNotificationSimple,
	WrapFuture,
};
pub use crate::delegates::IoDelegate;
pub use crate::io::{
	Compatibility, FutureOutput, FutureResponse, FutureResult, FutureRpcResult, IoHandler, IoHandlerExtension,
	MetaIoHandler,
};
pub use crate::middleware::{Middleware, Noop as NoopMiddleware};
