//! Frozen identity ledger for the Grok Build ACP `1.0.30` useful-newer run.
//!
//! The official npm stable channel advanced from the qualified `1.0.5`
//! ceiling to `1.0.30` with every stable `1.0.6..=1.0.30` published. These
//! tests pin the package, platform, and executable identity of every hop, the
//! mapped ACP surface literal set, the embedded default-model document, the
//! ACP module inventory, and the shipped-file inventory. No downloaded
//! artifact was executed; the installed host was observed only.

#[path = "grok_1_0_30_identity/claims.rs"]
mod claims;
#[path = "grok_1_0_30_identity/identity.rs"]
mod identity;
#[path = "grok_1_0_30_identity/inventory.rs"]
mod inventory;
#[path = "grok_1_0_30_identity/protocol.rs"]
mod protocol;
#[path = "grok_1_0_30_identity/support.rs"]
mod support;
