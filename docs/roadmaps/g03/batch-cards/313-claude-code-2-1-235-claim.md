# 313 Claude Code 2.1.235 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../100-claude-code-2-1-235-useful-newer.md`
Depends on: card 312; Research 162

## Goal

Raise the Claude Code headless and response-only qualified ceilings from
`2.1.234` to official `2.1.235` on the existing stream-JSON behaviors.
Keep later stables AllowUnverified.

## Scope

1. Extend Maintained headless `2.1.220..=2.1.235` and response-only
   `2.1.227..=2.1.235`. Keep AllowUnverified and the empty deny-list.
2. Move the synthetic later-stable UnverifiedNewer to `2.1.236`.
3. Refresh focused tests, matrices, Claude Code guides, architecture, and
   Contract 039 where it names the current bound.

## Out Of Scope

- flattening onto Claude Agent ACP
- mapping spellcheck, unused help flags, or sdk-tools artifact fields
- Gemini or other Research 159 families
- capturing a live prompt or authenticated stream
- install, update, or publication

## Acceptance Criteria

- [x] official `2.1.235` classifies as Qualified Maintained on both axes
- [x] `2.1.220` and `2.1.227` remain Qualified
- [x] `2.1.236` remains permitted UnverifiedNewer
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

- stop if card 312 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `2.1.235` is no longer the official stable point

## Auto-Continuation

No. After closeout, implement Grok `1.0.5` useful-newer qualification.
Ignore alpha `1.0.6`. Gemini stays deferred.

## Evidence

- Research 162
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.235/`
- Decoder specimens remain `claude-code-2.1.220`, `claude-code-2.1.227`,
  and `claude-code-2.1.228`
- headless latest qualified = `2.1.235`
- response-only latest qualified = `2.1.235`
- synthetic later-stable UnverifiedNewer is `2.1.236`
