# 238 Claude Code Headless 2.1.234 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../075-claude-code-headless-2-1-234-useful-newer.md`
Depends on: card 237; Research 132

## Goal

Raise the `claude-code.headless-stream-json` qualified ceiling from exact
`2.1.220` to exact `2.1.234` as a compatible extension of
`claude-code.headless.stream-json.v1`.

## Scope

1. Extend the existing Maintained segment through `2.1.234`. Keep baseline
   `2.1.220` and AllowUnverified.
2. Keep behavior `claude-code.headless.stream-json.v1`. Do not add a
   milestone revision.
3. Refresh focused tests, matrices, and the Claude Code guide version text.
4. Leave later stables visible UnverifiedNewer (synthetic `2.1.235`).

## Out Of Scope

- `claude-code.response-only-stream-json`
- Gemini or other Research 127 families
- capturing a live stream-JSON prompt
- install, update, or publication

## Acceptance Criteria

- [x] `2.1.220..=2.1.234` classifies as Qualified Maintained
- [x] `2.1.219` remains incompatible
- [x] `2.1.235` remains permitted UnverifiedNewer
- [x] response-only claim bounds stay `2.1.227..=2.1.228`
- [x] focused Claude Code proof and package verify pass
- [x] matrices and guides name the new headless ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 237 did not name compatible-extension
- stop if live provider work would be required to close the claim
- stop if `2.1.234` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a time.
Do not start response-only inside this card.

## Evidence

- Research 132
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.234/`
- Decoder specimen remains `claude-code-2.1.220`
- `CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION` = `2.1.234`
- Behavior: `claude-code.headless.stream-json.v1`
- synthetic later-stable UnverifiedNewer is `2.1.235`
