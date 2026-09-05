# 081 Claude SDK Bash Under Mediation

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-05
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: `v0.4.1` card 080 seam; Contracts 023 and 041; the 2026-09-04 Bovine requirement item 3

## Goal

Admit `Bash` on `claude-agent.sdk` behind consumer-mediated per-call
admission with the command input visible to the consumer, so an application
can deny some commands and allow others (Bovine denies Git, allows
validators). Ambient posture; no containment claim; nothing auto-allowed.

## Design Point

Card 080's sidecar deliberately keeps tool input off the private wire: the
callback record carries only the tool name and id, and the input is retained
in the sidecar and returned unchanged on allow. That is right for `Edit` and
`Write`, whose consumer decision is about the tool, not the payload. For
`Bash` the decision is about the command, so this card adds a bounded,
typed command view to the callback record for `Bash` only: the command
string, its byte length, and the `description` field the SDK supplies, each
bounded to the existing extension text maximum with an explicit
`truncated` flag. The full input still stays in the sidecar and is returned
unchanged on allow. Nothing is parsed, normalised, or classified; the
consumer owns the decision.

## Scope

1. Add `ClaudeAgentSdkTool::Bash` to the profile enum. `BashOutput`,
   `KillShell`, `NotebookEdit`, `Task`, `WebFetch`, and `WebSearch` remain
   never-available.
2. Require `ResourceAccess::ReadWrite` on the lease when `Bash` is admitted;
   reject at preparation otherwise (Bash can write regardless of the tool
   set).
3. Sidecar: when `Bash` is admitted, pass it in `tools`; keep `allowedTools`
   unset; extend the callback record with the bounded command view for
   `Bash` calls only; every `Bash` call waits on the host decision under
   every permission mode including `acceptEdits` (the SDK auto-approves
   edits in that mode, not shell).
4. Rust: surface the command view on the route-local callback type as
   typed fields; the existing allow/deny decision path is unchanged.
5. Fake-SDK fixture: drive a Bash call with a known command and
   description; prove the consumer sees the command intact before it runs,
   a denied command produces no process and no disk change, an allowed
   command runs with input unchanged, and `Bash` is denied without a host
   round trip when not admitted. Prove the view's truncation at the bound.
6. Guide, `claude-agent.sdk` matrix cells, `CHANGELOG.md` `[Unreleased]`,
   and the adapter API baseline regenerated additively.
7. Stop after one reviewable PR.

## Out Of Scope

Background shells (`BashOutput`, `KillShell`); any containment or sandbox
claim; command classification or allowlists inside Swallowtail; other
cards' surfaces; live Claude calls.

## Acceptance Criteria

- [ ] `Bash` never runs without a host `allow` under any permission mode
- [ ] the consumer sees the command and description before the decision
- [ ] input returned to the SDK on allow is byte-identical to the original
- [ ] read-only lease plus `Bash` is rejected at preparation
- [ ] default profile behaviour unchanged; API diff additive only

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:guides`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: no shell command runs without the consumer having seen it and
said yes. Smallest counterexample: a Bash call allowed by `acceptEdits`
without a callback, a callback whose command view differs from the input the
SDK receives, or `Bash` admitted on a read-only lease.

## Auto-Continuation

No. Stop after one reviewable PR for exact-head review.

## Stop Conditions

The SDK auto-approves `Bash` in a mode this card admits (record and return
to Chatterbox); the command view cannot be bounded without lossy
transformation of what the SDK executes.

## Result

- Added explicit `ClaudeAgentSdkTool::Bash` admission under a read-write lease;
  the default read-only and existing editing profiles remain unchanged.
- Bash callbacks carry bounded command and description views with a
  truncation flag. The sidecar retains full input and returns it unchanged on
  allow; `default`, `plan`, and `acceptEdits` all remain host-mediated.
- Provider-free proofs cover Rust framing, denial, truncation, unchanged input,
  fake command execution, no denied filesystem effect, and unadmitted Bash
  denial without a host round trip. No credentials or live provider calls were
  used.
- Validation passed: 71 library tests, 58 SDK-driver tests, 13 sidecar-asset
  tests; `effigy validate:focused` ran 334 package tests; affected-package,
  API, route, guide, docs, Northstar, format, and diff gates passed.
- Public API baseline is additive-only: `ClaudeAgentSdkTool::Bash`.
