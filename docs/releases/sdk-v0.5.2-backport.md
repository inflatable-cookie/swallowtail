# v0.5.2 SDK Backport Proof

Status: preparatory patch recipe against released `v0.5.1`; not a candidate
release or compatibility claim.

## Frozen Base

The recipe applies only to annotated tag `v0.5.1` with tag object
`97a6933abe13b2e8f05441e1ab950962e0683e65`, peeled commit
`e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`, and source tree
`d375b3227985e8e552ba9346d8f9b8631936db5f`. The backport does not merge
current `main` into that source.

The reviewable text patch, source manifest, exact changed-file inventory, and
runner live under
[`scripts/release/backports/sdk-v0.5.2/`](../../scripts/release/backports/sdk-v0.5.2/).
The manifest pins the expected resulting tree and patch SHA-256. The runner
refuses a different tag, commit, tree, dirty checkout, or an already-applied
patch. Its self-check applies the patch twice to fresh exact-base clones and
compares tree and file inventory.

Apply it only in a fresh task-owned clone:

```sh
scratch=$(mktemp -d)
git clone --no-checkout https://github.com/inflatable-cookie/swallowtail.git "$scratch/source"
git -C "$scratch/source" fetch --no-tags --depth=1 origin refs/tags/v0.5.1:refs/tags/v0.5.1
git -C "$scratch/source" checkout --detach e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27
bash scripts/release/backports/sdk-v0.5.2/runner.sh apply --source "$scratch/source"
```

The final command stages the exact patch in that clone and prints its tree
hash. To reconstruct the refusal and independent-reapplication checks, run
`effigy check:sdk-patch-backport` from the recipe checkout.

## Minimal Closure

The recipe carries selected source and regression changes from these reviewed
fixes:

- PR 410, commit `394a9b18253c567858fb8044305ce248df777b2e`: split the
  opening/readiness deadline from an optional registered-tool lease deadline.
  The SDK lease lives until session close; each call keeps its own maximum and
  an explicitly supplied lease deadline still bounds calls. Existing
  operation-scoped callers keep their prior default.
- PR 412, commit `d3ffd9a1e94b5b75a7acbbb3e3baa448ffd95124`, with test
  follow-up `32595f1b64e67473393f66c476c1e8528ebd2f64`: decode exact
  per-turn usage and project it as `TokenUsage` with usage provenance.
- PR 415, source commit `a8e7f40178ab8eed7a0a10b1b9282591c0f38c5c`: prevent a
  command sender from registering a pending reply after pump failure has
  already drained pending replies. Without that repair, a failed sidecar can
  leave the sender waiting indefinitely. The backport includes the narrow
  connection, pump, and regression-test closure for this race.

PR 415 is included because the close-hang/failure regression is part of the
acceptance proof. The PR's release notes, contract edits, and Effigy selectors
are not included. No PR 405 helper was needed. `files.txt` is the complete
allowed patch inventory; the runner rejects any other resulting file set.

## Released SDK Evidence

The released adapter pins Claude Agent SDK `0.3.259`, native binary `2.1.259`,
Node `22.23.2`, wire `swallowtail-claude-agent-sdk-jsonl-v1`, and sidecar source
tag `swallowtail-claude-agent-sdk-sidecar@0.4.4`. The patch leaves those points
unchanged.

The exact published SDK artifact is
[`@anthropic-ai/claude-agent-sdk@0.3.259`](https://www.npmjs.com/package/@anthropic-ai/claude-agent-sdk/v/0.3.259),
tarball
[`claude-agent-sdk-0.3.259.tgz`](https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk/-/claude-agent-sdk-0.3.259.tgz),
with integrity `sha512-5VJSzHQTAPFl2BytZSgyL0Xtdi3I7CeajEhO4KTvm6bx4nt1OIp+IHx78MuurA4Pp/t9UEPa3cWz8Q55Pi9MYw==`.
Its frozen declarations report per-turn `usage` on both success and error
results, including input, output, cache-read, and cache-creation counters.
The sidecar sends those four bounded counters, retaining absent cache counters
as `null`; malformed or missing usage fails closed. It does not emit
`modelUsage`, whose SDK documentation describes cumulative pipeline usage.

SDK `Query.getContextUsage({ detail })` is also present at `0.3.259` (added in
`0.3.257`), but it reports context occupancy through a separate API. This
patch does not claim context-window usage from per-turn token totals and does
not add a context-usage wire field. No `0.3.284` live evidence is used for the
released `0.3.259` route.

Sources: the versioned [SDK type declarations](https://github.com/anthropics/claude-agent-sdk-typescript/blob/v0.3.259/sdk.d.ts)
and [SDK changelog](https://github.com/anthropics/claude-agent-sdk-typescript/blob/v0.3.259/CHANGELOG.md).

## Compatibility Assessment

The tagged public API inventories under
`release-baselines/public-api-0.5.1/` are the baseline. The check selector
regenerates API output for `swallowtail-runtime`, `swallowtail-host-local`,
and `swallowtail-adapter-claude-agent` using `cargo-public-api 0.52.0` and
`nightly-2026-08-05`. The lease correction adds methods and an explicit
preparation entry point; it removes no prior method or default. Usage uses the
existing `TokenUsage` and `ProviderObservation::Usage` vocabulary. The check
reports exact additions and rejects removals. This scoped inventory does not
replace the later 40-package candidate API gate.

The recorded inventory found seven additions in `swallowtail-runtime` and no
additions or removals in host-local or the Claude adapter:
`PreparedRegisteredToolBinding::lease_deadline`,
`RegisteredToolBridgeLease::{lease_deadline,next_call_deadline}`,
`RegisteredToolOpenRequest::{lease_deadline,with_lease_deadline}`,
`RegisteredToolOperationKernel::next_call_deadline`, and
`RegisteredToolPreparation::prepare_with_lease_deadline`.

The patch changes no Cargo manifest, lockfile, workspace version, release
baseline, route binding, route matrix, sidecar version, protocol version, or
dependency. The workspace declares Rust `1.95.0`; this patch adds no dependency
or release-floor change. Candidate-wide dependency advisory, license/source,
MSRV, and current-stable checks remain required.

## Deterministic Proof

Effigy selectors on the main-based recipe run against an isolated checkout
derived from the exact tag:

- `effigy format:sdk-patch-backport` checks formatting in the three touched
  packages and the external consumer harness.
- `effigy check:sdk-patch-backport` proves clean-base refusal/application,
  fresh reapplication identity, package compilation, scoped public API
  compatibility, and an external Git-source prepared-facade consumer.
- `effigy validate:sdk-patch-backport` runs the bounded lease, late call,
  explicit deadline, permission wait, close/failure, usage/cache decoding, and
  untouched-route guard fixtures. Its task-local nextest profile terminates a
  hung test after two 60-second slow periods.

The external consumer fixture includes a locked dependency graph and pins the
derived source commit. The runner checks the fixture hashes, substitutes only
the isolated source URL, and uses `cargo run --locked`. It type-checks the
additive lease surface and `TokenUsage`, then calls
`prepare_claude_agent_sdk_session` without opening a provider session. Its
Cargo metadata check requires every Swallowtail dependency to resolve from the
same exact Git revision. Fixtures and build output stay under a fresh
`mktemp -d` root. No live provider, credential, consumer repository, or
released-line checkout is touched.

The runner uses the exact patch file and records the expected tree in
`manifest.json`. The selectors print the computed source tree and their actual
test or compile results. This PR carries the reproducible proof recipe; the
release-line candidate task must apply the reviewed patch independently and
record its candidate-specific results.

Recorded main-based proof:

- `effigy format:sdk-patch-backport`: exit 0; formatting checked on tree
  `35252ecf3dcb4254f66a9ed16caf813d2397f5f7`.
- `effigy check:sdk-patch-backport`: exit 0; exact base
  `e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`, derived tree
  `35252ecf3dcb4254f66a9ed16caf813d2397f5f7`; fresh applications agreed,
  no API removals, package compile passed, and the external prepared-facade
  consumer passed at source commit
  `388cbe6b5e67c585a76d5dfa14433b166f799109`.
- `effigy validate:sdk-patch-backport`: exit 0 on derived tree
  `35252ecf3dcb4254f66a9ed16caf813d2397f5f7`; 22 targeted tests passed:
  5 runtime, 1 host-local, and 16 adapter/integration tests.
- `effigy qa:docs`: exit 0; links, docs indexes, front doors, release identity,
  and guide coverage passed.
- `effigy skill run northstar/retired-concepts`: exit 0; 3 retired concepts
  checked.

## Remaining Release Gates

This proof does not qualify or prepare a release candidate. The release-line
task still needs an independent exact-base application and review, candidate
wide metadata and 40-package API checks, package/source archive and docs
checks, dependency/security and MSRV checks, route and feature truth review,
external source-consumer preparation, and candidate-wide deterministic QA.
The candidate then needs qualifying hosted CI on its exact head and the
Queue-owned milestone review. Any live application smoke remains separately
authorized. Tag creation or push requires operator authority naming the
final exact SHA.
