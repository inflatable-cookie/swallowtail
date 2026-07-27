# 2026-07-27 Kimi Local Server Lifecycle Driver

## Changed

- registered the separate Kimi local-server provider-session-management role
- added bounded authenticated HTTP transport for one approved loopback target
- added attached preparation with exact metadata, state-root identity, and
  opaque local bearer lease
- added owned foreground startup through exact `kimi web --no-open` arguments,
  readiness-origin verification, metadata preflight, and joined child cleanup
- added prepared inactive-session archive and restore operations
- kept delete unsupported before dispatch
- preserved provider rejection, post-dispatch uncertainty, deadline, and
  cancellation truth without retry

Provider login remains outside this route. The access profile accepts only the
provider-specific `kimi-code/local-server-bearer` mechanism with local-compute
metering. The adapter never reads the token file, emits the bearer, disables
authentication, selects a sibling port, or claims filesystem or descendant
process containment.

## Evidence

Deterministic fixtures use real bounded HTTP exchanges and host-owned opaque
references. They cover attached local and remote-authoritative execution,
owned remote-authoritative startup, exact safe arguments, authenticated
metadata, archive, restore, provider rejection, readiness mismatch cleanup,
and cancellation/deadline truth before and after dispatch.

The attached route never receives process authority. The owned route stops and
waits only for the child returned by its process service. Credential leases
release after joined HTTP work. Owned process cleanup joins before `close`
returns.

## Validation

- strict Kimi adapter Clippy passes
- the full Kimi adapter suite passes 39 deterministic tests; one live installed
  probe remains ignored
- workspace all-target checking passes
- formatting, docs, Northstar, and 22-route matrix checks pass
- `effigy doctor` remains at the pre-existing 32 findings: 23 warnings and
  9 errors

The public-API check returns the expected held-candidate diff. This additive
Kimi lifecycle surface remains part of card 059's deferred baseline
replacement; no candidate or publication state changed.

## Lane State

- card 062 is complete
- card 063 is ready
- cards 064-065 remain in bounds
- roadmap g02.020 remains active
- card 059 remains paused at its canonical-source gate

## Next

Execute card 063. Import an ACP-created Kimi session only after exact
cross-transport identity and target evidence produces a new local-server
management binding.
