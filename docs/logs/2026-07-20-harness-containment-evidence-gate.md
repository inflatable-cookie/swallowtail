# 2026-07-20 Harness Containment Evidence Gate

## Changed

- promoted Research 007 from current Linux, macOS, and Windows platform
  evidence
- tightened Contract 017's mechanism qualification and fail-before-start rule
- rejected partial Landlock, deprecated `sandbox-exec`, experimental Windows
  APIs, and unproved remote claims as complete filesystem containment
- paused roadmap 018 and marked card 057 blocked
- kept cards 058-059 planned but blocked; no Kimi production code was added
- split the card 056 Kimi fixture assertions so they add no doctor finding

## Current State

The Kimi protocol corpus remains valid. Production mapping cannot proceed on
the current local macOS host. A supported macOS route needs deployment-owned
App Sandbox packaging, signing, entitlements, and helper structure. Landlock
alone does not cover every required filesystem operation. The current Windows
process-sandbox surface is experimental.

No portable runtime containment record is justified until one concrete host
mechanism passes Contract 017.

## Validation Boundary

This batch validates evidence, contracts, roadmap state, and the unchanged ACP
fixtures. It cannot validate a containment implementation because the card's
documented stop condition fired before code.

## Risks

- partial filesystem restriction can be misreported as full containment
- runtime or credential read allowlists can expose unrelated host data
- inherited descriptors and alternate syscalls can bypass path-open controls
- local fixtures cannot prove remote containment
- selecting a deployment sandbox or deferring Kimi changes product topology
  and needs operator authority

## Next

Operator choice: authorize a deployment-owned contained host and its first
platform, or defer Kimi production work and compile host-owned ephemeral
llama.cpp as the next coverage lane.
