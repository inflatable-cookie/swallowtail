# Qwen And Ollama Interactive Implementation

Date: 2026-07-29
Roadmap: g02.034
Card: 117

## Changed

- added public prepared interactive profiles for `qwen.headless` and
  `ollama.attached`
- added Qwen one-child-per-turn execution with private exact `--resume`
  continuation, joined cleanup, mismatch rejection, and fail-closed state
- added Ollama bounded private transcript replay with one streaming chat
  attempt per turn and join-gated transactional commit
- admitted interactive direct inference through the existing exact
  attached-runtime evidence boundary
- updated route guidance and converted only the two selected interactive
  matrix cells

## Boundaries

Qwen exposes no provider session reference, public load, or public resume
binding. Failure, cancellation, timeout, mismatched provider identity, or
uncertain cleanup prevents another turn.

Ollama creates no provider session. Failed and partial attempts do not mutate
private history. Close clears history without stopping the server or unloading
the model.

Neither route gains realtime media, billed-cost evidence, retry, fallback,
archive, restore, delete, native provider close, sandbox, or write authority.

## Validation

- Qwen prepared facade: 5 passed
- Ollama prepared facade: 7 passed
- focused core, runtime, testkit, Qwen, and Ollama suites passed
- provider route and 22-solution feature matrices passed

## Next

Card 118 proves the public paths from extracted packages and closes the
61-cell residual programme inventory.
