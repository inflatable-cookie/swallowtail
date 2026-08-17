# DeepSeek Harness JSON-RPC Planning

Date: 2026-08-17
Roadmap: g03.069

## Outcome

Promoted DeepSeek Harness JSON-RPC into Swallowtail planning without collapsing
it onto Open Platform continuation:

- Research 124
- Spec 008
- Milestone g03.069 with ready card 218 and planned cards 219-221

First subset: exact runtime-bin `0.1.0rc6`, owned-process NDJSON JSON-RPC,
structured run, content-free reasoning progress, harness-owned tool activity,
usage, process-kill cancel. ACP, Web `/api`, headless CLI, and
`deepseek.continuation` stay outside.

Live proof may use host-local Ollama; that does not qualify
`deepseek-official`.

## Validation

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g03`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy qa:northstar:docs-front-door`

## Next

Card 218 completed on worker commit `e5aa7b9f`. Card 219 is the next ready
card; cards 220-221 continue only through their auto-continuation after the
predecessor lands.

## Card 218 closeout

- froze exact runtime-bin `0.1.0rc6`, package identities, Apple Silicon
  executable and spawn-helper digests, and the non-axis `serverInfo.version`
- added redacted JSON-RPC fixtures for text success, tool success, tool error,
  missing credential, and namespaced unknown events
- recorded the qualified-only compatibility and protocol-facade revisions,
  live-versus-durable cardinality split, stream bounds, idle ownership, and
  force-stop cancellation boundary
- added a package-independent validator covering framing, lifecycle,
  correlation, redaction, usage, terminal ordering, and safe rejection cases

Validation: `python3 scripts/check-deepseek-harness-corpus.py -v` passed 12
tests; `effigy qa:northstar` passed. The sole roadmap pointer now names card
219; cards 220-221 remain planned.

## Card 219 closeout

Card 219 completed on worker commit `9bbf4f61`:

- added the separately selectable `swallowtail-adapter-deepseek-harness`
  workspace package and exact rc6 runtime-bin claim
- added target-bound installed discovery with exact executable basename and
  opaque `0.1.0rc6` compatibility parsing
- added bounded JSON-RPC initialize/prompt/shutdown handling, session-event
  decoding, content-free reasoning progress, text, harness-owned tool
  activity, usage, idle folding, safe failure, and namespaced unknowns
- joined host deadline, process-kill cancellation, terminal delivery, and
  cleanup without claiming a native JSON-RPC cancel method

Validation: `effigy validate:focused
swallowtail-adapter-deepseek-harness` passed 8 tests; `cargo fmt --all
-- --check` passed; warnings-denied Clippy passed for the package and all
targets. Card 220 is now the sole roadmap pointer; card 221 remains planned.

## Card 220 closeout

Card 220 completed on worker commit `1c053dbb`:

- exposed `prepare_deepseek_harness_jsonrpc` with exact target, opaque
  host-approved Cordis configuration, access evidence, and bounded discovery
- added immutable prepared integration and operation evidence
- added explicit provider/model route selection and `prepare_run` →
  `start_run`
- bound ambient Cordis configuration, host-owned isolation, read-only working
  resource, Task/Process/Time services, and no provider retention/recovery

Validation: `effigy validate:focused
swallowtail-adapter-deepseek-harness` passed 10 tests; `cargo fmt --all
-- --check` passed; warnings-denied Clippy passed for the package and all
targets. Card 221 is now the sole roadmap pointer.

## Card 221 implementation closeout

Card 221's package and deterministic acceptance slice is implemented on the
worker branch:

- added the public prepared-integration example and exact operator guide
- promoted `deepseek-harness.jsonrpc` through the feature, activity, lifecycle,
  architecture, package, and release-baseline inventories
- kept immutable `v0.3.2` at 30 packages and 36 routes while current source
  carries 31 packages and 37 routes
- added separate installed and prompt smoke selectors with exact executable,
  Cordis, cwd, provider, and model inputs; live work remains host-gated
- refreshed the reviewed unreleased Claude API override exposed by the
  post-tag response-only compatibility change and added the DeepSeek Harness
  API baseline without touching tagged inventories

Validation passed:

- `effigy validate:focused swallowtail-adapter-deepseek-harness` — 10 tests,
  warnings-denied Clippy
- `effigy package:verify-affected swallowtail-adapter-deepseek-harness`
- `effigy package:api` — 30 immutable v0.3.2 packages plus 2 reviewed
  unreleased API surfaces
- `effigy package:metadata`, `effigy package:docs`, `effigy check:examples`
- `effigy qa:guides`, `effigy qa:routes`, `effigy qa:docs`, and
  `effigy qa:consumer-docs`

No exact `dsh-jsonrpc-agent-pkg-macos-arm64` executable or Cordis
configuration is available on this host, so the ignored installed/live tests
were not claimed as passed. The next operator action is to supply those
host-approved inputs and run the two separate selectors. ACP, Web `/api`,
session-id continuity, DeepSeek-official qualification, version/tag/release,
and registry work remain out of scope.
