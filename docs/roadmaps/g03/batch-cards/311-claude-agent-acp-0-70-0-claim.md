# 311 Claude Agent ACP 0.70.0 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../099-claude-agent-acp-0-70-0-useful-newer.md`
Depends on: card 310; Research 161

## Goal

Raise the `claude-agent.acp-adapter` qualified ceiling from `0.69.0` to
official `0.70.0` on existing initialize-meta-extensions-v7. Keep later
stables AllowUnverified.

## Scope

1. Extend Maintained `0.66.0..=0.70.0` on
   `claude-agent.acp.initialize-meta-extensions-v7`. Keep AllowUnverified.
2. Keep unpublished `0.58.0` excluded. Move synthetic later-stable
   UnverifiedNewer to `0.70.1`.
3. Refresh focused tests, matrices, Claude Agent guides, architecture,
   and contracts that name the ceiling.

## Out Of Scope

- Claude Code axes
- mapping Providers API, goal, Air, file-change, nested transcript, or
  host-owned steering fallback
- Gemini or other Research 159 families
- capturing a live prompt or authenticated initialize
- install, update, or publication

## Acceptance Criteria

- [x] official `0.70.0` classifies as Qualified Maintained
- [x] host `0.63.0` remains Qualified
- [x] `0.58.0` remains incompatible
- [x] `0.70.1` remains permitted UnverifiedNewer
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

- stop if card 310 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `0.70.0` is no longer the official stable point

## Auto-Continuation

No. After closeout, implement Claude Code `2.1.235` useful-newer
qualification (headless and response-only stay one family). Gemini stays
deferred.

## Evidence

- Research 161
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.70.0/`
- Decoder specimens remain `claude-agent-acp-v0.53.0-v0.61.0` and
  `claude-agent-acp-v0.62.0-v0.64.0`
- latest qualified = `0.70.0`
- v7 behavior = `claude-agent.acp.initialize-meta-extensions-v7`
- synthetic later-stable UnverifiedNewer is `0.70.1`
