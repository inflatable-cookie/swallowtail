# 2026-08-26 g04.074 Cline Headless Model Selection Closeout

Status: stopped after evidence
Owner: Tom
Milestone: g04.074
Cards: 204 complete; 205-206 blocked
Branch: `t3code/review-headless-model-selection`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-cba8f57c`
Base: `60cc3ec7e50568ee56fe3575eb53f8d4835bed7b` (`origin/main` at dispatch)
Planning base ancestor: `279a53c0f8ddf5896e457dd57eb3b639ae07d272`
PR: https://github.com/inflatable-cookie/swallowtail/pull/73

## Result

Card 204 completed an exact `cline@3.0.55` package, parser, provider/model
precedence, membership, persistence, application, output, and production-seam
audit. Research 221 admits no deliver-now provider/model row. Cards 205 and 206
are blocked and were not executed. The Cline adapter, prepared facade, child
argv, fixtures, guide, matrices, and API baseline are unchanged. No install,
login, account inspection, catalogue request, provider prompt, configuration
mutation, live probe, or paid operation was used.

## Evidence Stop

Exact `3.0.55` declares `-P, --provider <id>` and `-m, --model <model-id>` as
ordinary commander value options. `commanderToParsedArgs` copies both raw, with
no trim, alias, case handling, or validation, and produces no `invalidModel` or
`invalidProvider` counterpart to the fields it produces for `--thinking`,
`--compaction`, and `--retries`. `normalizeCliArgs` does not touch either
option, so `--model <id>` cannot swallow the prompt operand the way a bare
`--thinking` can.

Three gates fire independently.

Provider identity stays ambient. `args.provider?.trim() ||
lastUsedProviderSettings?.provider || "cline"` means omission resolves to
durable `lastUsedProvider` and then the literal `cline`, and an empty or
whitespace `-P` silently reverts to the same chain. Nothing in preparation
observes which provider the host has configured; audience
`cline.local-account` names the shared provider-settings store, not a provider
choice, so an adapter-fixed argument is unproven and a caller selector is out
of scope.

Model membership stays open. `args.model ?? selectedProviderSettings?.model ??
knownModelIds[0] ?? "anthropic/claude-sonnet-4.6"` lets an explicit value win
outright, and nothing compares it to `knownModels`, to the selected provider,
or to any table. `model-facts.ts` explicitly accommodates "user-typed unlisted
ids", so an invalid or mismatched identifier fails only inside the child at
provider request time — post-spawn from Swallowtail's position. Resolved
membership is also not uniform: for `cline` it is the generated plus bundled
sets, release-date ordered and alias-canonicalized; for `ollama` and `lmstudio`
it is a live local-server read that replaces the bundled list; for `baseten`,
`hicap`, `litellm`, and `poolside` it is an account-scoped fetch. Contract 020
forbids turning any of those into a preflight allowlist.

Explicit selection mutates durable ambient configuration.
`saveProviderSettings({...selected, provider, model: config.modelId})` runs
unconditionally before the run on the headless path, writes the entry with a
fresh `updatedAt`, and — because `main.ts` passes no options — moves
`lastUsedProvider`. `write` renames over `~/.cline/settings/providers.json`,
which the source's own comment says the CLI, VS Code extension, and hub share.
No flag disables or scopes the write; only `--data-dir`, `--config`, or
`CLINE_SANDBOX=1` contain it, and each redirects Cline's whole state root.
Contract 033 grants no such authority and prohibits the synthesized
configuration root. Failure is invisible on this route because the `catch`
calls `writeln`, a no-op in JSON mode.

Observation does not rescue a marginal case. `run_start` is emitted only from
`printModelProviderInfo` under `if (config.verbose)`, unchanged from Research
190. `run_result.model` is emitted on the selected argv but is
`{id, provider}` built from `messageModelInfo`, itself derived from the
requested `modelId`/`providerId`. It echoes what Swallowtail put in argv; it is
not a provider-confirmed applied model.

Pre-existing route truth recorded, not repaired: the current `cline.headless`
argv already triggers the same durable write with ambient-derived values. What
explicit selection changes is the content — a caller-directed model and
provider replacing the host's persisted defaults.

## Changed Surfaces

- `docs/research/221-cline-headless-model-selection-evidence.md`: promoted
  exact package, parser, provider and model precedence, membership sources,
  validation absence, persistence, observation, production audit, and empty
  deliver-now table
- cards 204-206, g04.074, g04 README, programme, triage, indexes, sole Next
  Task, this closeout

No production code, public API, shared contract/runtime, fixture, guide
capability, matrix, or changelog edit.

## Validation

Passed:

- `effigy validate:focused swallowtail-adapter-cline`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

`effigy doctor` at dispatch reproduced the inherited baseline exactly: 379
scan findings (333 warnings, 46 errors) plus one generated-in-src finding. No
drift; unrelated findings were not repaired. Docs-only edits do not change it.

## Carried Risk

`crates/swallowtail-adapter-cline/tests/fixtures/cline-headless-3.0.55/success.jsonl`
models `run_result.model` as a bare string, while exact `3.0.55` emits a
`{id, provider}` object. The headless decoder reads only `finishReason` and
`text` from `run_result`, and `protocol.json` names the field without claiming
a shape, so no Swallowtail claim is wrong. The example line is still
inaccurate for the frozen wire and should be corrected in a later Cline
fixture-truth lane. Card 204 is evidence-only, so it was not changed here.

## Continuation

Keep g04 open. Reassess the remaining per-route feature inventory for the next
serial lane unless the operator supplies a different direction. `cline.headless`
`-m` is evidence-closed at `3.0.55`; reopening needs a later package point or
separately authorized configuration handling, not a re-read. The g04.042
thinking dependency is unchanged, so cards 117-118 stay blocked. Contract 029
currentness remains standing. Do not compile the next family from this
closeout.
