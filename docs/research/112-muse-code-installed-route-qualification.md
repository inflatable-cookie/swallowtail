# 112 Muse Code Installed Route Qualification

Status: promoted
Owner: Tom
Date: 2026-08-06

## Question

Does Meta Muse Code expose a stable enough machine boundary to justify a
dedicated Swallowtail route, rather than being treated only as another new
interactive harness?

## Sources

- Meta's 2026-08-05 Muse Code and Muse Spark 1.2 announcement
- Meta's Muse Spark 1.1 and Meta Model API announcement
- the provider-installed Muse Code launcher and signed Apple Silicon payload
- exact `--version`, root help, subcommand help, deterministic echo execution,
  and one operator-authorized authenticated Meta execution

The authenticated probe used one low-effort model step, no web tools, no shell,
no writes, no retained session log, and an isolated temporary workspace. It did
not exercise approvals, workspace mutation, subagent control, or recovery.

## Installed Artifact

The installed launcher reports:

```text
Muse Code 0.1.0 (0.1.0-R708.1)
```

The launcher at `~/.local/bin/muse` is a 33,118-byte Bash program. Its default
stable channel may update the launcher and active payload in the background.
The exact payload is a separate 101,945,920-byte executable named
`muse-bin-0.1.0-R708.1`.

Observed artifact evidence:

- payload SHA-256:
  `4290bfafa5bbb81a6fd493aaea12f848c789b1d22edfa0c4b849151deba3e70c`
- launcher SHA-256:
  `21c66e550a71cac2e4af081cc33d10bec81993d0043ec492761fc449e6c440f6`
- Apple signature identifier: `muse-arm64`
- signing team: Meta Platforms, Inc. `V9WTTPBFK9`
- release-manifest version: `0.1.0-R708.1`

Direct invocation of the versioned payload returns the same version and runs
the deterministic echo provider without the launcher's update behavior.

## Machine Surface

`muse exec --json` is an explicit non-interactive JSONL surface. It supports:

- exact provider and model selection
- reasoning efforts `none`, `minimal`, `low`, `medium`, `high`, `xhigh`, and
  `ultra`
- a caller-selected workspace and optional Git worktree
- bounded model steps and tool-result bytes
- approval, sandbox, write, shell, and web-tool controls
- optional local event-log suppression
- a deterministic local `echo` provider

The broader CLI separately exposes session resume, transcript export, trace
inspection, skills, sandbox setup, cross-session messages, and Meta login.
Those surfaces are evidence for later qualification, not capabilities of the
first route.

## JSONL Evidence

Both echo and authenticated Meta runs emitted event envelope schema version 1.
The envelope carries:

- session, run, and task stream identities
- a strictly increasing sequence
- durable, ephemeral, and reconciliation records
- command acceptance and run linkage
- exact configured provider and model evidence
- run output deltas
- task proposal, acceptance, scheduling, side-effect intent, status, start,
  completion, and failure evidence
- one explicit run terminal record

The authenticated probe configured provider `meta`, profile `tbh`, and model
`muse-spark-1.2`; it completed with the requested low reasoning effort. The
stream exposed provider attempt and response metadata but no portable usage
record. Random session, command, request, and response identities remain
private capture data and are not copied into the repository.

The event stream is an event-sourced runtime protocol, not an OpenAI-compatible
chat stream. The Meta Model API remains a separate possible direct-provider
route even though Muse Code uses it internally.

## Route Decision

Muse Code merits a dedicated installed-harness package and route:

- package: `swallowtail-adapter-muse`
- family: `muse-code`
- route: `muse-code.headless`
- driver: `swallowtail.muse-code.headless`
- transport: Muse event JSONL over an owned process
- version axis: `muse-code.release`
- first qualified point: exact opaque `0.1.0-R708.1`

The route must bind the approved versioned payload, not silently execute an
auto-updating launcher. A later stable release requires a new exact artifact
and behavior assessment. Opaque qualification does not admit unverified newer
builds.

## First Production Subset

The first route is one read-only structured run:

- locally authenticated Meta account access; Swallowtail receives no token
- exact `meta` / `muse-spark-1.2` selection
- explicit reasoning effort
- `muse exec --json`
- no retained session log
- no shell, writes, or web tools
- bounded model steps, tool-result bytes, JSONL line size, event count, and
  event delivery
- output, terminal, safe failure, and identity/lifecycle-only task activity
- force-stop cancellation, host deadline, and joined process cleanup

The exact selected command retains Muse's provider-enforced sandbox while
configuration remains ambient. The qualified flags disable writes, shell, web
tools, foreign personal context, and session logging. Do not claim a model
catalogue, interactive continuation, plan mode, approvals, questions,
task-list snapshots, subagent topology or control, usage, recovery, worktree
ownership, or session management from help text alone.

## Deferred Surfaces

Muse's append-only session log and offline export make exact continuation and
recovery unusually promising. They also contain messages, tool results,
approvals, question outcomes, model identities, encrypted reasoning, and
fork/subagent lineage. Any later route must use the documented export schema or
live JSONL evidence without exposing raw private records.

`resume` currently advertises a terminal session picker, while `exec` accepts a
session UUID. Their exact relationship and headless multi-turn semantics remain
unqualified. Cross-session messaging requires socket and token authority that
the first route must not acquire.

## Contract Result

No new provider-neutral contract is required. Contracts 005-006, 009-010, 023,
029, 032-033, 037, 039-041, 044-045, 051, and 052 already govern the selected
surface and its exclusions.

Contract 036 requires architecture and contract review before the new package
enters the workspace. That review belongs to the package-acceptance card after
the exact route behavior is implemented. The current `v0.1.1` tag remains a
27-package, 33-route release and must not be described as containing Muse.

## Recommendation

Promote Muse Code ahead of ordinary third-party harness candidates. Its
first-party status, explicit structured protocol, deterministic echo provider,
exact model controls, and event-sourced lifecycle justify the maintenance
cost. Keep the Meta Model API and richer retained-session surface separate so
model access, harness execution, and recovery authority do not collapse into
one route.
