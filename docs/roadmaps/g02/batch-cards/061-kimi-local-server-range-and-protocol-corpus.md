# 061 Kimi Local Server Range And Protocol Corpus

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../020-kimi-code-local-server-route.md`

## Objective

Freeze the exact Kimi Code local-server boundary needed by a production Rust
driver before any server process or provider session is touched.

## Governing Refs

- Research 040
- Contracts 010, 014, 017, 029, 032-033, 037-038
- exact Kimi Code `0.28.1` and `0.29.0` tagged source

## Scope

1. Add a Kimi local-server driver descriptor and separate REST/WebSocket
   transport identity without registering a production role.
2. Add exact compatibility evidence for `0.28.1` and `0.29.0`, with permitted
   visible unverified-newer points above `0.29.0`.
3. Freeze bounded fixtures for:
   - health and exact metadata
   - authenticated OpenAPI and AsyncAPI identity
   - session create, lookup, archive, and restore
   - missing, busy, validation, unauthorized, and server-failure envelopes
   - WebSocket version `2` hello, subscription, cursor, event, ack,
     resynchronization, and close
4. Prove no selected delete route exists and the deprecated delete response
   alias means archive.
5. Add adapter-private protocol records and decoders with bounded payloads and
   safe diagnostics.
6. Record exact tagged source, behavior milestones, exclusions, and fixture
   provenance.

## Acceptance Criteria

- [x] ACP and local-server descriptors cannot substitute for each other
- [x] both qualified releases pass the same selected lifecycle corpus
- [x] exact executable and server-reported versions must agree
- [x] later stable releases retain exact `UnverifiedNewer` evidence
- [x] malformed, prerelease, older, and mismatched versions fail before effects
- [x] archive, restore, abort, connection close, and deletion remain distinct
- [x] token, raw session id, path, prompt, transcript, and response payload
      never enter stable diagnostics
- [x] no process, endpoint, credential, provider call, or session effect occurs

## Evidence

- `swallowtail.kimi.local-server` and
  `kimi-local-server-rest-ws-v2` are separate from ACP and advertise no driver
  role.
- Exact `0.28.1` and `0.29.0` claims share one REST/WebSocket v2 behavior
  revision; later stable versions remain permitted `UnverifiedNewer`.
- The adapter-private bounded corpus covers health, metadata, OpenAPI,
  AsyncAPI, session create and lookup, archive, restore, selected REST errors,
  WebSocket hello, subscribe, cursor, acknowledgement, durable and volatile
  events, resynchronization, abort, error, and close classification.
- Exact server metadata must corroborate the executable binding. Mismatch,
  older, prerelease, malformed, and oversized evidence fails with bounded
  diagnostics.
- The selected OpenAPI surface rejects a session DELETE method. Provenance
  records that the deprecated delete response schema is only an archive alias.
- No production role, transport I/O, process, credential, endpoint, provider
  request, or session effect was added.

## Validation

- focused Kimi adapter protocol and compatibility tests
- public API check
- docs and route checks
- `git diff --check`

Passed on 2026-07-27:

- `cargo clippy -p swallowtail-adapter-kimi --all-targets -- -D warnings`
- `cargo test -p swallowtail-adapter-kimi`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:routes`
- `git diff --check`

`effigy package:api` produced the expected held-candidate diff. It includes
this card's additive Kimi descriptor and claim plus the unbaselined lifecycle
APIs from cards 046-057. Card 059 owns candidate baseline replacement after
the canonical-source gate; card 061 does not rewrite release evidence from a
dirty working tree.

## Stop Conditions

- a qualified release changes selected REST or WebSocket semantics
- exact metadata cannot corroborate the executable release
- archive or restore semantics are weaker than Contract 038
- fixtures require live authentication or a running Kimi server

## Auto-Continuation

Yes. Continue to card 062 after the corpus and compatibility boundary pass.
