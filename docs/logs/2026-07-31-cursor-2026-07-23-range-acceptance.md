# 2026-07-31 Cursor 2026.07.23 Range Acceptance

## Changed

- qualified exact `2026.07.23-e383d2b` beside exact
  `2026.07.01-41b2de7`
- fixed build-suffix enforcement so every qualified calendar date retains its
  opaque build identity
- added a two-milestone compatibility corpus across catalogue, ACP, and
  headless routes
- kept the calendar gap unsupported and later dates visibly unverified

## Evidence

The official ACP registry archive matched SHA-256 and reported the expected
version. A prompt-free ACP initialize exchange retained protocol v1 and the
selected capability shape. Focused Cursor validation passed 34 tests across
six binaries. The 54-file package compiled from its extracted archive. Route,
lifecycle, feature, activity, docs, Northstar, and diff-hygiene checks passed.

No installation, login, provider prompt, authenticated catalogue, session,
workspace write, provider mutation, consumer edit, or publication ran.

## Next

Return to the g03 compatibility-maintenance checkpoint. Claude and Gemini
range cards remain paused.
