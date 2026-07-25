# Nucleus Prepared Facade Adoption

Date: 2026-07-24

## Outcome

Roadmap g02.004 and cards 011-012 are complete. Soundcheck card 013 is ready.

Nucleus now routes Codex catalogue, read-only Agent Chat, bounded task
execution, and read-only smoke setup through Swallowtail's prepared app-server
facade. The five reported setup failures are no longer assembled by consumer
code: exact executable version, compatibility, ambient configuration posture,
session access, and immutable plan/request agreement come from one prepared
integration.

## Ownership

Nucleus retains:

- executable-path and saved-login environment approval
- stable instance and model-route identities
- caller-asserted subscription access evidence
- working-resource selection
- prompts, reasoning, and tool declarations
- turns, callbacks, task linkage, outcomes, receipts, persistence, and UI

Swallowtail owns:

- installed-version discovery and compatibility classification
- configured-instance and requirements assembly
- catalogue, read-only, and bounded-workspace preparation
- matching plan/request construction
- process, protocol, and joined lifecycle mechanics

The retained executable resolver is not duplicate provider setup. Selecting a
host-approved local binary remains consumer execution-host authority.

## Removal

Nucleus removed its manual discovery and preflight modules, copied access
policies, custom thread task service, and manual host-service composition. The
new production preparation translation is 83 lines and adds no dependency.

No Nucleus product type entered Swallowtail. No runtime compatibility shim was
added.

## Validation

- Nucleus workspace check passes
- Nucleus health passes; the original doctor compile failure is gone
- 18 of 18 focused adapter tests pass; two authenticated tests remain gated
- 1,991 of 1,991 server tests pass; 12 gated tests remain skipped
- deterministic preparation proves exact version `0.145.0`, access provenance,
  catalogue, read-only, and bounded-workspace policy
- Nucleus docs and Northstar QA pass
- `git diff --check` passes
- authenticated installed Codex probes were not run

The server suite retains 22 pre-existing test-only unused-import warnings.
Nucleus Effigy doctor now reports 14 passing checks, one generated-source
warning, and one known god-file error check.

## Rollback

Rollback is one consumer-owned source change restoring the prior adapter,
host, smoke, and task files plus the removed discovery/preflight helpers, then
removing the new preparation module. Old and prepared paths must not coexist.

## Continuation

Card 013 is the sole next task. Enter Soundcheck, keep app-server catalogue and
exec structured runs separate, and preserve all product request and result
semantics.
