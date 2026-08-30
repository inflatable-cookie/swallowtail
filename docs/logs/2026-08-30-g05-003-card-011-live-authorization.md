# 2026-08-30 g05.003 Card 011 Live Authorization

Status: ready
Owner: Tom
Card: 011
Contracts: 059, 060

## Decision

The operator authorized one bounded live Claude Code `2.1.251` turn using the
existing local provider state and requested a cheap model. The probe must pin
exact `claude-haiku-4-5`, use a 90-second operation deadline, and stop rather
than fall back to a pricier model.

The authorization covers one provider session only. A request reaching the
provider consumes the attempt whether it succeeds or fails. Identity, model,
or authentication rejection before a provider request does not authorize
setup, login, update, model fallback, or a substitute live probe. No second
provider attempt is authorized.

Research 241 keeps `--max-budget-usd` omitted: its API-catalog estimate is not
the selected local-subscription allowance. Research 226 also leaves exact
`2.1.251` outside the maximum-turns feature's probed set. Cost is bounded here
by the exact Haiku model, one session, and the 90-second deadline.

## Dispatch

Card 011 is ready for one manual worker/PR loop. The worker must complete all
credential-free implementation and deterministic checks before consuming the
live attempt. Success returns one reviewable PR. Failure after the provider
request returns one sanitized stop report with every watcher claim withheld.
The worker does not retry, merge, start another card, or promote the open
consumer route-feature projection lane.
