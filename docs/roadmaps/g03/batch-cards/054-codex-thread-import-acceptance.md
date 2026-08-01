# 054 Codex Thread Import Acceptance

Status: ready
Owner: Tom
Created: 2026-08-01
Milestone: `../020-codex-external-thread-discovery-and-import.md`
Depends on: card 053

## Goal

Close the first complete provider proof with cross-host conformance, public
guidance, and extracted-package evidence.

## Scope

1. Run common catalogue/import conformance against the Codex driver.
2. Cover local and remote-authoritative hosts, cursor pages, stale targets,
   cancellation, deadline, process loss, and joined cleanup.
3. Update Codex prepared examples and exact capability truth.
4. Verify ordinary new/load/resume and lifecycle behavior is unchanged.
5. Assemble and compile the extracted Codex package.

## Out Of Scope

- authenticated model prompt or consumer adoption
- another provider or universal session facade
- broad workspace, candidate, or publication gates

## Acceptance Criteria

- [ ] Codex passes the complete common profile under both host identities
- [ ] list/read failure cannot create a usable binding
- [ ] existing new/load/resume and management regressions pass
- [ ] docs explain browse, select, import, load, and resume separately
- [ ] extracted Codex package compiles
- [ ] card 055 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-codex swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy qa:docs`
- `git diff --check`
- no live or broad suite

## Auto-Continuation

Yes. Continue to card 055 after Codex acceptance.
