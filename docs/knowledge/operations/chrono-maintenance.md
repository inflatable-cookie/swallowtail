# Transitive chrono maintenance disposition

Owner: Swallowtail maintainers. Review by2026-12-17, or before changing the
chrono/cbor-diag/serde_with dependency edges, whichever comes first.

[RUSTSEC-2026-0543](https://github.com/RustSec/advisory-db/blob/main/crates/chrono/RUSTSEC-2026-0543.md)
is an unmaintained notice affecting every chrono version, with no patched
release. [Upstream’s maintenance discussion](https://github.com/chronotope/chrono/issues/1768)
recommends migration. The current lock retains chrono0.4.45 through
cbor-diag0.1.12 (CBOR diagnostic date formatting) and optional chrono support in
serde_with3.21.0. Swallowtail adds no direct chrono API use or new exposure here.

Task240 brief19 permits a precise bounded maintenance disposition. Accept only
this maintenance advisory for the currently pinned transitive use until the
expiry in deny.toml. This accepts delayed maintenance response, not absence of
risk. Replacing the transitive serialization/date APIs across upstream consumers
is a separate dependency migration; it is not silently folded into SDK semantics.
Review upstream replacement releases and remove this exception on migration.
Every other advisory, including any future chrono vulnerability or unsoundness,
continues to fail the normal gate. The exception expires automatically; the
security job is neither disabled nor filtered.

The separate [libz-sys vulnerability](https://github.com/RustSec/advisory-db/blob/main/crates/libz-sys/RUSTSEC-2026-0544.md)
is repaired by the minimal lock update1.1.31→1.1.33. Its
[upstream zlib update](https://github.com/rust-lang/libz-sys/commit/b161d77050c00c6eeb7dd4b66ea14d4524d73dc4)
contains the patched implementation; no vulnerability exception is added.
Both task240 and ACP239 consume this shared lock/policy commit, not duplicate
workspace migrations. `effigy security:dependencies` and current CI cover it.
