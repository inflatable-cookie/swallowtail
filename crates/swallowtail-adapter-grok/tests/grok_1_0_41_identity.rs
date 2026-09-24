//! Frozen identity ledger for the Grok Build ACP `1.0.41` useful-newer run.
//!
//! The official npm stable channel advanced from the qualified `1.0.40`
//! ceiling to `1.0.41`, the single published hop after `1.0.40`. These tests
//! pin the package, platform, and executable identity of that hop, the mapped
//! ACP surface literal set against the frozen `1.0.40` corpus, the embedded
//! default-model document, the ACP module inventory delta, and the shipped
//! file inventory delta. No downloaded artifact was executed; host `grok` was
//! absent and was not installed.

#[path = "grok_1_0_41_identity/claims.rs"]
mod claims;
#[path = "grok_1_0_41_identity/identity.rs"]
mod identity;
#[path = "grok_1_0_41_identity/inventory.rs"]
mod inventory;
#[path = "grok_1_0_41_identity/protocol.rs"]
mod protocol;
#[path = "grok_1_0_41_identity/support.rs"]
mod support;
