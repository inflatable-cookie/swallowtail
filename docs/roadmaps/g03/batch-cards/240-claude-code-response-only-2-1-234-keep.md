# 240 Claude Code Response-Only 2.1.234 Keep-Bound Closeout

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../076-claude-code-response-only-2-1-234-provisional.md`
Depends on: card 239; Research 133

## Goal

Keep `claude-code.response-only-stream-json` qualified at
`2.1.227..=2.1.228`. Record host and npm `2.1.234` as the current
UnverifiedNewer observation. Do not deny it. Do not inherit the headless
ceiling.

## Scope

1. Leave selection constants, deny-list, and behavior revision unchanged.
2. Name `2.1.234` as the current provisional observation in the Claude Code
   guide and architecture note.
3. Prove focused tests still treat `2.1.234` as UnverifiedNewer.

## Out Of Scope

- raising latest qualified
- adding a deny-list entry
- `claude-code.headless-stream-json`
- a live response-only prompt
- Oh My Pi, Gemini, or other 127 families

## Acceptance Criteria

- [x] `2.1.227..=2.1.228` remains Qualified Maintained
- [x] `2.1.234` remains permitted UnverifiedNewer
- [x] deny-list stays empty
- [x] headless claim stays `2.1.220..=2.1.234`
- [x] public route truth names current provisional `2.1.234`
- [x] focused Claude Code proof passes

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- `effigy qa:routes` if matrices change
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 239 did not name keep-provisional
- stop if live provider work would be required to close

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a time.
Do not start Oh My Pi inside this card.

## Evidence

- Research 133
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.234/response-only.json`
- Latest qualified remains `2.1.228`
- Behavior: `claude-code.response-only.stream-json.v1`
