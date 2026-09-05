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

### Card 101 Manifest

Promoted planning commit: the `main` commit that introduces this section.
Card 101 becomes ready the moment card 100 merges with its live acceptance
recorded; the coordinator dispatches it on that notice without a further
Chatterbox round trip. Operator prepare authorization was granted
2026-09-05 on the `v0.4.1` standing-grant pattern.

| Field | Card 101 |
| --- | --- |
| Readiness | planned until card 100 merges; then ready without further promotion |
| Prerequisites | card 100 merged with the live turn recorded (system/init first, truthful close); Effigy local install at or beyond the per-gate-log fix; clean canonical base; no open feature PR |
| Completion conditions | `docs/releases/0.4.2.md` and index entry authored from card 100's result, the merged Contract 061 tranches (candidates B, K, L and the 767/767 completion), card 081's Bash mediation, and cards 093/094/095; patch class stated from the semantic API diff (all changes since `v0.4.1` are additive: card 100 adds 5 baseline lines, the tranches add contribution methods); read-only release status inferring `0.4.2` with the three-mutation plan; lock in sync before the first `--locked` gate; exactly one `effigy --json release prepare --yes --check-gates --version 0.4.2` with the per-gate logs kept as evidence; NO separate frozen-tree rerun; distinct `0.4.2` semantic baseline, route inventory, and dependency graph without touching `0.4.1` files; the four gate scripts and the consumer front-door script repointed to `0.4.2` exactly as card 091 did; candidate PR; independent review and workflow-dispatch CI in parallel; merge on both green |
| Owned mutable paths | as card 091's amended manifest, with `0.4.2` in place of `0.4.1`: workspace `Cargo.toml` versions through the prepare transaction; `Cargo.lock` workspace entries; `CHANGELOG.md` promotion; `docs/releases/0.4.2.md`; `docs/releases/README.md` current entry; `release-baselines/public-api-0.4.2/**`, `production-routes-0.4.2.txt`, `internal-dependencies-0.4.2.tsv`; `.release-prepared.json`; `scripts/check-public-api.sh`, `scripts/check-package-metadata.sh`, `scripts/check-provider-route-matrix.sh`, `scripts/check-consumer-front-door.py`, `scripts/README.md`; root `README.md` release-posture lines; this card's `## Result`; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md`, the `docs/releases/README.md` historical lines |
| Forbidden paths | every `crates/**/src` and test path; every `0.4.1` and earlier baseline; contracts; architecture; guides; matrices; version claims; any feature change |
| Approved concurrent siblings | none; feature freeze from card 100's merge until card 102 stops (card 082 stays paused) |
| Serial edges | card 102 follows the merged candidate with green exact-SHA CI and the Bovine packet |
| Worker capability class | release-preparation worker with Effigy release discipline; no credentials; no tag authority |
| Acceptance evidence | read-only status output; prepare JSON report with all gates green and per-gate logs; the `0.4.2` baseline files; PR head and merged SHA; workflow-dispatch run id at that SHA |
| Review oracle | one exact tree supports every candidate statement; release note status line reads `Status: candidate; not tagged` |
| Stop conditions | a gate fails (Effigy rolls back; report with the captured gate log; a transient renewed by Chatterbox under the standing grant; a real defect stops); release status infers anything but `0.4.2`; an open feature PR |
| Escalation owner | operator via Chatterbox; coordinator for mechanical blockers |

### Card 102 Manifest

Promoted planning commit: the `main` commit that introduces this section.
Card 102 becomes ready when card 101's candidate merges with green exact-SHA
CI; Chatterbox fills the candidate SHA into the Bovine packet and relays it
to the Acowtancy coordinator; the Swallowtail coordinator runs the source
consumer in parallel. No further Chatterbox promotion is needed to start.

| Field | Card 102 |
| --- | --- |
| Readiness | planned until card 101's candidate merges with green exact-SHA CI; then ready |
| Prerequisites | merged `0.4.2` candidate SHA; the Bovine Desktop smoke packet at `/Users/tom/Dev/projects/acowtancy/docs/handoffs/20260905-desktop-swallowtail-v042-smoke-packet.md` with the SHA filled in; the operator's host Node step (present `~/.local/bin/node` 22.23.2 first for the run) |
| Completion conditions | `effigy package:source-consumer` passes from a clean detached checkout of the merged SHA; one Bovine Desktop editing turn on `claude-agent.sdk` with the `read_write(AcceptEdits)` profile on a smoke branch pinned to the SHA: session opens with init evidence and no `open_rejected`, the in-workspace file lands with no per-edit prompt, the outside-workspace write is refused in the transcript, and the close is clean under card 100's instrumentation; the tag decision request compiled with the exact SHA |
| Owned mutable paths | this card's `## Result`; `PAPERCUTS.md` append only. The Desktop smoke branch and its one log are the Acowtancy lane's, never merged there |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md`, the release note status line at tag time |
| Forbidden paths | every crate; every baseline; the candidate itself after merge; any Swallowtail edit beyond the result |
| Approved concurrent siblings | none; feature freeze holds until this card stops |
| Serial edges | the operator's exact-SHA tag decision follows |
| Worker capability class | Swallowtail side: release-evidence worker running the source consumer; Desktop side: the Acowtancy lane under its own coordinator |
| Acceptance evidence | source-consumer output at the exact revision; the Desktop run log (sanitised) with the four acceptance points; retry budget one attempt plus one authorized retry for environment faults only |
| Review oracle | both proofs use the exact merged SHA; no provider-free substitute counts as the editing turn |
| Stop conditions | the Desktop turn fails on a route defect (report the typed code; the fix is a further patch, not a retag); a retry beyond the budget |
| Escalation owner | operator via Chatterbox; Swallowtail coordinator for the source consumer; Acowtancy coordinator for the Desktop run |
