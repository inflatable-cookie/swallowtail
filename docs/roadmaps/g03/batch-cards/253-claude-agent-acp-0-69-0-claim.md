# 253 Claude Agent ACP 0.69.0 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../082-claude-agent-acp-0-69-0-useful-newer.md`
Depends on: card 252; Research 139

## Goal

Raise the `claude-agent.acp-adapter` qualified ceiling from `0.64.0` to
`0.69.0`. Extend v6 through `0.65.0`. Add private v7 from `0.66.0`.

## Scope

1. Extend `0.64.0..=0.65.0` on
   `claude-agent.acp.host-steering-form-marker-v6` and mark it
   Deprecated.
2. Add Maintained `0.66.0..=0.69.0` on
   `claude-agent.acp.initialize-meta-extensions-v7`. Keep
   AllowUnverified.
3. Keep unpublished `0.58.0` excluded. Move synthetic later-stable
   UnverifiedNewer to `0.70.0`.
4. Refresh focused tests, matrices, Claude Agent guides, architecture,
   and contracts that name the ceiling.

## Out Of Scope

- Claude Code axes
- mapping goal, Air, file-change, nested transcript, or host-owned
  steering fallback
- Gemini or other Research 127 families
- capturing a live prompt or authenticated initialize
- install, update, or publication

## Acceptance Criteria

- [x] published `0.64.1` through `0.69.0` classify as Qualified
- [x] `0.58.0` remains incompatible
- [x] `0.70.0` remains permitted UnverifiedNewer
- [x] decoder specimens remain the existing corpora
- [x] focused Claude Agent proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 252 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `0.69.0` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a
time and qualify useful-newer support; do not leave the current
host/official stable unqualified. Gemini stays deferred.

## Evidence

- Research 139
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.69.0/`
- Decoder specimens remain `claude-agent-acp-v0.53.0-v0.61.0` and
  `claude-agent-acp-v0.62.0-v0.64.0`
- latest qualified = `0.69.0`
- v7 behavior = `claude-agent.acp.initialize-meta-extensions-v7`
- synthetic later-stable UnverifiedNewer is `0.70.0`
