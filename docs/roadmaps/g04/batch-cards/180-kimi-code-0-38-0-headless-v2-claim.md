# 180 Kimi Code 0.38.0 Headless V2 Claim

Status: complete
Owner: Tom
Milestone: [g04.064 Kimi Code 0.38.0 Headless V2 Useful Newer](../064-kimi-code-0-38-0-headless-v2-useful-newer.md)
Created: 2026-08-25
Depends on: card 179; Research 211 adapter-private claim decision

## Task

Qualify exact Kimi Code headless `0.38.0` under the Research 211 v2 behavior
revision. If card 179 reaches an incompatible stop or requires a new public
driver/facade, do not execute this card.

## Edit Set

In `crates/swallowtail-adapter-kimi/src/selection.rs` and route selection:

- keep claim id `kimi.headless.executable-window-2`
- keep `AllowUnverified` and baseline `0.29.0`
- keep `0.29.0..=0.37.2` under `kimi.headless.stream-json.v1`
- add exact `0.38.0` under Research 211's admitted private v2 revision,
  expected `kimi.headless.stream-json.v2`
- dispatch or decode by assessed behavior revision; do not accept v2 under v1
- test v1 baseline/ceiling, exact v2, and synthetic `0.38.1`

In tests and fixtures:

- bind the card-179 v2 corpus to decoder, activity, pump, terminal, prepared
  route, discovery, and foundation assertions as warranted
- prove v1 behavior remains unchanged through `0.37.2`
- prove exact `0.38.0` uses only the v2 mapping and mismatched revisions fail
  before provider work
- preserve honest retained-state, recovery, retry, cancellation, tool, stderr,
  exit, malformed-stream, incomplete-stream, and unknown-event truth

In docs:

- update the Kimi prepared-integration guide, route matrix, feature matrix,
  changelog, Research 211, identity/claim logs, roadmap/card indexes, programme,
  and currentness standing-lane checkpoint
- leave historical Research 179 and 210 text intact except links from current
  surfaces; do not erase the recorded correction
- close g04.064 as completed or stopped; keep g04 active

## Validation

```sh
cargo fmt -p swallowtail-adapter-kimi
effigy validate:focused swallowtail-adapter-kimi
effigy package:verify-affected swallowtail-adapter-kimi
effigy qa:routes
effigy qa:northstar
git diff --check
```

Run the relevant docs index and sole-next-action gates. Do not run live probes,
consumer checks, release checks, or broad workspace QA.

## Acceptance

- exact `0.38.0` classifies Qualified Maintained under the admitted v2 revision
- `0.29.0..=0.37.2` remains qualified under v1
- synthetic `0.38.1` remains permitted `UnverifiedNewer`
- decoder and lifecycle claims do not exceed exact source/fixture evidence
- named package, route, Northstar, and index gates pass

Auto-continuation: No. Return the PR for orchestrator review. Do not merge.

## Out Of Scope

- a public lifecycle redesign or shared contract change
- forcing legacy v1 at `0.38.0`
- reasoning effort, ACP, local-server, Platform Chat, or other families
- live provider work, release, merge, generation rollover, or g04 closure
