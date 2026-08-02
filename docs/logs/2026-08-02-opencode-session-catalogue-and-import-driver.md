# OpenCode Session Catalogue And Import Driver

Date: 2026-08-02
Roadmap: g03.022
Card: 059

## Change

The attached OpenCode facade now exposes separate provider-session catalogue
and explicit import operations. Catalogue access is bounded to one resolved
filesystem resource and one approved endpoint. It lists only through
`GET /session`, reads activity through `GET /session/status`, and projects
opaque candidate references with bounded title and update-time evidence.

Import repeats exact health, lookup, directory, title, update-time, revision,
root, archive, and idle-status checks. Only an unchanged available root issues
an imported binding. That binding uses the existing OpenCode load, ordered
history replay, and resume path.

## Boundaries

- exact qualified `1.14.48..=1.18.10` revisions only
- no project scan, synchronization, raw provider payload, or provider prompt
- child, active, archived, incompatible, and provider-unavailable sessions do
  not become importable
- the server remains externally attached; Swallowtail releases only its
  resource and delegated credential leases

## Evidence

- local and remote-authoritative end-to-end catalogue/import/load fixtures pass
- two-page traversal preserves candidate identity and activity classification
- title drift fails exact import revalidation and issues no binding
- `effigy validate:focused swallowtail-adapter-opencode swallowtail-runtime`
  passed 195 tests
- `git diff --check` passed

## Next

Card 060 owns full common conformance, in-flight cancellation and deadline
proof, attached-server and Basic-auth cleanup evidence, public guidance, and
extracted-package acceptance.
