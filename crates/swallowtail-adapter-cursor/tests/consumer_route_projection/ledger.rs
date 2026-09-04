//! Ledger of all 43 Cursor census rows (13 ACP, 13 catalogue, 17 headless).

pub const CURSOR_ACP_ROUTE: &str = "cursor-agent.acp";
pub const CURSOR_CATALOGUE_ROUTE: &str = "cursor-agent.catalogue";
pub const CURSOR_HEADLESS_ROUTE: &str = "cursor-agent.headless";

pub const PROFILE_ACP: &str = "CursorPreparedAcpSession";
pub const PROFILE_CATALOGUE: &str = "CursorPreparedCatalogue";
pub const PROFILE_HEADLESS_MAXIMAL: &str = "CursorPreparedHeadlessRun[maximal]";
pub const PROFILE_HEADLESS_MINIMAL: &str = "CursorPreparedHeadlessRun[minimal]";

pub type RowTuple = (&'static str, &'static str, &'static str);

pub struct LedgerEntry {
    pub route_id: &'static str,
    pub operation_shape: &'static str,
    pub semantic_id: &'static str,
    pub emitted_by: &'static [&'static str],
    pub withheld_because: &'static str,
}

#[path = "ledger/acp.rs"]
mod acp;
#[path = "ledger/catalogue.rs"]
mod catalogue;
#[path = "ledger/headless.rs"]
mod headless;

pub use acp::*;
pub use catalogue::*;
pub use headless::*;
