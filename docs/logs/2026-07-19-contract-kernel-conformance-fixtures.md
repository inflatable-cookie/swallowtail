# Contract Kernel Conformance Fixtures

Date: 2026-07-19
Status: recorded

## Outcome

`swallowtail-testkit` now provides one canonical Contract 003 fixture and
public assertions for capability rejection, provider-reference opacity,
diagnostic redaction, extension isolation, and explicit preserve/reject
extension policies.

## Design

- plain records and functions; no test runner or trait framework
- assertions accept public `swallowtail-core` values
- external integration tests exercise only exported APIs
- failure messages name the violated Contract 003 rule
- the unknown extension fixture uses a namespaced opaque payload

## Boundary Evidence

- crate graph remains `swallowtail-testkit -> swallowtail-core`
- no external dependencies
- no consumer, provider, runtime, transport, or async vocabulary
- ten focused tests pass across the workspace
