# 2026-09-02 g05.022 Card 056 Claude Agent ACP Model-Options Observation

Status: complete; provider-free; one PR; no merge
Owner: Tom

## Result

Card 056 publishes exact negotiated model-options observation on projected
`claude-agent.acp` session open. After existing model confirmation, the adapter
parses one `configOptions[id=model]` select with `category=model`, bounded
unique values/labels, exact current value, and current membership. Valid
evidence is retained on the session handle through the existing
`NegotiatedSessionModelOptions` type and
`InteractiveSessionHandle::negotiated_model_options` seam.

Required missing `configOptions[id=model]` fails both public opens through the
existing confirmation path (`swallowtail.claude_agent.acp.config_option_missing`)
with equal cleanup and no contribution. Snapshot-detail malformation that still
confirms `currentValue` is no snapshot on preserved `open_session` and
close+fail on `open_session_with_projection` with
`swallowtail.negotiated_model_options.invalid` and no contribution. Successful
open has no Absent observation state: confirmation already requires the model
entry. Only the projected active source emits
`feature.negotiated-model-options-observation`. Prepared contribution, load,
resume, and catalogue stay negative. No shared runtime/core or public control
changed. The accepted seam is `session().negotiated_model_options()`; the
outcome did not keep a pass-through accessor.

No provider session, live probe, claim, package pin, release mutation, tag, or
merge was authorized.

## Shared Surfaces

Card 055 continues in parallel and restacks after this PR merges. Touched
merge-order surfaces:

- `docs/guides/claude-agent-prepared-integration.md`
- `docs/guides/provider-solution-feature-matrix.csv` notes only
- `CHANGELOG.md` Unreleased
- g05.022 milestone, card 056, batch-card index, generation index, roadmaps
  Next Task
- this log and `docs/logs/README.md`
- managed ACP session handle forwards `negotiated_model_options`

`Cargo.toml` and `lib.rs` exports are unchanged. The unreleased Claude Agent
public API baseline has no new method from this card.

## Validation

Card-exact selectors on this worker head:

- `cargo fmt -p swallowtail-adapter-claude-agent`
- `effigy validate:focused swallowtail-adapter-claude-agent` — 217 tests passed
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api` — no additive public method on the unreleased Claude Agent baseline; the outcome pass-through accessor was removed; no shared-package change
- `effigy qa:routes` — 40-solution / 48-route matrices passed
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files` — 377 findings; `acknowledgement.rs` stayed out of the warning set; no new adapter god-files
- `git diff --check`

No provider, live, release, tag, or merge work.

## Authority

- [card 056](../roadmaps/g05/batch-cards/056-claude-agent-acp-negotiated-model-options-observation.md)
- [Research 279](../research/279-claude-agent-acp-capability-census-and-tranche-selection.md)
- [ACP parity delivery gate](../triage/2026-09-02-claude-agent-acp-parity-gate.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
