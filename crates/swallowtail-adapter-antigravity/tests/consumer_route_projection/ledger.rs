//! Ledger of all 32 Antigravity census rows (14 catalogue, 18 headless).

pub const ANTIGRAVITY_CATALOGUE_ROUTE: &str = "antigravity.catalogue";
pub const ANTIGRAVITY_HEADLESS_ROUTE: &str = "antigravity.headless";

pub const PROFILE_CATALOGUE: &str = "AntigravityPreparedCatalogue";
pub const PROFILE_HEADLESS_MAXIMAL: &str = "AntigravityPreparedHeadlessRun[maximal]";
pub const PROFILE_HEADLESS_MINIMAL: &str = "AntigravityPreparedHeadlessRun[minimal]";
pub const PROFILE_CONTINUATION: &str = "AntigravityPreparedContinuation";

pub type RowTuple = (&'static str, &'static str, &'static str);

pub struct LedgerEntry {
    pub route_id: &'static str,
    pub operation_shape: &'static str,
    pub semantic_id: &'static str,
    pub emitted_by: &'static [&'static str],
    pub withheld_because: &'static str,
}

#[path = "ledger/catalogue.rs"]
mod catalogue;
#[path = "ledger/headless.rs"]
mod headless;

pub use catalogue::*;
pub use headless::*;
