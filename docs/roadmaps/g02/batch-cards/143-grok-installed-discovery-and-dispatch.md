# 143 Grok Installed Discovery And Dispatch

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../043-grok-build-maintained-acp-route.md`

## Goal

Observe one host-approved Grok executable and bind only the exact qualified
behavior into immutable preflight.

## Governing Refs

- Research 070
- Contracts 008-010, 023, 029, 032-033, and 037
- roadmap g02.043
- card 142

## Scope

1. Add `swallowtail-adapter-grok`.
2. Parse exact `grok --no-auto-update --version` output and channel evidence.
3. Publish one exact `0.2.114` qualified claim.
4. Permit only explicit unverified-newer stable attempts above the ceiling.
5. Bind the discovery-owned driver, transport, access, configuration,
   retention, executable, and behavior revision into one prepared configured
   instance.
6. Prove local and remote-authoritative discovery, cancellation, deadline,
   redaction, completion, and joined cleanup.

Model route, operation isolation, session access, and provider-state policy
belong to card 144's operation preflight because installed discovery does not
observe or select an ACP session model.

## Acceptance Criteria

- [x] observation and support claim remain separate
- [x] `0.2.114` is qualified
- [x] `0.2.115` stable is visible unverified newer
- [x] alpha, prerelease, older, and malformed observations reject
- [x] no ambient executable search occurs
- [x] version drift fails before ACP or credential work
- [x] both host topologies join discovery work
- [x] prepared configured instance binds every discovery-owned route identity

## Evidence

- direct probe argv is exactly `--no-auto-update --version`
- exact `0.2.114` source revision is required
- later stable revisions remain observable without becoming guaranteed
- local and remote-authoritative fixtures cover completion and joined cleanup
- cancellation stops before spawn; deadline force-stops and joins
- 16 focused Grok adapter and protocol tests pass
- focused warnings-denied Grok clippy passes
- six focused adapter tests pass after prepared-instance promotion

## Validation

- focused Grok discovery and selection tests
- focused warnings-denied clippy
- docs QA after the implementation batch
- `git diff --check`

## Stop Conditions

- Stop if direct version probing creates provider state.
- Stop if channel evidence cannot distinguish stable from alpha.
- Do not implement the production session driver in this card.

## Auto-Continuation

Yes. Continue to card 144 after focused validation.
