# Post-Kimi Range And Remote ACP Selection

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/132-post-kimi-range-provider-coverage-evidence.md`

## Outcome

Remote ACP is selected as the next shared proof. The first lane is an
unauthenticated, provider-neutral transport over exact host-approved
HTTP/SSE or WebSocket endpoints.

Research 029 corrects the prior readiness assessment: the Active remote
transport RFD now has a maintained Rust HTTP transport crate at `2.0.0`.
Contract 035 fixes the connection, affinity, lifecycle, version, maturity, and
cleanup boundary. Roadmap 045 and cards 133-135 compile the implementation
runway.

## Why

- all twelve prior transport and lifecycle profiles have production proofs
- remote ACP is the remaining shared-protocol transport gap
- the Rust-native implementation avoids a foreign runtime or heavy container
- Grok Build repeats existing local CLI and ACP shapes
- Claude and Cursor SDKs remain foreign-runtime integrations
- interactive ACP authentication is not ready for a common contract
- more version-range work deepens represented boundaries rather than adding
  the missing one

## Contract Posture

- remote ACP is a transport, not a generic integration family
- support authority is `ExperimentalObserved` and requires opt-in
- authentication is excluded from the first proof
- HTTP/SSE and WebSocket selection is exact; no probing or fallback
- connection ids, session ids, cookies, credentials, and operation ids remain
  separate
- the Active RFD requires connection-scoped cookies for both Streamable HTTP
  and WebSocket; the WebSocket proof must retain upgrade cookies
- no reconnect, retry, replay, resumption, pooling, or multiplexing
- the SDK pin is separate from ACP wire and runtime interface versions
- explicit graceful close and joined cleanup are mandatory

## Generation Posture

g01 remains active at 45 roadmaps. Roadmap 045 fits the documented range and
owns three ready cards. Its closeout must reassess the generation boundary
before another material lane is compiled.

## Continuation

- card 133: shared records, independent corpus, and thirteenth profile
- card 134: reusable remote ACP client transport
- card 135: cross-topology portability, full QA, and closeout

## Validation

- `effigy qa:docs` passed
- `git diff --check` passed
- `effigy doctor` remains at the inherited 19 oversized-file findings:
  12 warnings and seven errors

## Sources

- [Research 029](../research/029-post-kimi-range-and-remote-acp-selection.md)
- [ACP RFD lifecycle](https://agentclientprotocol.com/rfds/updates)
- [Remote transport RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport)
- [ACP Rust HTTP crate](https://docs.rs/crate/agent-client-protocol-http/2.0.0)
