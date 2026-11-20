//! Provider-neutral bounded HTTP and server-sent-event transport.
//!
//! Adapters keep their provider protocols and public surfaces; this crate owns
//! the one bounded socket, framing, cancellation, timeout and secret-header
//! implementation they share.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod client;
mod error;
mod request;
mod response;
mod sse;
mod stream;

#[cfg(test)]
mod tests;

pub use client::{HttpTransport, STREAM_CAPACITY};
pub use error::TransportError;
pub use request::{
    DEFAULT_CONNECT_TIMEOUT, DEFAULT_RESPONSE_LIMIT, DEFAULT_TRANSFER_TIMEOUT, HttpRequest, Method,
    SecretHeader,
};
pub use response::HttpResponse;
pub use sse::{
    DEFAULT_EVENT_LIMIT, SseDecodeError, SseDecodeErrorKind, SseDecoder, SseFrame, SseFramer,
};
pub use stream::{BodyDecoder, StreamError, StreamItem, Subscription};
