# Research 281: v0.3.3 To Candidate Compatibility And Freeze Audit

Status: complete locally; exact-head review pending
Date: 2026-09-03
Task: g05.021 Card 050
Audited head: `b7f804b5940d666d81f7b43fb562be5797c59575`

## Result

The frozen source is suitable for coordinated pre-1.0 `0.4.0` candidate
preparation after independent exact-head review. Contract 036 fixes the minor:
the current source removes the previously guaranteed, unqualified OpenAI
Background `minimal` value, and the shared interactive close seam is now a
breaking public API and cleanup-boundary change.

This record is an audit, not candidate preparation. It changes no Cargo
version or requirement, changelog release state, release baseline, runtime code,
claim, fixture, workflow, tag, provider state, or consumer repository. The
temporary semantic API output was generated independently and is not retained
in source. The current unreleased API evidence was only used as a comparison
reference; it was not rewritten. The repair strengthens the existing route QA
gate with a deterministic audit-ledger assertion; it changes no runtime or
release behavior.

## Canonical freeze proof

| Field | Evidence |
| --- | --- |
| remote | `git@github.com:inflatable-cookie/swallowtail.git` |
| branch | canonical `main` |
| audited local head | `b7f804b5940d666d81f7b43fb562be5797c59575` |
| `origin/main` | exactly `b7f804b5940d666d81f7b43fb562be5797c59575` at fetch and final pre-freeze check |
| immutable tag | annotated `v0.3.3`, object `ca30b367e51a70c56b0998b27e7e660ba7145657`, peel `51d186208e75dca4c04f077dd7179ec3c2fafae9` |
| merge base | `51d186208e75dca4c04f077dd7179ec3c2fafae9` |
| range commits | 826; full ordered inventory in [`range-commits.tsv`](281-v0-4-0-compatibility-and-freeze-audit/range-commits.tsv), SHA-256 `18e2345ea85837b3b42a7f86f2579d1972b58e74d0aa2e35038fc1e2629f4783` |
| changed files | 2,694; full name-status inventory in [`range-name-status.tsv`](281-v0-4-0-compatibility-and-freeze-audit/range-name-status.tsv), SHA-256 `8d97dc631ea9a011e0dcfb275f20f8ee9f1f285c7cb0cf539169e0c35cf60fc9` |
| name-status totals | 1,961 added, 731 modified, 1 deleted, 1 renamed |
| open mergeable PRs | none from `gh pr list --state open`; no overlapping feature/currentness source head |

The full package, dependency, route, and count summary is in
[`census-summary.tsv`](281-v0-4-0-compatibility-and-freeze-audit/census-summary.tsv).

The complete 88-edge internal dependency ledger is
[`dependency-ledger.tsv`](281-v0-4-0-compatibility-and-freeze-audit/dependency-ledger.tsv).
Every edge is retained with the same consumer/dependency topology and the
same acyclic workspace direction. The immutable dependency artifact records
the historical `^0.3.2` literals; current source correctly normalizes those
requirements to `^0.3.3`, as the metadata checker expects. No dependency edge
was added, removed, or redirected. `Cargo.lock` has ordinary transitive
updates (including `nix` and `windows-sys`) from the full development range;
this is not an internal package-topology or public compatibility break.

## Package and route census

- The package set is exactly 40 at both the tag and audited head. The sorted
  package-set digest is `b6abf1f9218871ac15a6c4c9c057d4dc5518437109f4f06490ca45d13885a623`.
- The immutable `v0.3.3` release route inventory remains exactly 47. Its
  historical file was not changed.
- The current integration/route and lifecycle inventories each contain exactly
  49 rows. The only rows absent from the immutable 47-row release inventory are
  `pi.sdk-sidecar` and `claude-agent.sdk`.
- The route-behavior ledger now records 47 historical `yes` rows and exactly
  two historical `no` rows, `pi.sdk-sidecar` and `claude-agent.sdk`; the
  `effigy qa:routes` gate asserts that set and the current 49-route boundary.
- The current feature matrix has 41 solution rows and 49 unique route IDs; the
  tag had 39 solution rows and 47 route IDs. The current activity matrix is
  also reconciled by the route gate.
- No candidate route baseline was created. Card 051 owns creation of a
  distinct candidate inventory from this accepted current census, but its
  existing 49-route requirement fixes inclusion of both current-only rows,
  `pi.sdk-sidecar` and `claude-agent.sdk`; inclusion is not a later Card 051
  policy choice.

The complete current row, lifecycle posture, feature delta, compatibility,
consumer-effect, release-note, and rollback ledger is
[`route-behavior-ledger.tsv`](281-v0-4-0-compatibility-and-freeze-audit/route-behavior-ledger.tsv).
The exact old/new rows for all 29 changed or added route rows are in
[`route-delta.tsv`](281-v0-4-0-compatibility-and-freeze-audit/route-delta.tsv).
The ledger has 20 route-local negative rows. It also marks the shared close
break on all 25 current `InteractiveSessionDriver` routes, so a route-local
matrix equality does not hide the cross-cutting cleanup change.

## Semantic Rust API evidence

Generation used Contract 036's pinned semantic toolchain:

```text
cargo-public-api 0.52.0
nightly-2026-08-05
rustc 1.99.0-nightly (1ed2df61a 2026-08-04)
all features; --simplified repeated three times; blanket, auto-trait, and auto-derived implementations omitted
```

The output was generated with `scripts/generate-public-api-baseline.sh` into
temporary audit storage. The generation log SHA-256 was
`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
`swallowtail-adapter-ollama` emitted one existing rustdoc warning for an
unresolved `InteractiveSessionDriver::open_session` intra-doc link; generation
completed for all 40 packages. This audit does not repair that papercut.

[`package-api-ledger.tsv`](281-v0-4-0-compatibility-and-freeze-audit/package-api-ledger.tsv)
contains exactly one row per package, the immutable or current-unreleased
reference hash, independently generated current hash and line count, semantic
delta counts, explicit negative evidence, compatibility class, consumer
effect, release-note coverage, and upgrade/rollback wording.

Results:

- all 40 generated package outputs match their selected reference: 26 current
  unreleased references and 14 immutable `v0.3.3` references;
- 14 packages are semantically unchanged from immutable `v0.3.3`, with explicit
  negative rows in the ledger;
- 26 packages have additive API lines; there are 1,711 additions in total;
- the only removal is the approved exact zero-argument item:
  `pub fn swallowtail_runtime::InteractiveSessionHandle::close(alloc::boxed::Box<Self>) -> swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::CleanupOutcome>`;
- the replacement close signature is present with exact
  `SessionCleanupRequest` and `HostServices` parameters; it is the coordinated
  `0.4.0` signature break, not a compatible overload;
- reserved-reap surfaces (`TaskReapReservation`, `reserve_reap`, and
  `spawn_reapable`) are additive. `AcceptedForReap` remains ownership transfer,
  never join or cleanup completion.

The independent current hashes equal the existing unreleased evidence for all
26 unreleased package references. No historical API baseline changed.

## Guaranteed-behavior classification

The complete grouped classification is in
[`guaranteed-behavior-ledger.tsv`](281-v0-4-0-compatibility-and-freeze-audit/guaranteed-behavior-ledger.tsv).
It covers removal, signature, provider range, route identity, target, MSRV,
lifecycle, cleanup, access, isolation, and evidence changes, including
negative proofs. The material compatibility result is:

- `openai.background`: `minimal` is removed for exact GPT-5.6 and fails before
  endpoint, credential, request, or provider work. The corrected opaque facade
  and selected-tier checkpoint restriction are both release-note obligations.
- Interactive close: exact host services and one caller-selected absolute
  deadline now govern interruption, escalation, task/pump joins, credential
  release, and resource release. Expiry is failed or unconfirmed cleanup, not
  clean completion.
- `kimi-code.acp`: `QualifiedOnly` ends at `0.38.0`; exact `0.39.0` and
  `0.39.1` fail closed because the newer terminal path creates an uncontained
  local process. This is a provider-range/access/isolation restriction above
  the retained maintained range.
- `kimi-code.headless`: the v1/v2 identity boundary is corrected to
  `0.29.0..=0.32.0` and `0.33.0..=0.39.1`; the former v1 claim over
  `0.33.0..=0.37.2` must not be used.
- `claude-agent.sdk` and `pi.sdk-sidecar`: both are additive current
  integration rows, not changes to an old route identity. Claude is restored
  on the reserved-reap seam with provider-free delegated subscription
  authentication, Unix-only/root-only degraded cleanup, and no session
  persistence. Pi is an exact `0.84.2` sidecar route with app-owned durable
  session state, bounded load replay, and resume. Both are included in the
  frozen 49-route v0.4.0 candidate boundary; neither is in the immutable
  47-row release inventory.
- reserved reaping and process-tree evidence are additive and stricter. The
  host-local implementation remains root-only under ordinary macOS authority;
  only attested `OwnedTreeEmpty` supports `Clean`.
- all other changed route rows are compatible extensions, exact-version
  currentness evidence, or explicitly bounded route-local additions. Existing
  symbols, package identities, verified Rust floor, and existing verified
  target remain intact.

The current `[Unreleased]` changelog already contains concept-level coverage
for the close break, minimal removal, reserved reaping, tree evidence, Claude
SDK, currentness, and route-local feature changes. Pi SDK sidecar has a guide
and historical evidence but no dedicated current `[Unreleased]` route-add
entry. Because Card 051's existing candidate boundary is explicitly 49 routes,
Pi inclusion is frozen and Card 051 must add this required `Added` entry before
promotion:

> add `pi.sdk-sidecar` as a qualified exact `0.84.2` route through the
> application-provisioned Node `22.23.2` runtime, source-tagged sidecar, and
> `swallowtail-pi-sdk-jsonl-v1` wire; new, load, and resume use the exact
> host-leased cwd with bounded typed replay on load, durable app-owned session
> state, no archive/restore/delete, and no substitution for `pi.rpc`.

The same entry must state the consumer treatment: new consumers provision
those exact runtime, sidecar, wire, SDK, and session-directory axes; existing
v0.3.3 consumers have no action unless opting into the new route. Its rollback
treatment is exact: revert to `v0.3.3`, omit the route and sidecar calls, and do
not mix workspace versions or alias `pi.rpc`. This is a required Card 051
release-note action, not a candidate-inclusion choice. Card 050 does not
promote or edit the changelog.

## Release posture

The following remain correct and fixed by existing authority:

- coordinated pre-1.0 `0.4.0`, required by Contract 036's breaking behavior
  classification;
- 40 coordinated packages, all `publish = false`;
- immutable release inventory 47 and current integration/route truth 49,
  exactly `+pi.sdk-sidecar` and `+claude-agent.sdk`;
- Rust `1.95.0` for all packages and Apple Silicon macOS as the existing
  verified target; no floor raise or existing-target removal;
- source-only annotated-tag intent: no registry publication, GitHub Release,
  binary, sidecar, installer, tag creation, or tag push is authorized by this
  card.

Research 276 remains authoritative for currentness: no feature/currentness
implementation reopened, Kimi local-server `0.40.1` remains parked, the
watcher remains exact `2.1.251` and not live-ready, Gemini remains deferred,
Kimi ACP remains capped at `0.38.0`, and the Contract 061 projection remains
249 of 767 rows proved with 518 remaining.

No Card050 product-policy choice remains open for version, package set,
immutable route set, target, MSRV, or compatibility classification. Card 051's
existing 49-route candidate requirement fixes the two current integration-only
routes as candidate additions, including Pi; it must create that distinct
candidate inventory after exact-head review and carry the required Pi
release-note, consumer, and rollback wording above. The candidate 49 remains
distinct from the immutable 47-row release inventory.

## Authenticated working-application smoke: operator question

Before Card 052 selects or runs this proof, please provide one complete
authority packet naming every field below. This audit does not select an
application, contact a provider, or run a smoke:

1. repository and application;
2. exact route;
3. command or normal product action;
4. exact candidate SHA or tag consumed;
5. credential and provider authority;
6. permitted mutations;
7. success evidence and redaction rules;
8. cleanup authority and expected cleanup evidence;
9. retry budget and stop behavior.

## Immutable baseline proof

The 131-file manifest in
[`immutable-baselines.sha256.tsv`](281-v0-4-0-compatibility-and-freeze-audit/immutable-baselines.sha256.tsv)
has equal tag and audited-head hashes for every historical
`public-api-0.1.0` through `public-api-0.3.3`,
`internal-dependencies-0.1.0` through `internal-dependencies-0.3.3`,
`production-routes-0.1.1` through `production-routes-0.3.3`, and
`rust-toolchains-0.1.0` and `rust-toolchains-0.2.0` file. The manifest SHA-256
is `0dcae8d51984f1d236beddad0a92dde2d54129ca74c81b08b5bc431c53eaa5b`.
The only release-baseline path changes in the full range are under
`public-api-unreleased`; no immutable directory changed.

## Review state

Card 050 is locally complete only if the exact Card050 validation list passes
on this tree and the final pre-push identity still equals `b7f804b5…`.
Cards 051-052 remain planned. Card 051 must not become ready until an
independent exact-head review accepts this audit. It consumes the frozen
49-route candidate boundary, including `pi.sdk-sidecar` and
`claude-agent.sdk`, and must perform the fixed Pi release-note action above; it
still requires separate operator authorization before Effigy's mutating
prepare path.

The exact listed validation tier passed on this tree:

- `effigy package:metadata` — 40 crates at source version `0.3.3`, Rust
  `1.95`, immutable `v0.3.2` package baseline retained;
- `effigy package:api` — 40 packages, approved removals only;
- `effigy qa:routes` — 87 activity operations, 57 available, 30
  not-applicable, 49 production routes, 4 auxiliary catalogues, and the
  exact 47-historical/2-current-only route-ledger membership assertion;
- all five requested roadmap index/status/next-action checks passed;
- `effigy qa:docs:links` — 15 front-door and 1,163 research/log Markdown
  files checked;
- `effigy qa:docs:roadmaps:numbers` — 1,277 numbered milestone/card files
  unique against canonical `main` at `b7f804b5`;
- `effigy qa:northstar` and `git diff --check` — passed.

The Card050 list contains 12 commands despite its prose calling them 11; all
12 listed commands were run. No candidate release gate or workspace test
suite was run.
