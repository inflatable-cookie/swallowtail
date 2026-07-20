# Soundcheck Consumer Adoption

Date: 2026-07-20
Status: completed

## Result

- Soundcheck now routes Codex model discovery and every structured research,
  repair, ranking, and companion turn through Swallowtail
- consumer prompts, schemas, settings, validation, review, and mutation remain
  outside Swallowtail
- consumer feedback added typed external-search and safe-reasoning progress,
  preserved agent activity and usage snapshots, and bounded model-catalogue
  deadlines with joined cleanup
- Soundcheck supplies only host-approved executable, environment, resource,
  attachment, schema, network, and time capabilities

## Evidence

- Swallowtail runtime and Codex adapter suites pass
- Soundcheck app nextest passes 40 tests with 2 ignored
- Soundcheck Vitest passes 13 tests
- authenticated local Codex catalogue discovery through Swallowtail passes
- Soundcheck compile, docs QA, and direct-transport audit pass
- the operator completed the requested native proposal, progress, and
  cancellation walkthrough and confirmed the route behaved correctly

## Packaging Gate

- create the first immutable Swallowtail revision
- replace Soundcheck's sibling path dependency with that revision

After the packaging gate, execute the prepared Nucleus adoption handoff.
