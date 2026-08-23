# 2026-08-23 g04.047 Gemini Live Output-Token Maximum Compiled

## Change

- reassessed the remaining promoted per-route feature inventory after g04.046
- selected `gemini.live` caller output-token maximum
- compiled g04.047 and cards 130-132 as one serial evidence-first worker lane
- reserved Research 194 and the route-local closeout before dispatch

## Decision

The production Gemini Live route already fixes exact model
`gemini-3.1-flash-live-preview`, hosted raw-WebSocket transport, project
authorization API-key access, asymmetric PCM, output transcription,
caller-selectable thinking, and one provider-planned rollover. The generic
realtime request already carries an optional positive output-token maximum,
but Gemini preparation and setup do not bind or claim it.

Current official references say Live setup accepts a `GenerationConfig`,
define `GenerationConfig.maxOutputTokens`, and list a 65,536 output-token limit
for this model. The generic reference also warns that not every parameter is
configurable for every model. Card 130 must therefore freeze the exact composed
field, model/facade applicability, positive domain, omission,
reasoning/rollover/restoration behavior, and Contract 029 revision. Cards
131-132 may continue only for a non-empty Research 194 deliver-now set.

Client-side truncation, new shared runtime fields, reasoning changes, other
Gemini routes or models, live calls, and later feature families are excluded.
No implementation, capability, matrix, or compatibility claim was made during
compilation.

## Next

Execute g04.047 cards 130-132 serially in one isolated worker worktree and open
one PR.
