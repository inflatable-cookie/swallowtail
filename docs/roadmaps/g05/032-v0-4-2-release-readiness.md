# g05.032 v0.4.2 Release Readiness

Status: ready; card 100 repair is ready; cards 101 and 102 serial behind it; compressed lane from the start
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Depends on: Contract 036; immutable `v0.4.1` at `c3cce750`; the Bovine failure report; the g05.030 precedent and its compression decision
Vision tags: source release, consumer proof, Claude route

## Purpose

Ship the fix for Bovine Desktop's first live open of `claude-agent.sdk`
as patch `v0.4.2`, fast, with the live proof that `v0.4.1` lacked. The
release lane runs compressed from the outset: no duplicate gate reruns,
review and CI in parallel, source consumer as the mechanical proof, and one
live consumer open as the acceptance this defect class needs.

## Runway

1. Card 100 repairs open diagnostics, effective-model evidence, and Node
   newer-allowed at open, with one operator-authorized live open.
2. Card 101 prepares the candidate: patch audit folded in (the delta since
   `v0.4.1` is card 100 plus whatever merged tranches are green), one
   Effigy prepare transaction with per-gate logs under the updated Effigy,
   candidate PR with review and exact-SHA CI in parallel, merge.
3. Card 102 runs the source consumer and one Bovine Desktop editing session
   on the candidate, then stops for the operator tag decision.

## Release Boundary

No card creates or pushes a tag. The operator authorizes the exact SHA. No
publication, GitHub Release, binaries, sidecars, installers, or consumer
mutation.

## Batch Cards

- [100 Claude SDK Open Diagnostics And Live-Open Repair](batch-cards/100-claude-sdk-open-diagnostics-and-live-open-repair.md) — ready
- [101 v0.4.2 Candidate Preparation](batch-cards/101-v0-4-2-candidate-preparation.md) — planned; serial after card 100; operator prepare authorization granted 2026-09-05 on the standing-grant pattern
- [102 v0.4.2 Consumer Proof And Tag Gate](batch-cards/102-v0-4-2-consumer-proof-and-tag-gate.md) — planned; serial after card 101; Bovine editing session as the smoke

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.

| Field | Card 100 |
| --- | --- |
| Readiness | ready |
| Prerequisites | `v0.4.1` on `main`; the Bovine report; card 082 paused in its workspace |
| Completion conditions | spawn hook takes the SDK's single `SpawnOptions` object with `signal` forwarded; account projection verified against the frozen 0.3.259 `sdk.d.ts`; fake SDK calls the hook and returns `accountInfo()` in the real shape; sidecar codes surfaced on every rejection; effective model published from init evidence; Node newer-allowed at open with an `UnverifiedNewer` record; provider-free fixtures for all three; one live open recorded with the real `system.model` and account projection; guide, matrices, changelog, additive baseline; one PR |
| Owned mutable paths | `crates/swallowtail-adapter-claude-agent/src/sdk/**`; `crates/swallowtail-adapter-claude-agent/sidecar/**`; `crates/swallowtail-adapter-claude-agent/tests/**`; `release-baselines/public-api-0.4.1/swallowtail-adapter-claude-agent.txt` regenerated additively; `docs/guides/claude-agent-sdk-prepared-integration.md`; the `claude-agent.sdk` matrix cells; `CHANGELOG.md` `[Unreleased]`; this card's `## Result`; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md` |
| Forbidden paths | every other crate; `claude_code_*` and ACP modules; contracts; the SDK wrapper and native pins (only the Node open check changes); card 082's model-change surfaces |
| Approved concurrent siblings | g05.009 cards 097-099; card 094 remainder. Card 082 is paused, not concurrent, because it shares the sidecar |
| Serial edges | card 082 rebases and resumes after card 100 merges; card 101 follows card 100 |
| Worker capability class | Rust plus Node sidecar implementation worker; frontier-tier; the live open uses the operator's first-party subscription login on this host, authorized 2026-09-05 |
| Acceptance evidence | fixtures for the three changes; the live open log with `system.model`, `apiKeySource`, `apiProvider` labels only; additive API diff |
| Review oracle | the card's invariant |
| Stop conditions | the live open reveals a rejection that needs a design decision (report the code; return to Chatterbox) |
| Escalation owner | operator via Chatterbox; coordinator for mechanical blockers |

## Acceptance

- [ ] Bovine's `read_write(AcceptEdits)` open succeeds on the candidate and
      a multi-turn edit lands in the leased cwd
- [ ] every open failure names its sidecar code
- [ ] `v0.4.2` tagged on the operator's exact-SHA authorization
