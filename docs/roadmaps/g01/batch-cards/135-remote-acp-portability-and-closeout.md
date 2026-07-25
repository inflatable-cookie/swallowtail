# 135 Remote ACP Portability And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../045-remote-acp-transport-proof.md`

## Objective

Prove the shared remote ACP transport across both authoritative host topologies
and close the first provider-neutral transport lane.

## Governing Refs

- Research 029
- Contracts 004, 005, 009, 011, 014, 015, 019, 029, and 035
- roadmap g01.045
- cards 133-134

## Scope

1. Run the thirteenth conformance profile against HTTP/SSE and WebSocket
   fixtures under local and remote-authoritative execution-host identities.
2. Prove operation, connection, session, endpoint, audience, configured
   instance, and host binding.
3. Prove callback exchange, cancellation, deadline, disconnect, malformed
   protocol, explicit close, and cleanup failure behavior.
4. Prove no retry, reconnect, replay, resumption, fallback, pooling,
   multiplexing, or live credential use.
5. Cross-check a maintained-SDK test server while retaining raw loopback
   fixtures as the primary oracle.
6. Audit redaction and dependency direction.
7. Promote realized architecture and currentness surfaces.
8. Run full repository QA and record the generation-boundary checkpoint.

## Boundaries

- no provider-specific remote ACP adapter
- no public endpoint or live authentication
- no stable remote ACP support claim
- no provider, agent, model, credential, endpoint, or topology fallback
- no Nucleus or Soundcheck edit
- no automatic g02 rollover

## Acceptance Criteria

- [x] both transports pass the same public profile under both host identities
- [x] transport-specific lifecycle and affinity differences remain tested
- [x] cancellation and disconnect never imply provider-session deletion
- [x] explicit close joins every owned task and connection
- [x] stable diagnostics contain no sensitive or raw protocol material
- [x] full QA passes or failures are recorded honestly
- [x] roadmap 045 and all currentness surfaces close coherently
- [x] one provider-selection or generation checkpoint remains next

## Validation

- focused remote ACP profile and transport tests
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Evidence Required

- profile matrix across transport and host topology
- focused and full test counts
- dependency audit
- redaction and cleanup results
- explicit remaining auth, protocol-maturity, capability, and topology risks

## Stop Conditions

- any topology requires client-owned execution-host authority
- raw and maintained-SDK fixtures disagree materially
- full QA exposes a contract-level defect
- the next provider choice would establish unsupported product policy

## Auto-Continuation

No. Return to a provider-selection and g01 generation-boundary checkpoint.

## Outcome

The public client passes the same remote ACP preflight and lifecycle matrix for
HTTP/SSE and WebSocket under local and remote-authoritative host identities.
The matrix binds the exact operation, connection, session, endpoint, audience,
configured instance, transport, and execution host. Both topologies use only
task, time, and network services.

Raw loopback corpora remain the primary transport oracle. The maintained
`agent-client-protocol-http = 2.0.0` server independently agrees on health,
missing-connection, and content-type rejection behavior. Lifecycle and
redaction tests retain distinct cancellation, deadline, disconnect, malformed
protocol, explicit close, and cleanup-failure truth without recovery or
provider-session deletion claims.

The production dependency audit keeps ACP SDK, HTTP, WebSocket, cookie, and
Tokio dependencies private to `swallowtail-transport-acp-remote`. Core,
runtime, and testkit remain free of those dependencies.

Focused transport tests pass 8/8 and the thirteenth conformance pack passes
14/14. Full repository QA inventories 629 tests: 625 pass and four separately
gated installed/live probes remain ignored. Doctor remains at the inherited
19 oversized-file findings: 12 warnings and seven errors. `git diff --check`
passes.

Roadmap 045 is complete. Roadmap 046 and card 136 are ready for the required
g01 generation-boundary and provider-coverage checkpoint. They do not
preselect a provider or roll to g02.
