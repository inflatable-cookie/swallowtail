# 013 Soundcheck Prepared Facade Migration

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../005-soundcheck-prepared-facade-adoption.md`

## Objective

Replace Soundcheck's manual Codex catalogue and structured-run preparation with
the completed facade.

## Governing Refs

- Swallowtail Contract 037 and completed roadmap g02.003
- completed Nucleus adoption evidence
- Swallowtail repository authority map
- Soundcheck `AGENTS.md` and active contracts/roadmap at execution time

## Scope

1. Enter Soundcheck under explicit consumer-repository authority.
2. Migrate app-server model catalogue to prepared catalogue setup.
3. Migrate Codex exec tagging/research operations to prepared structured-run
   setup.
4. Preserve executable selection, working directory, schema, screenshot,
   model, reasoning, search, timeout, cancellation, and progress inputs.
5. Preserve product evidence, validation, repair, ranking, review, and
   application downstream.
6. Use source rollback rather than a runtime compatibility shim.

## Acceptance Criteria

- [x] catalogue and exec use separate prepared paths
- [x] exact installed version is observed before preflight
- [x] schema, attachment, reasoning, search, deadline, and cancellation remain
      exact
- [x] existing DTOs and product workflow remain unchanged
- [x] deterministic parity passes before deletion
- [x] no direct or legacy provider transport returns

## Validation

- Soundcheck focused Codex, settings, and tagging tests
- deterministic catalogue and structured-run fixtures
- `effigy health` and relevant focused tests
- `cargo check -p soundcheck-app --all-targets --locked`
- `git diff --check`

## Evidence Required

- mapping of old helpers to prepared inputs
- deterministic parity results
- exact changed-file and ownership audit
- card 014 readiness assessment

## Exact Migration Inputs

| Current Soundcheck surface | Prepared replacement |
| --- | --- |
| `swallowtail_codex::host::local_host` and service helpers | retain Soundcheck path, screenshot, environment, network, and working-resource authority; return an opaque target from `approve_installed_executable` and compose exact services |
| `swallowtail_codex::preflight::catalog_plan` | app-server `prepare_codex`, then `prepare_catalogue` with the current request id and catalogue deadline |
| `swallowtail_codex::preflight::run_requirements` and `run_plan` | exec `prepare_codex`, then `prepare_structured_exec(CodexExecProfileInput)` |
| direct `CodexAppServerDriver` catalogue call | execute the prepared catalogue plan and request with `prepared.environment()` |
| direct `CodexExecDriver` structured run | execute the prepared exec plan and request with `prepared.environment()` |

Create separate app-server and exec prepared integrations. Preserve
`soundcheck.codex.catalog`, `soundcheck.codex.exec`, the model-route identity,
and the current OAuth access profile as explicit consumer inputs. Do not share
one configured instance across the two drivers.

Map `CodexRunRequest` exactly:

- `model` -> `CodexModelSelection`
- `prompt` -> `OperationContent`
- `working_directory` -> the approved `WorkingResourceRef`
- `reasoning_effort` -> `with_reasoning_mode`
- `allow_search` -> offline policies or host-approved network plus enabled
  search
- `timeout` -> `with_deadline`
- `schema` -> JSON Schema `StructuredOutputDescriptor`
- optional screenshot -> one image `AttachmentDescriptor`

Keep `CodexCancellation`, progress projection, terminal decoding, structured
output validation, repair, ranking, review, and proposal application unchanged.
Delete `src-tauri/src/swallowtail_codex/preflight.rs` only after deterministic
catalogue and structured-run parity. Narrow `host.rs` only after the composed
host retains all existing authority and joined cleanup.

Nucleus adoption is complete. Soundcheck execution proceeded under its
repository authority and the operator's prior authorization.

## Execution Checkpoint

Soundcheck now uses separate prepared app-server catalogue and structured-exec
paths. The migration preserves consumer request and result projection, replaces
manual host/task/preflight assembly, and deletes the superseded preflight
module.

A deterministic isolated consumer crate compiled the production Soundcheck
integration against the current Swallowtail source. Four tests passed and the
installed-Codex probe remained ignored. The fixture bound exact
`codex-cli 0.145.0` evidence into both plans and covered access provenance,
reasoning, host-approved search, schema, screenshot, deadline, cancellation,
progress, and media projection. Soundcheck Vitest passed 13 tests. Soundcheck
docs QA, Northstar QA, and `git diff --check` passed.

Soundcheck `effigy health`, 106 Rust tests, 13 Vitest tests, the locked app
check, normal QA, and `git diff --check` pass. A transient unrelated SQLite
dependency conflict surfaced while concurrent Soundcheck work was incomplete;
that work resolved the graph before closeout and the complete normal validation
set was rerun.

Card 014 is ready. No installed or authenticated Codex probe ran.

## Stop Conditions

- facade needs a Soundcheck-only public type
- migration changes product search, reasoning, validation, or application
  policy
- catalogue and exec lifecycles would be flattened
- consumer mutation authority is absent
- unrelated user changes cannot be preserved

## Auto-Continuation

Yes, only within Soundcheck and only after card 014 is explicitly ready from
passing parity.
