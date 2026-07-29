# 2026-07-28 Retained Execution Closeout And Resource Selection

## Scope

Complete roadmap g02.031 card 106: apply all final retained-execution
dispositions, prove the Kimi paths from package artifacts, and select the next
matrix family.

## Matrix Closeout

The 59 starting retained-execution and recovery `No` cells now have final
dispositions:

| Disposition | Cells |
| --- | ---: |
| `Not applicable` | 32 |
| retained exact `No` | 24 |
| realized `Yes` | 2 |
| route-dependent `Partial` | 1 |
| **Total** | **59** |

The two `Yes` cells are Kimi local-server active-turn reattachment and
provider-managed recovery. Installed Kimi recovery is `Partial`: the qualified
headless route supports it and ACP does not.

The full audited matrix falls from 366 to 334 `No` cells and now contains 119
`Not applicable` cells. The route gate preserves the historical 59-cell
classification and exact final disposition.

## Package Evidence

- all 23 package archives assemble from the dirty source snapshot
- the extracted package workspace passes locked check and test compilation
- packaged Kimi headless structured, local-server structured, corpus,
  lifecycle, binding-import, and interactive suites pass
- the intentional additive Kimi input methods are recorded in the public API
  baseline
- no package was published and no retained release candidate was replaced

## Validation

- `effigy qa`
- `effigy check:examples`
- `effigy package:check`
- `effigy qa:routes`
- `effigy qa:docs`
- `git diff --check`

Docs, Northstar, route, formatting, all-target, Clippy, workspace test,
doc-test, locked-example, metadata, API, generated-doc, MSRV/current-stable,
archive, extracted-workspace, and packaged-facade checks pass.

## Retained Risks

- Kimi ACP exposes no qualified managed-recovery agreement.
- Kimi local-server reattachment is maximum one, same turn, same cursor, and
  no replay. It is not session resume or retained background execution.
- Twenty-two selected-surface absences remain exact `No`.
- Amazon Bedrock asynchronous invocation and Anthropic Message Batches remain
  separate-route work with different operation, access, retention, and result
  lifecycles.
- No live authentication, provider request, paid operation, container, model
  server, publication, or consumer repository was used.

## Continuation

Roadmap g02.032 and cards 107-110 own the next 31-cell family:
working-resource selection and bounded workspace text writes. Card 107 is
ready. It must keep resource location, access, callback I/O, ambient harness
authority, and containment separate.
