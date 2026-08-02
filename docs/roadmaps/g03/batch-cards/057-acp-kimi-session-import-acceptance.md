# 057 ACP And Kimi Session Import Acceptance

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../021-acp-session-list-and-kimi-import.md`
Depends on: card 056

## Goal

Close the shared-protocol proof with topology conformance, package evidence,
and honest classification of the remaining ACP adapters.

## Scope

1. Run common catalogue/import conformance against Kimi ACP under local and
   remote-authoritative host identities.
2. Cover pagination, stale candidates, cancellation, deadline, process loss,
   and joined cleanup.
3. Keep Claude and Cursor unavailable unless exact list and load behavior is
   independently qualified.
4. Update ACP and Kimi prepared guidance and route truth.
5. Assemble and compile the extracted affected packages.

## Out Of Scope

- capability promotion for another ACP agent
- remote-provider claims beyond existing topology proof
- authenticated prompt, consumer adoption, or broad suite

## Acceptance Criteria

- [x] Kimi passes the complete profile under both host identities
- [x] protocol support alone does not widen another adapter
- [x] unsupported ACP agents fail before dispatch
- [x] existing Kimi load/resume regressions pass
- [x] affected packages compile independently
- [x] card 058 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-kimi swallowtail-testkit`
- `effigy package:verify-affected swallowtail-protocol-acp swallowtail-adapter-kimi`
- `effigy qa:docs`
- `git diff --check`
- no live or broad suite

## Auto-Continuation

Yes. Continue to card 058 after ACP/Kimi acceptance.

## Evidence

- the Kimi suite invokes common provider-session import conformance and repeats
  production list/import under local and remote-authoritative host identities
- pagination retains cursor scope and stable candidate ordinals across fresh
  operation attachments
- in-flight cancellation and deadline stop the child, join the protocol pump,
  and release resource and credential leases; disconnect and cleanup failure
  remain distinct stages
- a negotiated ACP agent without list capability fails before `session/list`;
  Claude and Cursor descriptors still expose no catalogue/import roles
- Kimi load/resume regressions remain inside 272 focused ACP, Kimi, and testkit
  tests; the protocol and Kimi packages compile independently
