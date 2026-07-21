# OpenCode HTTP Protocol Fixture

Date: 2026-07-20
Roadmap: g01 012
Card: 039

## Outcome

- captured installed OpenCode `1.14.48` health and OpenAPI evidence
- froze six attached-server operations in a new fixture-only adapter crate
- kept endpoint selection explicit and provider authentication delegated
- closed the plan-binding gap for opaque credential references, credential
  mechanism, endpoint audience, and separate harness provider ids
- fixed a deny-first read-only session mapping without config mutation
- excluded unstable permission-reply routes, server disposal, auth mutation,
  sharing, deletion, resume, write access, and dynamic tools
- made card 040 ready with a scoped blocking-I/O and joined-task shape

## Evidence

- maintained docs report default port `4096`; installed help reports `0`
- installed OpenAPI SHA-256 is recorded in the versioned fixture
- installed permission response routing differs from maintained server docs;
  the supported subset does not depend on either form
- success, provider failure, abort, disconnect, permission, question,
  keepalive, unknown-event, catalogue, and HTTP error fixtures are bounded
- six focused `swallowtail-adapter-opencode` tests pass
- full `effigy qa` passes with 137 tests

## Remaining Risks

- no production HTTP request or SSE reader exists yet
- only unauthenticated server transport with delegated provider auth is in scope
- provider/model catalogue responses may contain fields that must stay adapter
  private and redacted
- read timeout and abort must be proven to join the scoped blocking reader

## Next

Execute card 040 against the frozen `1.14.48` subset. Reject any behavior not
represented by the fixture before network work.
