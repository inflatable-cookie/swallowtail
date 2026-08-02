# OpenCode Session Import Acceptance

Date: 2026-08-02
Roadmap: g03.022
Card: 060

## Change

The attached OpenCode session catalogue/import route now passes its complete
acceptance profile. Local and remote-authoritative host fixtures both preserve
exact host, endpoint, resource, access, revision, route, and policy identity.
Only an unchanged inactive root produces a binding, which enters the existing
bounded load/replay and resume behavior.

Cancellation and deadline controls abort in-flight HTTP work but wait for the
blocking job before releasing leases. Delegated Basic-auth and working-resource
cleanup stays joined. The externally attached server remains usable after
success, cancellation, timeout, stale revalidation, and cleanup failure.

## Public Truth

The OpenCode guide and route matrix now expose provider-session catalogue and
import as separate prepared operations. They reject raw-id attachment and
automatic synchronization. Unverified-newer revisions remain visible but do
not inherit import support.

## Evidence

- `effigy validate:focused swallowtail-adapter-opencode swallowtail-testkit`
  passed 172 tests
- `effigy package:verify-affected swallowtail-adapter-opencode` passed
- `effigy qa:docs` passed
- `git diff --check` passed
- no live server, provider authentication mutation, prompt, or broad suite

## Next

Card 061 classifies every remaining harness route against the complete
catalogue, revalidation, replay, continuation, resource, activity, and exact
version profile.
