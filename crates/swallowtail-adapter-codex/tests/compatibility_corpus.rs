use serde_json::Value;
use std::collections::BTreeSet;

const EXEC_RELEASES: &str = include_str!("fixtures/compatibility/exec-releases.json");
const APP_SERVER_RELEASES: &str = include_str!("fixtures/compatibility/app-server-releases.json");
const APP_SERVER_GATES: &str = include_str!("fixtures/compatibility/app-server-gate-cases.json");
const APP_SERVER_TRANSCRIPT: &str = include_str!("fixtures/compatibility/app-server-core.jsonl");
const APP_SERVER_THREAD_CATALOGUE: &str =
    include_str!("fixtures/compatibility/app-server-thread-catalogue.json");
const APP_SERVER_THREAD_RECONCILIATION: &str =
    include_str!("fixtures/compatibility/app-server-thread-reconciliation.json");
const CODEX_0_147_RANGE: &str = include_str!("fixtures/compatibility/codex-0-147-range.json");
const CODEX_0_148_RANGE: &str = include_str!("fixtures/compatibility/codex-0-148-range.json");
const CODEX_0_149_RANGE: &str = include_str!("fixtures/compatibility/codex-0-149-range.json");
const CODEX_0_149_1_RANGE: &str = include_str!("fixtures/compatibility/codex-0-149-1-range.json");
const CODEX_0_151_0_RANGE: &str = include_str!("fixtures/compatibility/codex-0-151-0-range.json");

include!("compatibility_corpus/exec.rs");
include!("compatibility_corpus/app_server.rs");
include!("compatibility_corpus/thread.rs");
include!("compatibility_corpus/support.rs");
include!("compatibility_corpus/range_0_147.rs");
include!("compatibility_corpus/range_0_148.rs");
include!("compatibility_corpus/range_0_149.rs");
include!("compatibility_corpus/range_0_149_1.rs");
include!("compatibility_corpus/range_0_151.rs");
