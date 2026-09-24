# 342 Command Code 1.65.0 Paid-Model Live Acceptance

Status: accepted live evidence; exact `1.65.0` point only
Owner: Swallowtail worker
Created: 2026-09-24
Task: g06.029; planning `9709bc23f17e7ae38318c15c00fcec77f9edb2ce`
Contracts: 029, 043

## Question

Does the exact `command-code.headless` `1.65.0` point complete the selected
live surface on the paid probe model `deepseek/deepseek-v4-flash`, repeating
the g06.006 gate that Research 330 recorded on `1.54.0`?

## Answer

Yes. The one authorized gate ran once on `deepseek/deepseek-v4-flash` with
the stored local Command Code account, and both live probes passed: the
structured one-turn completion and the Contract 043 two-turn private exact-id
continuation.

## Pre-gate boundary

Immediately before any provider operation, the host carried exact `1.65.0`
from `command-code --version`, `dist/index.mjs` SHA-256
`157feefa0140e78f060ef2c1f9c50d10de702196ea229dc73db6bbcc39a0bcbb` matching
Research 339, and local auto-update disabled. The fake-provider corpus on the
prepared facade passed first. No `command-code update`, `/update`,
`--list-models`, login, or catalogue operation ran. The adapter passed
`--no-auto-update` throughout.

Pinned install used official npm `command-code@1.65.0`. Registry `latest`
remained `1.65.0` at the boundary and after the gate. Digests still matched
Research 339 after both probes.

## Observed gate

- Structured run (`probe:command-code-plan`): terminal `Completed` with the
  exact reply text, `Clean` cleanup, and every streamed AgentEvent decoded
  valid. No `malformed_stream`, no retry, no second attempt.
- Interactive continuation (`probe:command-code-interactive`): two turns,
  each terminal `Completed` with its exact reply text and `Clean` cleanup;
  the second turn resumed through the privately retained exact session id
  only (no `--continue`, no `--fork-session`); the session closed `Clean`.

Both gate prompts used no tools, so provider tool dispatch was not exercised.
No raw provider stream, account identifier, prompt, session id, tool body, or
private path is retained.

## Disposition

Live acceptance for exact-`1.65.0` structured runs, interactive sessions,
streaming events, and usage decoding now rests on this paid-model gate.
Research 330 stays the immutable exact-`1.54.0` paid-model record. Research
116 and 118 stay immutable and bound to `1.15.1`; authenticated credit-failure
evidence stays version-bound to `1.15.1`. The `command-code.npm` claim stays
one exact `QualifiedOnly` point at `1.65.0`.

## Next move

None under this lane. Chatterbox reconciles the live cells after closeout.
