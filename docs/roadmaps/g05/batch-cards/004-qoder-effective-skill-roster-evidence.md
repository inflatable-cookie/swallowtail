# 004 Qoder Effective Skill Roster Evidence

Status: ready
Owner: Tom
Created: 2026-08-28
Milestone: `../002-effective-harness-skill-visibility-proof.md`
Depends on: Contract 058; Research 255
Research: 256

## Goal

Determine whether exact Qoder headless `1.1.25` can return the complete
effective selected-run skill roster before model inference.

## Scope

1. Freeze the exact source and protocol semantics of init `skills` and
   `plugins`.
2. Trace global, project-local, bundled, plugin, and unknown provenance into
   those collections, including deliberate installation workflows such as
   `npx skills` without running an ambient install.
3. Prove init timing relative to prompt acceptance, authentication, provider
   allocation, and model inference.
4. Prove completeness, bounds, invalid shapes, freshness, selected model/run
   identity, and empty behavior.
5. Return a deliver-now table or an honest empty set in Research 256.

## Output

Research and frozen Qoder-local evidence only. Do not change production code,
public API, route claims, shared contracts, or front-door planning.

## Acceptance Criteria

- [ ] exact field source and lifetime are frozen
- [ ] global and project membership is tested as first-class evidence
- [ ] prompt-free and no-model-inference timing is proved or rejected
- [ ] complete, empty, unavailable, and partial states are separated
- [ ] binding cards stay planned after an empty set

## Validation

- `effigy validate:focused swallowtail-adapter-qoder`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

- credentials, login, provider prompt, paid work, install/update, or ambient
  host mutation is required
- only worker or adapter file scanning, file presence, or package contents can
  establish membership instead of the exact Qoder roster
- exact source leaves completeness or selected-run visibility ambiguous

## Auto-Continuation

No. Return Research 256 and one reviewable PR.
