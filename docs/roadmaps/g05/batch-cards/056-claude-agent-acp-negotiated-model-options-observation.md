# 056 Claude Agent ACP Negotiated Model-Options Observation

Status: complete
Owner: Tom
Created: 2026-09-02
Milestone: `../022-claude-agent-dual-route-parity.md`
Depends on: completed cards 053-054; Research 279; Contract 061

## Goal

Publish exact negotiated model-options observation for projected
`claude-agent.acp` session open without creating a catalogue, selectable
control, new shared type, or lifecycle change.

## Scope

1. Parse exactly one `configOptions[id=model]` select option after existing
   model confirmation. Require bounded unique option values and labels, an
   exact current value, and current membership in the options.
2. Retain valid evidence on `ClaudeAgentSessionHandle` through the existing
   `InteractiveSessionHandle::negotiated_model_options` seam and existing
   `NegotiatedSessionModelOptions` runtime type.
3. Preserve `open_session` behavior and signature. On the preserved path,
   absent or malformed optional evidence yields no snapshot. On
   `open_session_with_projection`, malformed, duplicate, unbounded, or
   current-missing evidence closes the opened session and fails.
4. Publish only observation row
   `feature.negotiated-model-options-observation` from the active-session
   source on projected open. Prepared and active source ids remain distinct.
   No row on load/resume, catalogue, preserved open, or prepared evidence.
5. Follow the Cline parser and projected-open proof shape where semantics are
   identical. Keep Claude-specific diagnostics and lifecycle ownership inside
   the adapter.
6. Add exact provider-free cases for matching, absent, malformed, duplicate,
   unbounded, current-not-in-options, source disagreement, preserved parity,
   projected cleanup, load/resume omission, and catalogue negatives.
7. Update the Claude guide, route feature matrix, milestone/card closeout,
   changelog, and log only for the delivered observation.

## Out Of Scope

Model catalogue or selection; mid-session model/effort control; mode mutation;
read-write interactive access; persistent permission choices; terminal/Bash;
MCP; auth; session-management expansion; attachments; commands; steering;
subagents; SDK route; shared runtime/core changes; claims; package pins; live
provider work; release preparation.

## Acceptance Criteria

- [x] exact bounded negotiated model options survive session open
- [x] existing runtime type and handle seam are reused with no shared API change
- [x] preserved open remains behavior-compatible for absent and malformed optional evidence
- [x] projected invalid evidence closes and fails without a contribution
- [x] only projected active open publishes the observation row
- [x] load/resume and catalogue remain negative
- [x] all named mutations and cleanup counterexamples fail provider-free
- [x] public API and god-file baselines hold

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
effigy package:api
effigy qa:routes
effigy qa:docs
effigy qa:northstar
effigy --json scan god-files
git diff --check
```

Do not run live probes, provider sessions, release commands, or broad workspace
tests.

## Review Oracle

Invariant: the contribution is exact active-session observation, never a
catalogue or control.

Smallest counterexample: a duplicate option is accepted, preserved open starts
failing on optional evidence, or a prepared/load/resume source emits the row.

## Stop Conditions

Stop if delivery requires a new shared type, public control, claim change,
load/resume inference, provider contact, or overlap with card 055 beyond the
named adapter index/manifest surfaces.

## Auto-Continuation

No. Exact-head review, then merge before card 055's same-repository restack.
