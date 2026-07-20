# Soundcheck Adoption Handoff

Status: prepared
Owner: Soundcheck downstream
Source milestone: 007 Soundcheck Structured-Run Readiness
Updated: 2026-07-19

## Outcome

Soundcheck can replace its direct Codex model-catalog and structured-run
mechanics with Swallowtail without moving product policy across the repository
boundary. This document is a downstream plan. It does not claim adoption has
happened.

## Replacement Boundary

Add one Soundcheck-owned `swallowtail_codex` integration module. It owns
Swallowtail registration, preflight records, local host wiring, and projection
back into the existing Tauri command shapes.

Keep these in their current Soundcheck modules:

- plugin evidence and screenshots as product resources
- prompts and JSON Schemas
- taxonomy decoding, validation, compatibility repair, ranking, and companion
  selection
- progress wording and Tauri event payloads
- proposal review, persistence, and application
- saved model/reasoning settings

Do not move `AssistantEvidence`, `AssistantTaxonomyProposal`, taxonomy records,
or application commands into Swallowtail.

## Current-Seam Mapping

| Current Soundcheck mechanism | Swallowtail replacement | Remaining owner |
| --- | --- | --- |
| `find_codex` and status projection | host bootstrap resolves the executable and saved-login environment into opaque configured-instance references | Soundcheck host integration |
| `discover_codex_models` | `CodexAppServerDriver::list_models` with a model-catalog preflight plan | Soundcheck maps catalog metadata into its settings DTO |
| scratch-directory creation | `WorkingResourceService::create_temporary` | local execution host |
| schema-file writes | `StructuredOutputDescriptor` plus `SchemaService` lease | Swallowtail host/driver transport; schema meaning stays Soundcheck-owned |
| optional screenshot path | `AttachmentDescriptor` plus `AttachmentService` lease | Soundcheck names the product resource; host authorizes it |
| model and reasoning argv | exact model route plus `OperationPolicy::reasoning_mode` | saved selection remains Soundcheck-owned |
| live/offline search argv | explicit external-network/search policy and matching capability plan | Soundcheck chooses policy per turn |
| JSONL reader and progress loop | ordered `RunHandle` event stream | Soundcheck maps generic events to existing product phases |
| timeout loop | host deadline plus `TerminalStatus::TimedOut` | Soundcheck chooses 15-minute or 3-minute deadline |
| cancellation flag and child kill | `RunHandle::cancellation` plus `TerminalStatus::Cancelled` | Soundcheck retains request-id command routing |
| final-message file | redacted `TerminalOutcome::output` | Soundcheck decodes and validates the returned JSON |
| child/scratch cleanup | `RunHandle::close` plus scoped host-lease cleanup | Swallowtail mechanism |

The initial status command may keep Soundcheck's executable/version/login
probe. Swallowtail does not yet claim a Codex discovery driver. Failure to
construct the configured instance or list models must still project as
unavailable or unauthenticated through the existing Soundcheck DTO.

## Per-Turn Mapping

- research: live search, selected reasoning, proposal schema, optional image,
  15-minute deadline
- repair: offline, selected reasoning, proposal schema, no image, 3-minute
  deadline
- function ranking: offline, selected reasoning, ranking schema, no image,
  3-minute deadline
- companion selection: live search, selected reasoning, companion schema, no
  image, 3-minute deadline

Each request gets an exact capability/preflight plan. Do not reuse a plan that
claims search, schema, image, or reasoning constraints absent from the request.

## Dependency Pin And Temporary Gate

First commit the verified Swallowtail source, then replace
`VERIFIED_SWALLOWTAIL_REV` below with that exact Git revision. Do not depend on
a branch name or an uncommitted local path.

```toml
[features]
swallowtail-codex = [
  "dep:futures-util",
  "dep:swallowtail-adapter-codex",
  "dep:swallowtail-core",
  "dep:swallowtail-host-local",
  "dep:swallowtail-runtime",
]

[dependencies]
futures-util = { version = "0.3", optional = true }
swallowtail-adapter-codex = { git = "https://github.com/inflatable-cookie/swallowtail", rev = "VERIFIED_SWALLOWTAIL_REV", optional = true }
swallowtail-core = { git = "https://github.com/inflatable-cookie/swallowtail", rev = "VERIFIED_SWALLOWTAIL_REV", optional = true }
swallowtail-host-local = { git = "https://github.com/inflatable-cookie/swallowtail", rev = "VERIFIED_SWALLOWTAIL_REV", optional = true }
swallowtail-runtime = { git = "https://github.com/inflatable-cookie/swallowtail", rev = "VERIFIED_SWALLOWTAIL_REV", optional = true }
```

Keep the feature off by default while both connector paths compile. The gate is
temporary migration scaffolding, not a supported permanent backend selector.

## Host Wiring

The integration module should:

1. resolve the existing Codex executable and minimal saved-login environment
   under Soundcheck host authority
2. build `LocalProcessHost` approvals for that executable, environment,
   temporary root, schema sources, screenshots, and operation resources
3. provide a Soundcheck/Tauri `ScopedTaskService`; Swallowtail remains
   executor-neutral
4. register the local host for process, time, working-resource, attachment, and
   schema services
5. register Soundcheck's host network-policy service only for live-search
   turns
6. create model-catalog and exact per-turn preflight plans from the selected
   instance, route, access state, capability constraints, and host-service set

Allowlist only the environment needed by Codex authentication and launch,
normally `HOME`, `PATH`, optional `CODEX_HOME`, and the platform temporary
directory. Add proxy or certificate variables only when explicitly configured.
Codex-spawned tools receive no inherited environment through the driver policy.

After starting a run, take its event stream and terminal future, then store the
boxed handle under the existing request id. This lets the separate cancel
command borrow `RunHandle::cancellation`; the worker removes and closes the
handle after the terminal outcome arrives.

## Downstream Sequence

1. Pin the committed Swallowtail revision and add the optional feature.
2. Add `swallowtail_codex` plus the Tauri task/network host adapters.
3. Route `available_codex_models` through the new catalog path under the
   feature; keep the public command and TypeScript DTO unchanged.
4. Route only `run_codex_turn` mechanics through the new structured-run path.
   Keep the surrounding product orchestration unchanged.
5. Route request-id cancellation to the stored Swallowtail run handle.
6. Run legacy and Swallowtail connector tests in separate builds, then enable
   the feature for manual acceptance.
7. Make the Swallowtail path the normal build only after the checklist passes.
8. Remove the legacy process/model-catalog implementation and the temporary
   feature gate in the same downstream lane. Do not retain two backends.

## Soundcheck-Owned Validation

- model list retains ids, display names, descriptions, provider default, and
  reasoning choices
- saved ChatGPT/Codex login works with the allowlisted environment
- a host-created non-Git temporary resource runs successfully
- research works with and without a screenshot
- research and companion turns use live search; repair and ranking stay offline
- each turn returns JSON matching its supplied schema transport
- progress remains ordered and the UI remains responsive
- cancel reaches a distinct cancelled terminal state and joins the child
- 15-minute and 3-minute deadlines reach a distinct timed-out state and join
  the child
- schema, screenshot, and temporary-resource leases leave no residue
- invalid output still enters Soundcheck's one-repair path
- ranking, companion selection, review, and apply behavior remain product-owned
- default logs and errors expose no prompt, output, schema, screenshot path,
  auth path, or raw provider payload
- existing Soundcheck unit, Tauri, Effigy, and manual tagging checks pass

## Rollback

Before legacy removal, disable `swallowtail-codex` and rebuild. No persistent
data changes are involved.

After legacy removal, revert the downstream transport-switch/removal commit and
restore the last verified dependency revision. Saved settings, taxonomy data,
screenshots, and proposals require no migration or rollback. If only a
Swallowtail regression is involved, pin the previous verified revision first.

## Removal Gate

Delete the legacy Codex launch, JSONL parsing, model-list RPC, timeout, and
child-cleanup code only after all Soundcheck-owned checks pass with the feature
enabled. Do not delete product validation, repair, ranking, progress projection,
review, or apply logic.
