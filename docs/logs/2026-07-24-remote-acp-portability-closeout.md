# Remote ACP Portability Closeout

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/135-remote-acp-portability-and-closeout.md`

## Outcome

The provider-neutral remote ACP client now passes one public portability
matrix across HTTP/2 SSE and WebSocket under local and remote-authoritative
execution-host identities. Each cell consumes an exact preflight plan,
host-approved endpoint grant, operation scope, and matching host services.

The matrix proves operation, connection, session, endpoint, audience,
configured-instance, transport, and host binding. It records only task, time,
and network host attempts. Credential and process authority remain absent.

## Transport Evidence

HTTP/SSE sends initialization through the public connection, receives the
server result, preserves scoped affinity, and closes with the explicit delete
path. WebSocket opens through the same public client and uses its distinct
explicit close path.

Raw card-133 corpora remain the primary oracle. A maintained
`agent-client-protocol-http = 2.0.0` server cross-check independently confirms
health success, missing-connection rejection, and content-type rejection.

Cancellation, deadline, disconnect, malformed protocol, explicit close, and
cleanup failure remain distinct. No path claims retry, reconnect, replay,
resumption, fallback, pooling, multiplexing, credential use, or
provider-session deletion.

## Redaction And Dependency Audit

Every stable transport error was checked against private endpoint, cookie,
session, header, and provider payload material. Cleanup failure exposes only
the stable `swallowtail.remote_acp.cleanup_failed` diagnostic.

ACP SDK, HTTP, WebSocket, cookie, and Tokio dependencies remain private to
`swallowtail-transport-acp-remote`. The provider-neutral core, runtime, and
testkit manifests contain none of them.

## Validation

- `cargo test -p swallowtail-transport-acp-remote` — 8 passed
- `cargo clippy -p swallowtail-transport-acp-remote --all-targets -- -D warnings`
- `cargo test -p swallowtail-testkit --test conformance_profiles` — 14 passed
- `effigy qa` — passed
- workspace inventory — 629 tests: 625 passed, four gated probes ignored
- `effigy doctor` — inherited 19 findings: 12 warnings, seven errors
- `git diff --check` — passed

## Remaining Risks

- remote ACP support remains `ExperimentalObserved`; the transport RFD and
  ecosystem are still maturing
- authentication and provider-specific identity remain unproven
- exact ACP SDK `2.0.0` is schema and server evidence, not a qualified
  interface-version range
- topology proof is deterministic and host-authoritative; no public live
  endpoint was exercised
- cancellation or disconnect does not establish provider-session deletion

## Continuation

Roadmap 045 and cards 133-135 are complete. Roadmap 046 and card 136 are ready
for the g01 generation-boundary and provider-coverage checkpoint. No provider
or generation rollover is preselected.
