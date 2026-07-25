# Soundcheck Prepared Facade Adoption

Date: 2026-07-24
Roadmap: g02.005
Card: 013

## Outcome

Soundcheck now routes Codex model discovery through a prepared app-server
catalogue and structured operations through a separate prepared exec profile.
The consumer supplies its approved executable, environment, working resource,
OAuth assertion, model, reasoning, search, schema, screenshot, and deadline
inputs. Swallowtail owns exact version discovery, immutable preflight and
request construction, service requirements, process transport, cancellation,
and cleanup.

The migration removes Soundcheck's manual configured-instance, capability,
access, route, requirements, plan, structured-request, and thread-task setup.
Soundcheck's DTOs, prompts, progress, taxonomy validation, repair, ranking,
review, and mutation remain unchanged.

## Evidence

- deterministic isolated consumer compilation against the current Swallowtail
  source: pass
- production integration module tests: 4 passed, 1 installed-Codex test ignored
- exact fixture version: `codex-cli 0.145.0`
- separate catalogue and exec plans: pass
- access provenance, model route, reasoning, search, schema, screenshot,
  deadline, cancellation, progress, and media projection: pass
- Soundcheck Vitest: 13 passed
- Soundcheck docs QA and Northstar QA: pass
- Soundcheck `git diff --check`: pass
- direct/legacy transport audit: pass

## Validation

- `effigy health`: pass
- `effigy test cargo-nextest`: 106 passed, 2 skipped
- `effigy test vitest`: 13 passed
- `cargo check -p soundcheck-app --all-targets --locked`: pass
- `effigy qa`: pass
- Soundcheck and Swallowtail `git diff --check`: pass

An unrelated SQLite dependency conflict appeared while concurrent Soundcheck
work was incomplete. That work resolved its graph during the batch. The full
normal validation set above ran after resolution. This migration changed no
Soundcheck manifest or DAW-state source.

No installed or authenticated Codex probe ran.

## Disposition

Cards 013-014 and roadmap g02.005 are complete. Card 015 is ready for packaged
cross-consumer runtime proof.
