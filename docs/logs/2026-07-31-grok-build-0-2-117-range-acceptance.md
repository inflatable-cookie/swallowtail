# 2026-07-31 Grok Build 0.2.117 Range Acceptance

## Changed

- added a separately gated, ignored installed Grok Build discovery selector
- classified installed exact `0.2.117 (f1c06093089f) [stable]` as qualified
  through the host-approved local process path
- updated architecture, route truth, feature truth, backlog history, and front
  doors to the maintained `0.2.114..=0.2.117` range
- closed roadmap g03.012 and cards 030-032

## Current State

Grok Build ACP now has two maintained behavior segments across four exact
stable releases. Exact `0.2.117` records newer ACP task-control behavior but
does not add a public task-control or subagent-control operation. Versions
above the maintained boundary remain visible as unverified newer.

Paused g03 cards 004-007 remain in bounds for Claude Agent and Gemini range
maintenance. Pi persisted load/resume and provider-session binding persistence
remain deferred. Registry publication remains closed until the operator
reopens it after sustained application proof.

## Validation

- installed selector: one passed in `0.04s`
- focused Grok: 24 passed in two seconds
- extracted package: 45 files compiled in three seconds
- provider route, lifecycle, feature, and activity matrices passed
- docs, Northstar, formatting, and diff checks passed
- no authentication, provider prompt, model request, or provider-state mutation

## Next

Run the g03 compatibility-maintenance checkpoint. Select another bounded lane
only if consumer evidence or material installed upstream drift justifies it.
