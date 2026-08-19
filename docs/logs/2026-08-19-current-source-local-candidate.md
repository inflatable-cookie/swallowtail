# Current Source Local Candidate

Date: 2026-08-19
Roadmap: `../roadmaps/g04/003-current-source-tag-before-readiness.md`
Card: `../roadmaps/g04/batch-cards/007-current-source-local-candidate.md`

## Result

The complete local `v0.3.3` source candidate is prepared. All 40 workspace
packages and coordinated internal requirements use `0.3.3`. Changelog,
release notes, 47-route inventory, 40-package semantic API baselines, metadata,
consumer front door, and source-only distribution copy agree.

OpenHands remains a package without a production route. Existing-package APIs
stay compatible. Immutable `v0.3.2` inventories remain 30 packages and 36
routes. No Spec 011 facade types ship in this candidate.

The first prepare attempt failed `security` on `h2` `0.4.15`
(RUSTSEC-2026-0258). `h2` `0.4.17` landed first; the second prepare passed
all 11 gates.

## Validation

- `effigy release prepare --yes --check-gates --version 0.3.3` — prepared
- all 11 configured gates passed on the prepared tree: api, docs, floor, fmt,
  lint, lint:no-features, metadata, qa, security, source, test
- isolated source consumer passed; `.release-prepared.json` is local state
  and is not a source identity
- Effigy `v0.11.0+local.17c109f.dirty`; prepared from
  `21b634ed1185c53375a8364b9b1b7b7e89e94d9b`

## Authority

This is a worker-branch candidate, not a release identity. No annotated tag,
GitHub Release, crates.io, workflow, consumer, or provider mutation ran.
Merge and tagging remain operator-authorized cards 008-009.

## Next Move

Validate the local `v0.3.3` candidate, then authorize card 008 canonical CI
at the exact SHA. Tagging remains a separate authorization.
