# 086 Claude SDK Discovery Identity

Status: complete; merged through PR 241 as `d127837f8c3c39b377b5eae35aa4d9aebedcae09`
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-06
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: Research 280; the SDK discovery in `sdk/selection.rs` and `sdk/asset.rs`; Contract 029

## Goal

Freeze evidence on running the SDK sidecar against a discovered host-installed native Claude Code and against discovered or minimally bundled Node, across published points, without changing any claim. The output is a research document that card 087 can act on.

## Scope

1. Today the sidecar passes `pathToClaudeCodeExecutable: nativeBinary`, the SDK-bundled native. Probe three configurations with initialize-only sessions (no prompt, no turn): bundled native; host-installed `claude` at the current stable; host `claude` at each published point in `2.1.227..=2.1.259`. Record `system/init`, `supportedModels`, `harnessSchema`, and any refusal, per configuration.
2. Node: host `/opt/homebrew/bin/node` 26.x, `~/.local/bin/node` 22.x, and the pinned `22.23.2`; record which the sidecar accepts at open and which the SDK's own `engines` admits.
3. Classify each (wrapper, native, node) triple as compatible, refused-with-code, or surface-changed, with the exact evidence line. This is the admitted range candidate for card 087.
4. Write `docs/research/2xx-claude-agent-sdk-discovery-identity.md` in the Research 280 shape (question, method, inventory, invariants, decision, falsification, withheld). No claim, guide, matrix, or pin changes.

## Out Of Scope

Any code change; any claim change; any live turn. Initialize-only probes are not turns; if a probe would send a prompt, stop.

## Acceptance Criteria

- [x] every probed triple recorded with evidence lines; none inferred
- [x] the decision section names the candidate admitted ranges per axis and the incompatible reasons
- [x] no source, claim, or pin changed
- [x] research doc indexed in `docs/research/README.md`

## Validation

- `effigy qa:docs`
- `git diff --check`

## Review Oracle

Invariant: every compatibility statement in the research cites a probe line. Smallest counterexample: a range asserted from a changelog.

## Stop Conditions

A probe requires a live turn or credentials beyond the operator's existing subscription (stop; operator authorization via Chatterbox).

## Auto-Continuation

No. Stop for exact-head review.

## Result

Research 287 records the Card 086 evidence boundary. The existing open-only
evidence covers bundled native 2.1.259 with Node 22.23.2 and 26.7.0,
including the retired repo-side account_not_subscription diagnostic,
initialize controls, supported-model row count, and the absence of system/init;
the stronger prior Node 22.23.2 live evidence is cited but remains outside this
card's initialize-only acceptance. Host claude 2.1.258 and older native points
remain withheld because Card 100's resumed evidence at lines 261-272 covers
Node 22.23.2 with the real SDK/native path, not a host-native crossing. No new
credential-dependent probe, prompt, or live turn was run.

No candidate admitted range was produced. Production claims, pins, source,
guides, matrices, and baselines are unchanged. Independent exact-head review
approved the final research record at `34841818`, and hosted/docs validation
passed. Card 087 remains gated because
the required system/init evidence needs the first query message, which is a
live turn under this sidecar and this card's stop conditions.
