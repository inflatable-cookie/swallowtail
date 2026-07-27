# OpenCode Session Deletion Closeout

Date: 2026-07-27
Card: `../roadmaps/g02/batch-cards/057-opencode-session-deletion-conformance-and-closeout.md`

## Proof

OpenCode deletion now runs through the production prepared facade at the
minimum of all eight exact deletion segments and latest-qualified `1.18.4`.
The same mapping passes under local and remote-authoritative execution-host
identities. Stable `1.18.5` remains executable only after explicit
unverified-newer acceptance.

The matrix proves:

- exact `ProviderDataDeleted` with `ProviderDefinedDescendants`
- denied endpoint and elapsed deadline before DELETE cause no delete attempt
- missing target and 401 are provider rejections, not already-absent success
- malformed 2xx, 5xx, disconnect, cancellation, and deadline after dispatch
  remain unconfirmed
- blocking transport work joins before resource and delegated credential
  release
- raw provider bodies remain outside diagnostics
- archive, restore, retry, fallback, and attached-server lifecycle authority
  remain absent

## Fixture Repair

The first full parallel runs exposed nondeterministic empty HTTP replies on
macOS. Accepted fixture sockets inherited the listener's nonblocking mode.
Their first read could return `EAGAIN`, close the stream, and surface a curl
transport failure.

The fixture now explicitly resets every accepted socket to blocking mode.
This changes deterministic test infrastructure only. Production curl behavior
is unchanged.

## Guidance

The 22-route guide now lists OpenCode inactive provider-session deletion and
both public entry points. The compiled example includes
`delete_inactive_session`.

## Validation

- full OpenCode adapter: 56 passed; one live installed probe skipped
- repeated focused deletion conformance: 5 passed
- isolated prior leak-marker test: passed
- examples, Rust check, format check, docs QA, Northstar QA, and diff check:
  passed

## Continuation

Roadmap 018 is complete. Cards 058-060 remain in bounds under roadmap 019.
Card 058 is the sole next task: publish the exact 22-route lifecycle matrix
without treating consumer-local thread state or driver cleanup as provider
management.
