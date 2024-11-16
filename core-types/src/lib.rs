//! JSON-RPC types

pub mod error;
pub mod id;
pub mod params;
pub mod request;
pub mod response;
pub mod version;

pub use serde;
pub use serde_json;
pub use serde_json::to_string;
pub use serde_json::value::to_value;
pub use serde_json::Value;

pub use self::error::{Error, ErrorCode};
pub use self::id::Id;
pub use self::params::Params;
pub use self::request::{Call, MethodCall, Notification, Request};
pub use self::response::{Failure, Output, Response, Success};
pub use self::version::Version;

/// A Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// workaround for https://github.com/serde-rs/json/issues/505
/// Arbitrary precision confuses serde when deserializing into untagged enums,
/// this is a workaround
pub fn serde_from_str<'a, T>(input: &'a str) -> std::result::Result<T, serde_json::Error>
where
	T: serde::de::Deserialize<'a>,
{
	if cfg!(feature = "arbitrary_precision") {
		let val = serde_json::from_str::<Value>(input)?;
		T::deserialize(val)
	} else {
		serde_json::from_str::<T>(input)
	}
}
