# 067 Pi RPC Model Catalogue

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../021-model-catalogue-coverage.md`

## Objective

Expose Pi's configured models through the existing provider-neutral catalogue
role and prepared facade without opening a provider session or selecting a
model.

## Governing Refs

- Research 042
- Contracts 020, 028-029, 032-033, 037
- exact Pi `0.80.10` RPC documentation and source

## Scope

1. Freeze `get_available_models` request, response, malformed, rejected,
   overflow, deadline, disconnect, and cleanup fixtures.
2. Add the catalogue role and required host services to the Pi descriptor.
3. Add a route-free catalogue capability and preflight profile.
4. Start one offline, ephemeral, provider-suppressed, tool-free RPC process.
5. Project bounded provider id, model id, display name, reasoning, modalities,
   context window, and output limit observations.
6. Close and join process work before delegated credential release.
7. Add a typed `prepare_catalogue` operation to `PiPreparedIntegration`.
8. Preserve session behavior and exact `0.80.10` guarantee.

## Acceptance Criteria

- [x] catalogue requires no model route or working resource
- [x] no prompt, session persistence, tool, extension, update, retry, refresh,
      or model invocation occurs
- [x] provider and model ids remain separate
- [x] unknown safe metadata does not imply capability
- [x] every represented failure path joins owned work and releases delegated
      access
- [x] local and remote-authoritative deterministic fixtures pass
- [x] existing Pi session and version-discovery suites remain unchanged

## Evidence

- Pi now declares `ModelCatalogDriver` and exposes typed
  `PiPreparedIntegration::prepare_catalogue`.
- The tool-free, provider-suppressed ephemeral process sends only
  `get_available_models`.
- Bounded projection retains separate provider/model identity, display name,
  reasoning-support evidence, input modalities, context window, and output
  limit. Raw endpoint, API, cost, and provider payload fields stay private.
- Success, malformed, rejected, overflow, correlation drift, disconnect,
  deadline, and cleanup failure fixtures pass. Local and remote-authoritative
  prepared fixtures pass.
- The common catalogue request has no independent cancellation control.
  Research 042 and Contract 020 now record that shared runtime gap; this card
  does not claim cancellation by future drop.
- Focused core and Pi tests pass (75). Workspace all-target check, formatting,
  docs QA, route-matrix validation, and diff checks pass. The public-API hash
  gate detects this intentional Pi/core addition plus unrelated prior dirty-
  tree API changes; the retained release baseline remains untouched.

## Validation

- focused Pi protocol, prepared-facade, catalogue, lifecycle, and topology tests
- public API, formatting, docs, route matrix, and `git diff --check`

## Auto-Continuation

Yes. Continue to card 068 after focused validation passes.
