# 221 Cline Headless Model-Selection Evidence

Status: promoted
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Card: g04.074 / 204

## Question

Which exact `cline.headless` provider/model rows, if any, can Swallowtail bind
on qualified package `3.0.55` as immutable model routes without ambient
provider drift, open or catalogue-dependent model membership, silent fallback,
invented effective observation, or unauthorized provider-settings mutation?

## Method And Boundary

Exact `cline@3.0.55` artifacts and tagged sources were inspected on 2026-08-26
in a disposable directory. Cline was not installed onto the host, no platform
binary was downloaded or executed, and no login, credential, account,
catalogue request, provider prompt, or `--json` prompt was used. Host PATH
still has no `cline`.

The selected route remains `cline.headless` (behavior
`cline.headless.stdio-json-v1`, argv `cline --json --auto-approve false` plus
optional `--plan`, `-c <cwd>`, and one prompt operand). It sits on axis
`cline.package` `3.0.55`, and it currently selects neither a provider nor a
model. `cline.acp` is cited only as the early-return contrast.

Source paths are read at GitHub commit
`ad442cbb6a81d21773ceabc1398ea5eb58170718`, the commit Research 146, 147, 190,
and 220 already froze for annotated tag `cli-v3.0.55`
(`c238103e631d492b97bf9e63b060390f1bb8a8a6`). `apps/cli/package.json` there is
`@cline/cli@3.0.55` and `apps/cli/README.md` is byte-identical to the published
wrapper README (`94c3c1b2…`). Wrapper integrity is unchanged from Research 146:
`sha512-3JQ5vPl8/BIyTShTpn0VDX59lbzFhGxRwYqZmAVLOPcX83ETaHbgH2fc9iJYv1pB2ChIpXQEXmFHowgz8F1GgQ==`
(npm shasum `88172d7b7ace564811185858da42b8f62a56751b`).

The published `cline` wrapper is a Node shim that resolves an optional platform
package. The compiled binary was not extracted, so every finding below is
source-level for the exact tagged package point. Current official pages are
leads; the exact package is the finding.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| npm `cline@3.0.55` dist integrity | wrapper identity unchanged from Research 146/147/190/220 | 2026-08-26 | `sha512-3JQ5vPl8/BIyTShTpn0VDX59lbzFhGxRwYqZmAVLOPcX83ETaHbgH2fc9iJYv1pB2ChIpXQEXmFHowgz8F1GgQ==` |
| `apps/cli/package.json` | `@cline/cli@3.0.55` at the tagged commit | 2026-08-26 | `669d6f7b553bdeb6131894cd55cb81f191c03e767d5186a5ff71d0be181e9064` |
| `apps/cli/README.md` | byte-identical to the published wrapper README | 2026-08-26 | `94c3c1b240b5e8a971cc806876dde2e893c78c8cc0085efe23c961e113c197ad` |
| `apps/cli/src/commands/program.ts` | commander `-P/--provider`, `-m/--model`; raw copy in `commanderToParsedArgs` | 2026-08-26 | `392401e42a77120d80b852eed43b81a8f24f1b2fe1d399a2a952181583692aec` |
| `apps/cli/src/utils/helpers.ts` | pre-commander `normalizeCliArgs`; sandbox data-dir wiring | 2026-08-26 | `61330c996391129868de943cd04d093532c4e4bee865f4b0b44aca521666488e` |
| `apps/cli/src/utils/types.ts` | `ParsedArgs` / `Config` model and provider fields | 2026-08-26 | `92c6553ff513179b18f084469dc8b4a568766f4e2fb4012ce491e09e28ee95f7` |
| `apps/cli/src/utils/provider-auth.ts` | `normalizeProviderId`, persisted key lookup | 2026-08-26 | `e3bd9cd99f4d3fba16cd4fef48a1a641a3abf82f3fa1903b44f6168471042c63` |
| `apps/cli/src/utils/output.ts` | `writeln` suppressed in JSON mode; `writeErr`/`emitJsonLine` envelopes | 2026-08-26 | `f1b8131f50d69dcacc773d7e968c363c2db3ab508048691a67916c073e257a5e` |
| `apps/cli/src/main.ts` | ACP early return; provider/model precedence; `saveProviderSettings` | 2026-08-26 | `1b7a0a5b680aa6f3f736826c449f64e9a62dbe7b57e0b55cbc74379d45f37274` |
| `apps/cli/src/runtime/run-agent.ts` | verbose-gated `run_start`; `run_result.model` | 2026-08-26 | `34d6dfc8e7324ad91f6e6a9daa032d11aeed36fbd9e1e790af3d9f3c828de5e6` |
| `sdk/packages/core/src/services/storage/provider-settings-manager.ts` | durable `providers.json` write and `lastUsedProvider` | 2026-08-26 | `98b6ca9e49e5ed9f37ed779758ac93ad8eea008884af2e5cc4f0d32553bc3549` |
| `sdk/packages/core/src/services/llms/provider-settings.ts` | `ProviderSettingsSchema`; `toProviderConfig`; `includeKnownModels` | 2026-08-26 | `a94dade2e3d750ee7ccd377686c5819a1e2501e0c8752367abe4dcf17539b0af` |
| `sdk/packages/core/src/services/llms/provider-defaults.ts` | `resolveProviderConfig`, `mergeKnownModels`, private/public model fetchers | 2026-08-26 | `29e6bbc2b5e142047184dab334ef44216cdd68af3d7ec6034d364f576b65e12a` |
| `sdk/packages/core/src/services/llms/configured-provider-registry.ts` | configured-provider handler config | 2026-08-26 | `2f1183bfe1e586557937057172a836ba9456aca4409dec6874741e7a8d412202` |
| `sdk/packages/core/src/services/llms/runtime-registry.ts` | handler creation from configured providers | 2026-08-26 | `6eb597790bc796435de49b5592faf3152b75f3870d472865bd3587e9338d89e6` |
| `sdk/packages/core/src/runtime/host/local-runtime-host.ts` | session manifest model/provider; abort-path result model | 2026-08-26 | `61fce1f2f9b9061b86721f48a7a1786fa869fc96b566c4cd18fc3379c0ed0743` |
| `sdk/packages/agents/src/agent-runtime.ts` | `messageModelInfo` derived from requested `providerId`/`modelId` | 2026-08-26 | `340976da43ba5bc423cd9094a4cc6a61ec8d34ec853eeba2bc037b1f773333a5` |
| `sdk/packages/shared/src/agents/types.ts` | `AgentResult.model` shape `{id, provider, info?}` | 2026-08-26 | `aaf98184c8f326102872e52ed80e09724ac2268b15d4b72b74e1b97758dcfc91` |
| `sdk/packages/llms/src/providers/ids.ts` | `normalizeProviderId` alias map; unknown ids pass through | 2026-08-26 | `4438c4607f3eb27ad81f50ab2382002c9015019b72d72b53beb523a4686a9fc9` |
| `sdk/packages/llms/src/providers/builtins.ts` | `cline` spec; `modelsSourceUrl` registered only for `ollama`/`lmstudio` | 2026-08-26 | `94f9324f9d0f51efaef589370d4a802ddb282d878b4191babc9f49ebb301b98e` |
| `sdk/packages/llms/src/providers/vendors/cline.ts` | Cline vendor spec; no public model source | 2026-08-26 | `b1d009d72e9f57c3314501bb3bbea87cab901f08b7b221475b27c2d9a690acb7` |
| `sdk/packages/llms/src/providers/model-facts.ts` | explicit accommodation of "user-typed unlisted ids" | 2026-08-26 | `a3af38a5d3761e0555777c2efd663b0b503e34eb5abec047baac67c4666645d1` |
| `sdk/packages/llms/src/providers/model-registry.ts` | registry mutation surface; no id validation gate | 2026-08-26 | `186005c71e1a6a891c1503a4901858dcb5007e5c7491e81ee15ec93ea2c8f6d3` |
| `sdk/packages/llms/src/catalog/catalog-live.ts` | network models.dev catalogue fetch | 2026-08-26 | `ff3a6b19bfb2f65c0a51b19e5e1703c0b087affee7a3aa03195567b0647ee2b5` |
| `sdk/packages/llms/src/catalog/model-id-aliases.ts` | Vercel/OpenRouter alias canonicalization rules | 2026-08-26 | `d6300d9f37b12aa3b687880431a63900e1fa1febfe9263a7cf3579d9db6fa9ae` |

## Parser

Both options are ordinary commander value options declared on the root command:

```ts
// apps/cli/src/commands/program.ts — addRootOptions
.option("-P, --provider <id>", "Provider id (default: cline)")
.option("-m, --model <model-id>", "Model to use for the session with the selected provider")
```

`commanderToParsedArgs` copies both raw, with no trim, case normalization,
alias, or validation:

```ts
if (opts.model !== undefined) result.model = opts.model;
if (opts.provider !== undefined) result.provider = opts.provider;
```

There is no `invalidModel` or `invalidProvider` counterpart to the
`invalidThinkingLevel`, `invalidCompactionMode`, and `invalidRetries` fields
the same function produces for other options. Nothing in the CLI rejects a
model or provider value.

`normalizeCliArgs` rewrites only `--autoapprove`/`--auto-approve`,
`--thinking`, and `--reasoning-effort`. It does not touch `-m`, `--model`,
`-P`, or `--provider`. Unlike a bare `--thinking`, both options declare a
required value, so `--model <id>` cannot swallow a following prompt operand.
Canonical placement after `--auto-approve false` and any `--plan`, and before
`-c <cwd> <prompt>`, is safe.

| Form | Exact `3.0.55` behavior |
| --- | --- |
| `--model <id>`, `--model=<id>`, `-m <id>` | accepted; value copied verbatim |
| repeated `--model` | non-variadic commander option; last occurrence wins; no conflict diagnostic |
| trailing `-m` with no value | `CommanderError` with non-zero exit; `writeErr` runs before `setCurrentOutputMode`, so the message is plain stderr text, not a `{type:"error"}` envelope |
| `--model ""` | `opts.model !== undefined`, so `result.model = ""`; the downstream `??` chain keeps `""` and `config.modelId` becomes the empty string |
| `--provider ""` or whitespace | `args.provider?.trim() \|\| …` is falsy, so provider falls back silently to ambient last-used, then `cline` |
| omission | no `model`/`provider` in `ParsedArgs`; ambient resolution applies |

The empty-value asymmetry is real: model uses `??` and keeps an empty string,
provider uses `||` and silently reverts to ambient state.

## Provider Resolution

```ts
// apps/cli/src/main.ts
const lastUsedProviderSettings =
    providerSettingsManager.getLastUsedProviderSettings({ isClinePassEnabled: true });
const provider = normalizeProviderId(
    args.provider?.trim() || lastUsedProviderSettings?.provider || "cline",
);
let selectedProviderSettings = providerSettingsManager.getProviderSettings(provider);
```

Precedence is explicit argv, then `lastUsedProvider` read from durable
`providers.json`, then the literal `cline`. `normalizeProviderId` is an alias
map lookup over a trimmed string; an unrecognized id passes through unchanged
and is never rejected.

Omitting `-P` therefore leaves provider identity ambient. The prepared route
observes nothing about which provider the host has configured: preparation
classifies only the installed executable and binds a local-account access
profile whose audience is `cline.local-account`, the provider-owned settings
store. That store holds settings for every provider the host has configured,
not a chosen LLM provider. Nothing in current route or access facts derives
provider id `cline`, so an adapter-fixed provider argument is not proven.

Passing an explicit `-P` fixes the provider for the child but does not make it
free: it participates in the durable write below.

## Model Resolution And Membership

```ts
// apps/cli/src/main.ts — Config construction
modelId:
    args.model ??
    selectedProviderSettings?.model ??
    knownModelIds[0] ??
    "anthropic/claude-sonnet-4.6",
```

Precedence is explicit argv, then the persisted per-provider model, then the
first key of the resolved catalogue, then a hardcoded fallback. Explicit argv
wins outright — which also means membership never gates an explicit value.

`knownModelIds` comes from one call:

```ts
const persistedProviderConfig =
    providerSettingsManager.getProviderConfig(provider, { includeKnownModels: false });
const catalogOptions = isInteractive
    ? { loadLatestOnInit: true, loadPrivateOnAuth: true, failOnError: false }
    : undefined;
const resolvedProviderConfig =
    await coreServer.resolveProviderConfig(provider, catalogOptions, persistedProviderConfig);
knownModels = resolvedProviderConfig?.knownModels;
```

`--json` forces `isHeadless`, so `isInteractive` is false and `catalogOptions`
is `undefined`. `includeKnownModels: false` leaves `config.knownModels`
undefined, so the `userKnownModels` merge input is empty. Within
`resolveProviderConfig`, that means `getLiveModelsCatalog` is not called on
this route.

The remaining merge inputs are not uniform across providers:

| Membership source | Gate at `3.0.55` | Consequence for `cline.headless` |
| --- | --- | --- |
| generated catalogue + bundled `defaults.knownModels` | always | package-static at this commit |
| live models.dev catalogue | `modelCatalog.loadLatestOnInit` | not reached on the selected argv |
| private per-account models | `PRIVATE_PROVIDER_MODEL_FETCHERS[provider]` and an auth token; registered for `baseten`, `hicap`, `litellm`, `poolside` | account-scoped network read for those providers |
| public model source | provider spec declares `modelsSourceUrl`; registered only for `ollama` (`http://localhost:11434/api/tags`) and `lmstudio` (`http://localhost:1234/v1/models`) | live host read that *replaces* the bundled list for those providers |

For provider `cline` specifically, `mergeKnownModels` takes the generated plus
bundled sets, sorts by release date, and applies
`preferCanonicalModelIds(..., VERCEL_OPENROUTER_MODEL_ID_ALIAS_RULES)`. That
set is package-static at this commit, but it is release-date ordered and
alias-canonicalized, so `knownModelIds[0]` is a rotating default, not a fixed
identity. For `ollama` and `lmstudio` the set is whatever the host's local
server currently reports. For `litellm` it is the private fetch alone.

A catalogue resolution failure is swallowed: the `catch` calls `writeln`, and
`writeln` returns immediately when the output mode is `json`. The run then
continues with `knownModels` undefined, `knownModelIds` empty, and `modelId`
at the hardcoded `anthropic/claude-sonnet-4.6`, with nothing on the selected
wire indicating that happened.

Under Contract 020 none of these sets is a preflight allowlist. Catalogue
presence "does not prove … existence of a configured model route" or "a
default model, provider, endpoint, or routing preference", and a bundled
catalogue read inside the child cannot be observed by Swallowtail preflight in
any case — the platform binary carries its own embedded copy and was never
extracted.

## Explicit Selection Is Never Validated

Between `commanderToParsedArgs` and `Config` there is no comparison of
`args.model` against `knownModelIds`, against `selectedProviderSettings`,
against the selected provider, or against any table. Provider/model agreement
is never checked; a model belonging to one provider is dispatched unchanged
under another.

The routing layer treats unlisted ids as supported rather than invalid.
`model-facts.ts` names the case directly — "user-typed unlisted ids such as
`claude-opus-4-6:1m`" — and Research 190 already froze the matching reasoning
path, where an unlisted model takes a reduced-fact branch instead of a
rejection.

So an invalid or mismatched identifier fails, if at all, inside the child at
provider request time. From Swallowtail's position that is entirely
post-spawn: the process has started, the ambient credential has been accepted,
the prompt has been sent, and the durable write below has already happened.
There is no fail-closed membership gate before provider effects.

## Configuration Persistence

The CLI persists the resolved selection unconditionally, before the run, on
the headless path only (the ACP branch returns earlier):

```ts
// apps/cli/src/main.ts
try {
    const persistApiKey = providedApiKey
        ? { apiKey: providedApiKey }
        : apiKey && !isOAuthProvider(provider) ? { apiKey } : {};
    providerSettingsManager.saveProviderSettings({
        ...(selectedProviderSettings ?? {}),
        provider,
        model: config.modelId,
        ...persistApiKey,
    });
} catch (error) { /* writeln — suppressed in JSON mode */ }
```

`saveProviderSettings` is a durable write, not a cache update:

```ts
// provider-settings-manager.ts
const shouldSetLastUsed = options.setLastUsed !== false;
const next: StoredProviderSettings = {
    ...previous,
    providers: { ...previous.providers, [providerId]: { settings: validatedSettings, updatedAt: nowIso(), tokenSource } },
    lastUsedProvider: shouldSetLastUsed ? providerId : previous.lastUsedProvider,
};
this.write(next);
```

`main.ts` passes no options, so `setLastUsed` defaults on and the write also
moves `lastUsedProvider`. `write` stages a pid-unique temp file and renames it
over `resolveProviderSettingsPath()` — `~/.cline/settings/providers.json`
unless redirected — then chmods `0600`. The source's own comment states that
"Concurrent Cline processes (CLI, extension, hub) share this file". The schema
field is `model: z.string().optional()` with no minimum length, so an empty
model persists as `""`.

Three findings follow.

**The write already happens today.** The current `cline.headless` argv reaches
this code, so the existing route already rewrites the provider entry,
refreshes `updatedAt`, and re-stamps `lastUsedProvider` on every run. Where no
entry existed, it creates one carrying `knownModelIds[0]` or the hardcoded
fallback. That is pre-existing route truth under `Ambient`; this lane records
it rather than repairing it.

**Explicit selection changes what is written.** With `-m`, the caller's model
becomes the host's persisted default for that provider. With `-P`, the
caller's provider becomes `lastUsedProvider`. Both durably change the
resolution of every later Cline run, including runs by the VS Code extension
and hub. Today's write is ambient-derived and effectively idempotent; a
caller-directed write is not.

**The write cannot be disabled or operation-scoped.** No CLI flag gates it.
The only containment is `--data-dir`, `--config`, or `CLINE_SANDBOX=1`, which
redirect Cline's entire state root — `CLINE_DATA_DIR`, the database, sessions,
teams, `CLINE_PROVIDER_SETTINGS_PATH`, and the hooks log — to a synthesized
directory. That is temporary home and configuration construction.

Contract 033 settles this. No posture "grants configuration-file discovery,
parsing, mutation, migration, installation, or deletion authority", and
`HostScoped` explicitly "does not authorize an adapter to create a temporary
home or copy ambient files" — and its lease and host service do not exist in
the current runtime, so pure preflight rejects the posture regardless. A
caller-directed durable write to a file shared with other Cline surfaces is
new configuration authority. Under the card's own gate that is a stop, not an
incidental provider detail.

Failure of the write is silent on this route: the `catch` calls `writeln`,
which is a no-op in JSON mode. A consumer cannot tell a successful persist
from a failed one.

## Application, Output, And Observation

`config.providerId` and `config.modelId` reach `runAgent`, the session start
input, the session manifest (`provider`, `model`), the agent config, and the
provider handler. Application at the CLI boundary is exact.

Observation is not. `run_start` carries `providerId`, `modelId`, and `catalog`,
but its sole emitter is `printModelProviderInfo`, called under
`if (config.verbose)`. The selected argv passes no `-v/--verbose`, so no
`run_start` line is emitted. This is unchanged from Research 190.

`run_result` *is* emitted on the selected argv and does carry a `model` field.
It is not an effective-value observation:

```ts
// agent-runtime.ts — resolveRuntimeConfig
const messageModelInfo = rest.messageModelInfo ?? { id: modelId, provider: providerId };
```

`AgentResult.model` is `{ id, provider, info? }` built from the requested
`modelId`/`providerId`; the abort path in `local-runtime-host.ts` constructs
the same shape from `session.config`. `run_result.model` therefore echoes the
value Swallowtail placed in argv. An echo of the request is not acceptance
evidence that the provider applied it. `run_result` is also absent on the
abort and non-completed paths.

One audit note: the named fixture
`tests/fixtures/cline-headless-3.0.55/success.jsonl` models `run_result.model`
as a bare string, while the exact source emits the `{id, provider}` object.
The Swallowtail decoder reads only `finishReason` and `text` from
`run_result`, so nothing depends on the shape and no claim is wrong, but the
example line is inaccurate for the frozen wire.

Requested, planned, and dispatched states are Swallowtail-owned.
Parser-accepted and CLI-applied states are source-visible. Provider-applied
and effective model state is withheld.

## Omission, Lifecycle, And Production Seam

Omission retains the exact current argv
`cline --json --auto-approve false [--plan] -c <cwd> <prompt>` and leaves
provider and model to ambient last-used settings, the bundled catalogue order,
and the hardcoded fallback. That ambient default is existing route truth, not
a Swallowtail model claim.

Production is unchanged and correct as it stands.
`ClineHeadlessRunProfileInput` carries request identity, content, working
resource, deadline, and optional `HarnessMode`; it has no model route.
`headless/validation.rs` rejects any plan carrying `model_id()` or
`model_route_id()`. `headless/command.rs` emits no `-m` or `-P`, and its
negative assertions keep them out. The guide states there is no model
catalogue or caller-supplied model route on this route. `ModelRoute` exists in
`swallowtail-core` and would be the shape to bind, but no admitted row exists
to bind to it. The API baseline, fixtures, example, and feature matrices need
no change.

Had a row been admitted, argv immutability and lifecycle would not have been
the obstacle: argv is fixed at spawn, the headless path never rebuilds
provider or model, `--model` composes with `--plan` without interaction, and
one-run activity, terminal, cancellation, deadline, failure, retention, and
joined cleanup are all independent of model identity.

## Thinking Dependency

Research 190 named absent provider and model selection as the reason
`cline.headless` cannot qualify a thinking value. That dependency is not
removed. Because no exact provider/model row is admitted here, cards 117-118
stay blocked for the same reason and are not reopened by this record. No
reasoning claim is made or implied.

## Route And Value Dispositions

| Route | Value | CLI parse | Applied by the exact route | Disposition | Reason |
| --- | --- | --- | --- | --- | --- |
| `cline.headless` | any exact model via `-m/--model`, provider omitted | accepted | yes, as `config.modelId` | withheld | provider identity stays ambient (`lastUsedProvider` else `cline`); the pair cannot be tied to the configured instance or access audience |
| `cline.headless` | exact model via `-m` plus adapter-fixed `-P cline` | accepted | yes | withheld | no current route or access fact derives provider `cline`; audience `cline.local-account` is the shared settings store, not a provider choice |
| `cline.headless` | exact model via `-m` plus caller-selected `-P <id>` | accepted | yes | withheld | caller provider selection is an explicit non-goal |
| `cline.headless` | any model drawn from bundled or generated `knownModels` | n/a | n/a | withheld | Contract 020: catalogue presence proves no model route; membership is release-date ordered, alias-canonicalized, and for some providers live or account-scoped |
| `cline.headless` | unknown or provider-mismatched model id | accepted | yes | not applicable | no local validation; fails late at the provider or not at all |
| `cline.headless` | `--model ""` | accepted | yes, as the empty string | not applicable | empty value survives the `??` chain and persists as `""` |
| `cline.headless` | omitted | n/a | ambient provider and model resolution | retain existing behavior | current argv and route truth are unchanged |
| `cline.acp` | `-m` / `-P` | accepted then discarded | no | not applicable | ACP returns before provider/model resolution |
| any other package point | all | n/a | n/a | evidence-gated | no `UnverifiedNewer` inheritance; currentness stays standing |

## Contract 020 And 033 Decision

A `ModelRoute` needs one exact model and provider identity that the qualified
route dispatches, validates before provider effects, and holds immutable for
the bounded run, in agreement with the configured instance and access
audience.

Exact `3.0.55` fails on three independent legs, any one of which is a stop:

1. **Provider stays ambient.** Without `-P` the provider is `lastUsedProvider`
   from durable settings, else `cline`. The route observes none of that. With
   `-P` it becomes either a caller provider selector, which is out of scope,
   or an adapter-fixed assumption that no route or access fact proves.
2. **Membership is open.** Explicit `-m` bypasses every table. There is no
   validation, no invalid-model exit path, no provider/model agreement check,
   and unlisted ids are explicitly accommodated downstream. Any Swallowtail
   allowlist would be a catalogue claim Contract 020 forbids, unverifiable
   against the child's embedded catalogue, and — for provider `cline`, which
   is usage-billed under `EntitlementMetering::SubscriptionAllowance` —
   entitlement-scoped in a way the route never observes.
3. **Selection mutates durable ambient configuration.** `saveProviderSettings`
   runs before the run, cannot be disabled or operation-scoped by any
   invocation, and writes the caller's model and provider into a settings file
   shared with the extension and hub. Contract 033 grants no such authority,
   and the only containment is a synthesized configuration root the same
   contract prohibits.

Observation would not have rescued a marginal case. `run_start` stays behind
an unselected `--verbose`, and `run_result.model` is a request echo, not a
provider-confirmed applied model.

## Behavior Revision And Compatibility

No behavior revision is needed or proposed. `cline.headless.stdio-json-v1` and
the `cline.package` `3.0.55` qualified point are unchanged, and the
compatibility claim ceiling is unchanged. Contract 029 currentness stays in its
standing lane; this record qualifies no newer point.

## Promotion

Research 221 promotes no deliver-now `cline.headless` provider/model row. The
deliver-now set is empty.

Card 205 must not bind a `ModelRoute`. Card 206 has no dispatch to prove. The
Cline adapter, its fixtures, the headless guide, the route and feature
matrices, and the unreleased package API baseline are unchanged.

A later lane may reopen this only with an exact package point that fixes
provider identity from route facts, closes model membership before provider
effects, and either omits the provider-settings write or exposes a way to
scope it — or with separately authorized Swallowtail configuration authority
that Contract 033 does not currently grant.

Cline ACP model selection, caller provider selection, API keys, credential and
catalogue work, thinking delivery, aliases, fallback, settings mutation,
temporary configuration roots, sibling routes, currentness, and release remain
out of scope.
