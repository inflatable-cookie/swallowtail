# 2026-08-30 g05.007 Card 020 Live Authorization

Status: superseded for platform envelope; turn later consumed on Linux
Owner: Tom
Milestone: g05.007
Card: 020
Contracts: 044, 059, 060

## Supersession

The operator selected the Linux envelope on 2026-08-31 after the first worker
stopped before contact. This log remains the historical initial authorization;
its `darwin-arm64` digest and unchanged-probe boundary are superseded by
[the Linux envelope](./2026-08-31-g05-007-card-020-linux-envelope.md). No
provider request occurred under this Darwin envelope. The one-turn budget was
later consumed on Linux; see
[the card 020 live stop](./2026-08-31-g05-007-card-020-linux-live-stop.md).

## Decision

The operator authorized exactly one fresh Claude Code `2.1.251` watcher
acceptance turn using exact model `claude-haiku-4-5`. No model fallback and no
automatic, failure-driven, or review-driven rerun is authorized.

The worker must use the unchanged repaired selector from card 019. Before any
provider request it verifies exact installed version, native SHA-256
`625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5`,
absence of `ANTHROPIC_API_KEY`, clean source state, and credential-free
validation. It uses existing local subscription state and a `90`-second
operation deadline. It may not inspect credentials, login, install or update
Claude, mutate ambient settings, use an alias, or substitute another prompt or
probe.

Any request reaching Claude consumes the turn. Identity or validation failure
before contact consumes nothing but stops this handoff. A failed, incomplete,
or ambiguous live result consumes the authorization and returns to an evidence
stop with all watcher claims withheld.

## Proof Boundary

Success requires the repaired ordered conjunction: reserved tool discovery,
watcher start, active completion-gate response attributed to native Stop,
same-session continuation, explicit wait or stop, joined zero state, complete
HostWatcher lifecycle activity, clean provider terminal, and joined bridge and
process cleanup. Proactive wait, direct gate use, terminal-only rejection, a
success string, or registry presence alone is insufficient.

Only bounded event kinds, counts, safe turn/session correlation, revisions, and
ordering may be retained. Raw provider or HTTP bodies, prompt text, endpoint,
bearer, credential material, paths, commands, arguments, environment, PID,
watcher output, and source artifacts remain private.

## Dispatch

This initial dispatch stopped before contact and is no longer current. The
Linux envelope owns the next serial manual worker/PR loop. Card 011 and g05.003
remain immutable evidence stops. The open consumer route-feature projection
note remains outside this lane and unpromoted.
