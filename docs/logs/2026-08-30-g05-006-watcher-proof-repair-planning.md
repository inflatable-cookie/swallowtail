# 2026-08-30 g05.006 Watcher Proof Repair Planning

Status: complete
Owner: Tom
Milestone: g05.006
Card: 019

## Decision

The operator selected the quoted card 011 next move: repair the proof design
and in-progress activity path before considering another provider
authorization.

The repair is a new credential-free milestone rather than a reopening of the
consumed attempt. Card 011 and g05.003 remain honest evidence stops. Prototype
head `49f2692f` is salvage evidence only and must not be merged or cherry-picked
as a whole.

## Promoted Boundary

Contract 059 now requires bounded lossless watcher lifecycle delivery that is
independent of provider stdout cadence. The route uses the existing runtime
projector and may claim complete watcher lifecycle only when started,
in-progress, and terminal activity can all be delivered exactly once.

Card 019 requires a future-live proof recorder to establish the conjunction
the old selector missed: exact reserved tool discovery, watcher start, a Stop
completion-gate response while the watcher is active, native hook lifecycle,
same-session continuation, explicit wait or stop, joined zero state, and clean
provider terminal success. Proactive wait, direct gate use, and adapter-only
terminal rejection are negative fixtures.

## Authority

No provider turn, paid work, credential access, login, installation, update,
or ambient configuration mutation is authorized. The repair PR must keep the
watcher route claim, matrix, guide, and consumer route-feature projection
withheld.

## Next

Dispatch card 019 through its committed worker handoff. Review the returned PR
against both card oracles. Reassess live authorization only after that repair
lands.
