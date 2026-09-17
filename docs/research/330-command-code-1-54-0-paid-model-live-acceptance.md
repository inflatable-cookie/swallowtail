# 330 Command Code 1.54.0 Paid-Model Live Acceptance

Status: accepted live evidence; exact `1.54.0` point only
Owner: Swallowtail worker
Created: 2026-09-17
Task: g06.006; planning `38f784a5b657307018a11ff697419d37d99169e9`
Contracts: 029, 043

## Question

Does the exact `command-code.headless` `1.54.0` point complete the selected
live surface on the paid probe model `deepseek/deepseek-v4-flash`, settling
whether the g05.069 `malformed_stream` stop was a free-backend artefact?

## Answer

Yes. The one authorized gate ran once on `deepseek/deepseek-v4-flash` with
the stored local Command Code account, and both live probes passed: the
structured one-turn completion and the Contract 043 two-turn private exact-id
continuation. The `malformed_stream` failure does not reproduce on the paid
backend; it stays classified as a free-backend event-grammar artefact on
`meituan/LongCat-2.0:free`.

## Pre-gate boundary

Immediately before any provider operation, the host carried exact `1.54.0`
from `command-code --version`, `dist/index.mjs` SHA-256
`157feefa0140e78f060ef2c1f9c50d10de702196ea229dc73db6bbcc39a0bcbb`, and
`~/.commandcode/updates.json` with auto-update disabled. No
`command-code update`, `/update`, `--list-models`, install, login, host
update, or catalogue operation ran. The adapter passed `--no-auto-update`
throughout.

Official npm stable was re-probed at the boundary: `latest` is now `1.55.1`,
with `1.54.1`, `1.54.2`, `1.55.0`, and `1.55.1` published since the Research
317 freeze. That is an observation only; the gate stayed on exact `1.54.0`
and no claim moves.

## Observed gate

- Structured run (`probe:command-code-plan`): terminal `Completed` with the
  exact reply text, `Clean` cleanup, and every streamed AgentEvent decoded
  valid. No `malformed_stream`, no retry, no second attempt.
- Interactive continuation (`probe:command-code-interactive`): two turns,
  each terminal `Completed` with its exact reply text and `Clean` cleanup;
  the second turn resumed through the privately retained exact session id
  only (no `--continue`, no `--fork-session`); the session closed `Clean`.

The typed credit-failure path was excluded from this gate; it was already
observed on this exact point. Both gate prompts used no tools, so provider
tool dispatch was not exercised. No raw provider stream, account identifier,
prompt, session id, tool body, or private path is retained.

## Disposition

Live acceptance for exact-`1.54.0` structured runs, interactive sessions,
streaming events, and usage decoding now rests on this paid-model gate.
Research 116 and 118 stay immutable and bound to `1.15.1`; authenticated
credit-failure evidence stays version-bound to `1.15.1`. The
`command-code.npm` claim stays one exact `QualifiedOnly` point at `1.54.0`.

## Next move

None under this lane. Any version other than exact `1.54.0` needs its own
identity and live lane; Chatterbox reconciles the version-bound live cells
from this acceptance.
