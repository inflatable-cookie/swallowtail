use serde_json::Value;

pub(crate) const API_VERSION: &str = "2023-06-01";
pub(crate) const BETA_VERSION: &str = "managed-agents-2026-04-01";
pub(crate) const ENDPOINT_AUDIENCE: &str = "api.anthropic.com";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Method {
    Get,
    Post,
    Delete,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Request {
    pub method: Method,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    fn get(path: String, query: Vec<(String, String)>) -> Self {
        Self {
            method: Method::Get,
            path,
            query,
            body: None,
        }
    }

    fn post(path: String, body: Value) -> Self {
        Self {
            method: Method::Post,
            path,
            query: Vec::new(),
            body: Some(serde_json::to_vec(&body).expect("managed request JSON serializes")),
        }
    }

    fn delete(path: String) -> Self {
        Self {
            method: Method::Delete,
            path,
            query: Vec::new(),
            body: None,
        }
    }

    #[must_use]
    pub(crate) fn expects_stream(&self) -> bool {
        self.path.ends_with("/events/stream")
    }
}

mod event;
mod request;
mod response;

pub(crate) use event::{
    IdleReason, ManagedEvent, ManagedEventKind, parse_history, parse_stream, reconcile,
};
pub(crate) use response::{
    parse_deletion, parse_environment, parse_session_usage, parse_session_with_tools,
    validate_agent,
};
#[cfg(test)]
pub(crate) use response::{validate_environment, validate_session};

#[cfg(test)]
#[path = "managed/tests.rs"]
mod tests;
