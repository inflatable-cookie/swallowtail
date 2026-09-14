//! Frozen catalogue-specific identity ledger for the Grok Build `1.0.30`
//! useful-newer run.
//!
//! The exact `QualifiedOnly` catalogue point moves from `1.0.25` to the
//! current official stable and installed `1.0.30`. These tests pin the
//! package, platform, and executable identity of every compared hop, the
//! catalogue command grammar, the shipped bullet grammar and authentication
//! preamble, the `xai-grok-pager/src/models.rs` module path, and the embedded
//! default-model document. No downloaded artifact was executed.

#[path = "grok_1_0_30_catalogue_identity/identity.rs"]
mod identity;
#[path = "grok_1_0_30_catalogue_identity/support.rs"]
mod support;
