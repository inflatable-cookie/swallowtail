# Codex Prepared Facade Conformance And Guidance

Date: 2026-07-24

## Outcome

Roadmap g02.003 and card 010 are complete. Card 011 is ready under Nucleus
repository authority.

The Codex prepared facade now has deterministic cross-topology conformance,
safe consumer-facing diagnostics, public usage guidance, and exact Nucleus and
Soundcheck migration inputs. No consumer repository changed.

## Deterministic Matrix

| Boundary | Prepared-facade evidence | Retained low-level evidence |
| --- | --- | --- |
| exact discovery | exec and app-server bind one host-approved target, exact version, access provenance, and configured instance | installed-discovery local/remote and independent claim suites |
| version behavior | deprecated `0.100.0`, current `0.145.0`, excluded `0.108.0`, and unverified-newer `0.146.0` remain distinct | legacy exec/app-server corpora and exact milestone dispatch |
| catalogue | prepared catalogue executes under local and remote-authoritative hosts | paging, deadline, disconnect, and joined cleanup |
| read-only session | plan-derived access/configuration agreement, reasoning, tools, and exact host | callbacks, interruption, cancellation, resume binding, and cleanup |
| bounded workspace | separate version-gated profile and exact working-resource service | provider workspace sandbox, denied network, provider requests, and topology |
| structured exec | exact model, reasoning, network/search, schema, image, deadline, retention, and posture | streaming, cancellation, timeout, materialization, policy mismatch, and cleanup |
| preparation failures | missing/spawn, output, exit, malformed, incompatible, drift, cancellation, timeout, and cleanup keep distinct safe stages | raw discovery outcomes remain distinct |
| redaction | prepared values and projected failures hide operation payloads; target and environment remain opaque | parser, process, callback, and runtime diagnostics remain redacted |

The facade calls the existing low-level drivers. It adds no second protocol or
lifecycle implementation.

## Public Guidance

`docs/guides/codex-prepared-integration.md` records:

- exact target approval and preparation
- the four named profiles and their explicit consumer choices
- unsupported app-server session deadlines, tool-bearing resume, and exec
  tools
- version-range and unverified-newer behavior
- safe preparation stages
- the unchanged low-level escape hatch

The compile-tested example constructs preparation and read-only/structured
profile inputs without manual configured-instance or requirements assembly.

## Consumer Inputs

Card 011 maps Nucleus host, discovery, preflight, catalogue, chat, bounded task,
and smoke surfaces onto the prepared app-server facade. It retains Nucleus turn
deadlines, tools, callbacks, linkage, receipts, persistence, and UI.

Card 013 maps Soundcheck host, catalogue, preflight, and structured-run
surfaces onto separate prepared app-server and exec integrations. It retains
schemas, screenshots, reasoning, search, cancellation, progress, validation,
repair, ranking, review, and application.

Card 013 remains planned behind Nucleus adoption. Its input mapping is ready;
the ordering gate is unchanged.

## Validation

- 89 of 89 Codex tests pass
- 16 prepared-facade tests pass
- 682 workspace tests are inventoried: 678 pass and four separately gated live
  probes remain ignored
- workspace check and warnings-denied lint pass
- all 23 public-package API baselines pass
- package documentation passes
- declared MSRV and current-stable checks pass
- docs and Northstar QA pass
- `git diff --check` passes
- Effigy doctor remains at the inherited 19 findings: 12 warnings and 7 errors

## Remaining Risks

- live installed and authenticated Codex probes remain separately gated
- unverified-newer Codex versions execute without a support guarantee
- app-server session-open deadlines remain unsupported; turn deadlines remain
  available
- Codex cannot safely redeclare dynamic tools on resume
- consumer adoption must prove parity inside each consumer repository before
  deleting its old helpers

## Continuation

Card 011 is the sole next task. Enter Nucleus under its own instructions,
replace manual preparation without changing product policy, and stop before
Soundcheck until the Nucleus parity gate is complete.

Publication, registry, tag, push, release, and workflow mutation remain out of
scope.
