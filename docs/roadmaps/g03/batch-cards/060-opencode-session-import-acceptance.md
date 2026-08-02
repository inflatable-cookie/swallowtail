# 060 OpenCode Session Import Acceptance

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../022-opencode-external-session-discovery-and-import.md`
Depends on: card 059

## Goal

Close the attached-HTTP proof with topology conformance, public guidance, and
extracted-package evidence.

## Scope

1. Run common catalogue/import conformance under local and remote-authoritative
   host identities.
2. Cover pagination, stale targets, cancellation, deadlines, Basic-auth lease
   cleanup, and attached-server preservation.
3. Update OpenCode facade guidance and exact feature truth.
4. Verify existing session lifecycle and deletion behavior remain separate.
5. Assemble and compile the extracted OpenCode package.

## Out Of Scope

- authenticated prompt or live external server acceptance
- consumer adoption or another provider
- broad workspace, publication, or candidate gates

## Acceptance Criteria

- [x] OpenCode passes the complete profile under both host identities
- [x] attachment identity cannot drift between list and import
- [x] attached servers survive success, cancellation, and failure
- [x] deletion and management remain independent operations
- [x] extracted package compiles
- [x] card 061 becomes the sole ready and next task

## Evidence

- the provider-neutral catalogue/import contract is included in OpenCode's
  acceptance target
- local and remote-authoritative host identities produce bindings for only
  their exact host, endpoint, resource, route, and policy plan
- two-page traversal, stale target rejection, unverified-newer rejection,
  in-flight cancellation, deadline, and credential-cleanup failure fixtures
  pass
- cancellation and deadline abort the curl transfer, wait for blocking work,
  release the delegated Basic-auth and resource leases, and preserve the
  externally attached server
- session catalogue/import roles remain separate from interactive deletion and
  operation-private structured-run cleanup
- `effigy validate:focused swallowtail-adapter-opencode swallowtail-testkit`
  passed 172 tests
- `effigy package:verify-affected swallowtail-adapter-opencode` passed
- `effigy qa:docs` passed
- `git diff --check` passed

## Validation

- `effigy validate:focused swallowtail-adapter-opencode swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-opencode`
- `effigy qa:docs`
- `git diff --check`
- no live or broad suite

## Auto-Continuation

Yes. Continue to card 061 after OpenCode acceptance.
