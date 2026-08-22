# 190 Cline Thinking Control Evidence

Status: promoted
Owner: Tom
Created: 2026-08-22
Updated: 2026-08-22
Card: g04.042 / 116

## Question

Which exact `cline.acp` and `cline.headless` thinking values, if any, can
Swallowtail bind on qualified package `3.0.55` as portable reasoning selections
without aliasing, clamping, default substitution, provider/model inference, or
cross-transport promotion?

## Method And Boundary

Current official Cline documentation and exact `cline@3.0.55` artifacts were
inspected on 2026-08-22 in a disposable directory. Cline was not installed onto
the host, no platform binary was downloaded or executed, and no login,
credential, account, catalogue, ACP session, or `--json` prompt was used. Host
PATH still has no `cline`.

The routes remain `cline.acp` (behavior `cline.acp.stdio-v1`, argv
`cline --acp`) and `cline.headless` (behavior `cline.headless.stdio-json-v1`,
argv `cline --json --auto-approve false` plus one prompt operand and optional
`-c`). Both sit on axis `cline.package` `3.0.55`. Neither selects a provider or
a model.

Current official pages are leads. The exact package is the finding.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| npm `cline@3.0.55` wrapper tarball | package identity; the 6-file Node wrapper carries no thinking logic | 2026-08-22 | `7eec2ad80d8dfa27b9baaa22c7340ebe861850f6057b9e2e80a5dd9d2ef2f5ef` |
| published `package/README.md` | exact packaged flag table: optional level, bare-flag `medium`, "off when omitted" | 2026-08-22 | `94c3c1b240b5e8a971cc806876dde2e893c78c8cc0085efe23c961e113c197ad` |
| [CLI reference](https://docs.cline.bot/cli/cli-reference) | current official `--thinking <level>` table stating `(default medium)` | 2026-08-22 | `830872afa73875f573d39ab8f41d48ebe73c1901e6b0578cc71c2f96cbad522f` |
| [ACP](https://docs.cline.bot/usage/acp) | current official ACP page; contains no thinking or reasoning text | 2026-08-22 | `d5bf84e51972f2ca892d1eebf36bc1418e43395d8c3893b10d6e399e6802e76b` |
| `apps/cli/src/commands/program.ts` | commander declaration and value parse | 2026-08-22 | `392401e42a77120d80b852eed43b81a8f24f1b2fe1d399a2a952181583692aec` |
| `apps/cli/src/utils/helpers.ts` | pre-commander argv normalization and legacy alias | 2026-08-22 | `61330c996391129868de943cd04d093532c4e4bee865f4b0b44aca521666488e` |
| `apps/cli/src/utils/reasoning.ts` | explicit-versus-persisted precedence | 2026-08-22 | `edc026f2207b5ef0135a0d74ba9727e71ab577417a8bb475080b3448f079c687` |
| `apps/cli/src/main.ts` | invalid-value rejection, ACP early return, print-path config | 2026-08-22 | `1b7a0a5b680aa6f3f736826c449f64e9a62dbe7b57e0b55cbc74379d45f37274` |
| `apps/cli/src/acp/index.ts` | ACP entry options | 2026-08-22 | `8cbcf4c04bb9b01ccc877c661210346380732a32b796ac8335663b5d141adbb7` |
| `apps/cli/src/acp/acpAgent.ts` | ACP session config construction | 2026-08-22 | `248092d41e330ef1898f98b99d35c6713574a7b9305d95601177c07e64db9e71` |
| `apps/cli/src/runtime/run-agent.ts` | verbosity-gated `run_start` thinking field; sole call site | 2026-08-22 | `34d6dfc8e7324ad91f6e6a9daa032d11aeed36fbd9e1e790af3d9f3c828de5e6` |
| `sdk/packages/core/src/runtime/host/local-runtime-host.ts` | persisted provider-config effort fallback | 2026-08-22 | `61fce1f2f9b9061b86721f48a7a1786fa869fc96b566c4cd18fc3379c0ed0743` |
| `sdk/packages/llms/src/providers/gateway.ts` | reasoning merge and disable short circuit | 2026-08-22 | `3ef8893d5275709cec98d429f42c808b21feb959bd57b50032da25c6a160c238` |
| `sdk/packages/llms/src/providers/routing/reasoning-options.ts` | model-entitled normalization, off support, budget derivation | 2026-08-22 | `2785f6692bccd7a0b1573a271b107b0e404c6e7d51b0383ca9c7a77577829d37` |
| `sdk/packages/llms/src/providers/routing/portable-reasoning.ts` | provider-id gate for portable off and effort | 2026-08-22 | `16b12edead31941b7894419978203ab1592e214958c40d7065b849ef52776594` |
| `sdk/packages/llms/src/providers/model-facts.ts` | nearest-value effort clamp | 2026-08-22 | `a3af38a5d3761e0555777c2efd663b0b503e34eb5abec047baac67c4666645d1` |
| `sdk/packages/shared/src/llms/reasoning-options.ts` | seven-level upstream vocabulary | 2026-08-22 | `9f17376a0228407a3b1088c64cffc71bbc61dbe1d221b871bf89117df4c8edde` |

Source paths are read at GitHub commit
`ad442cbb6a81d21773ceabc1398ea5eb58170718`, the commit Research 146 and 147
already froze for tag `cli-v3.0.55`. That commit is bound to the published
package by two exact facts: `apps/cli/package.json` there is
`@cline/cli@3.0.55`, and `apps/cli/README.md` there is byte-identical to the
`README.md` inside the published `cline@3.0.55` tarball
(`94c3c1b2…`). Wrapper integrity is unchanged from Research 146:
`sha512-3JQ5vPl8/BIyTShTpn0VDX59lbzFhGxRwYqZmAVLOPcX83ETaHbgH2fc9iJYv1pB2ChIpXQEXmFHowgz8F1GgQ==`.

HTML digests identify retrieved documentation bodies. They are not a
compatibility guarantee. The platform executable was not extracted, so every
finding below is source-level for the exact tagged package point.

## Official Contradiction, Resolved

Three current official descriptions disagree:

- the packaged README names `--thinking [none|low|medium|high|xhigh]`, says a
  bare flag means `medium`, and says "thinking is off when the flag is omitted"
- the CLI reference names `--thinking <level>` with `(default medium)`
- the commander description string in the exact source says
  "Bare `--thinking` uses medium; omitted leaves provider default"

The exact source settles it. Omission is neither `off` nor `medium`:

```ts
// apps/cli/src/utils/reasoning.ts
if (thinkingExplicitlySet) { ... }
if (persistedReasoning?.enabled === false || persistedReasoning?.effort === "none") {
    return { thinking: false, reasoningEffort: undefined };
}
if (isActiveReasoningEffort(persistedReasoning?.effort)) {
    return { thinking: true, reasoningEffort: persistedReasoning.effort };
}
if (persistedReasoning?.enabled === true) {
    return { thinking: true, reasoningEffort: "medium" };
}
return { thinking: undefined, reasoningEffort: undefined };
```

Omitting the flag consults persisted provider settings and otherwise leaves
both fields undefined, so the model or provider default applies. The README's
"off when omitted" is wrong for the exact package. The CLI reference's
`(default medium)` describes only the bare-flag case.

`medium` reaches commander through pre-parse argv rewriting, not through a
commander default:

```ts
// apps/cli/src/utils/helpers.ts — normalizeCliArgs, run before commander
if (token === "--thinking") {
    const nextToken = args[index + 1];
    if (nextToken !== undefined && !nextToken.startsWith("-")) {
        normalized.push("--thinking", nextToken); index += 1; continue;
    }
    normalized.push("--thinking", "medium"); continue;
}
if (token === "--reasoning-effort") { /* same shape, rewritten to --thinking */ }
if (token.startsWith("--reasoning-effort=")) {
    normalized.push(token.replace(/^--reasoning-effort=/, "--thinking="));
}
```

Two consequences. `--reasoning-effort` is an upstream alias of `--thinking`,
not a second option. A bare `--thinking` immediately before the headless prompt
operand would swallow that operand as its level, because the normalizer takes
any following token that does not start with `-`.

The same commit's own `apps/cli/src/cli.e2e.test.ts` expects help text
`--thinking [level]` while `program.ts` declares `--thinking <level>` (it makes
the same mismatched claim for `--auto-approve`). Help spelling is therefore not
reliable evidence at this point. Behavior is unaffected because the normalizer
always supplies a level before commander parses.

## Exact Value Parse

```ts
// apps/cli/src/commands/program.ts — commanderToParsedArgs
if (opts.thinking !== undefined) {
    const effort = String(opts.thinking).trim().toLowerCase();
    if (effort === "none" || effort === "low" || effort === "medium" ||
        effort === "high" || effort === "xhigh") {
        result.thinkingExplicitlySet = true;
        if (effort === "none") { result.thinking = false; result.reasoningEffort = undefined; }
        else { result.thinking = true; result.reasoningEffort = effort; }
    } else if (effort) {
        result.invalidThinkingLevel = effort;
    }
}
```

Values are trimmed and lowercased, so upstream accepts case and whitespace
variants of the five names. An empty value is silently ignored and leaves the
selection unset. Any other non-empty value is rejected before transport work:

```ts
// apps/cli/src/main.ts
if (args.invalidThinkingLevel) {
    writeErr(`invalid thinking level "${args.invalidThinkingLevel}" (expected "none", "low", "medium", "high", or "xhigh")`);
    process.exitCode = 1;
    return;
}
```

That rejection runs before the ACP branch and before the print path, so it is
the one behavior both routes share.

## `cline.acp`: Parsed, Then Discarded

The ACP branch returns before any thinking-bearing configuration is built, and
it forwards exactly one option:

```ts
// apps/cli/src/main.ts
if (args.acpMode) {
    const { runAcpMode } = await import("./acp/index");
    await runAcpMode({ autoApproveTools: args.autoApproveOverride === true });
    return;
}
```

`runAcpMode` passes only `autoApproveTools` into `AcpAgent`, and the agent
hard-codes the field:

```ts
// apps/cli/src/acp/acpAgent.ts — AcpAgent.buildConfig
return {
    providerId, modelId: session.currentModelId, apiKey, systemPrompt,
    execution: undefined, verbose: false, sandbox: false,
    thinking: false,
    outputMode: "text", mode: session.currentMode, ...
};
```

`buildConfig` never sets `reasoningEffort`. No ACP request, session config
option, or documented ACP environment variable in the exact source carries a
thinking level, and the current official ACP page contains no thinking text.

So on `cline.acp` at `3.0.55` a valid `--thinking <level>` is accepted, exits
nothing, and is discarded; the ACP child always runs with `thinking: false`.
Only an invalid level changes behavior, by failing the process. A flag that a
route ignores is not a dispatch surface. This is the card 116 "silently
ignored" stop condition, and it is independent of any model question.

## `cline.headless`: Applied, Then Model-Entitled

The print path does reach the resolver and does carry the selection into the
run configuration:

```ts
// apps/cli/src/main.ts
const resolvedReasoning = resolveCliReasoning({
    thinking: args.thinking,
    thinkingExplicitlySet: args.thinkingExplicitlySet,
    reasoningEffort: args.reasoningEffort,
    persistedReasoning: selectedProviderSettings?.reasoning,
});
// ...
thinking: resolvedReasoning.thinking,
reasoningEffort: resolvedReasoning.reasoningEffort,
```

An explicit CLI level wins over `selectedProviderSettings.reasoning`, so
persisted CLI provider settings cannot override a passed value. That much is
invariant.

Everything after that is provider- and model-entitled, and the route selects
neither. The model is resolved from host state and a catalogue:

```ts
// apps/cli/src/main.ts
modelId: args.model
    ?? selectedProviderSettings?.model
    ?? knownModelIds[0]
    ?? "anthropic/claude-sonnet-4.6",
```

`knownModelIds` comes from `coreServer.resolveProviderConfig(...)`, and the
provider is `-P/--provider`, default `cline`. Swallowtail's headless argv
passes neither `-m` nor `-P`.

A second persisted source reappears below the CLI:

```ts
// sdk/packages/core/src/runtime/host/local-runtime-host.ts
reasoningEffort: configWithProvider.reasoningEffort ?? providerConfig.reasoningEffort,
```

For `--thinking none` the CLI effort is `undefined`, so a persisted
provider-config effort is substituted here. It does not survive, because the
gateway short-circuits an explicit disable:

```ts
// sdk/packages/llms/src/providers/gateway.ts — mergeReasoningOptions
if (legacy?.enabled === false || requested?.enabled === false) {
    return { enabled: false };
}
```

That is a near miss, not a guarantee, and it is the only place the injected
persisted value is dropped.

The resolved intent is then normalized against the model's advertised controls:

```ts
// sdk/packages/llms/src/providers/routing/reasoning-options.ts
if (reasoning.enabled === false && isClineProvider(request.providerId) &&
    isClaudeFableModelId(request.modelId)) {
    return { ...request, reasoning: undefined };   // explicit off discarded
}
const options = context.model.reasoningOptions;
if (options === undefined) {
    // custom/unlisted models get only broadly supported effort values
    const effort = reasoning.effort === "minimal" ? "low"
        : reasoning.effort === "xhigh" || reasoning.effort === "max" ? "high"
        : reasoning.effort;
    return { ...request, reasoning: { ...reasoning, effort } };
}
if (options.length === 0) { return { ...request, reasoning: undefined }; }
// ...
if (reasoning.enabled === false) {
    return controls.supportsOff
        ? { ...request, reasoning: { enabled: false } }
        : { ...request, reasoning: undefined };
}
const effort = normalizeReasoningEffort(reasoning.effort, controls.efforts);
```

```ts
// sdk/packages/llms/src/providers/model-facts.ts — normalizeReasoningEffort
if (supportedEfforts.length === 0) return undefined;
if (supportedEfforts.includes(effort)) return effort;
const requestedIndex = ACTIVE_REASONING_EFFORTS.indexOf(effort);
return supportedEfforts.reduce((nearest, candidate) => { /* nearest by index distance */ });
```

Four distinct hazards, all keyed on the unselected model:

1. nearest-value clamp when the model does not advertise the requested tier
2. outright substitution for unlisted models, where `xhigh` becomes `high`
3. silent removal when the model advertises an empty control list, or when an
   explicit off meets a model that does not support off
4. conversion of a named tier into a derived token budget through
   `resolveReasoningBudgetFromRatio` when the model advertises `budget_tokens`
   rather than efforts

`portable-reasoning.ts` adds a provider gate on top: for a provider outside its
`PORTABLE_REASONING_PROVIDERS` set, `enabled === false` resolves to `undefined`
rather than a portable off.

The selected route returns no acknowledgement of the value at all. The only
envelope that carries a thinking field is emitted behind a verbosity guard:

```ts
// apps/cli/src/runtime/run-agent.ts — runAgent
if (config.verbose) {
    printModelProviderInfo(config);
}
```

```ts
// apps/cli/src/runtime/run-agent.ts — printModelProviderInfo
const thinking = config.thinking ? "on" : "off";
if (config.outputMode === "json") {
    emitJsonLine("stdout", { type: "run_start", providerId, modelId, catalog, thinking, mode, sessionId });
    return;
}
```

`printModelProviderInfo` is the sole call site of that envelope, and it runs
only when `config.verbose` is true. The selected `cline.headless` argv is
`--json --auto-approve false -c <cwd> <prompt>`
(`crates/swallowtail-adapter-cline/src/headless/command.rs`), which passes no
`-v/--verbose`. So no `run_start` line is emitted on this route and the wire
reports nothing about the requested value.

`--verbose` is an unselected surface, and selecting it would not rescue the
mapping: `run_start.thinking` is a boolean that cannot distinguish `low` from
`medium`, `high`, or `xhigh`, and it reports `off` for `none` whatever the
routing layer later does with the request.

So `cline.headless` at `3.0.55` has no acceptance evidence for a thinking
value under Contract 040 — not a coarse one, none. Dispatch stops at argv.

## Syntax And Lifetime

| Item | Exact `3.0.55` finding | Disposition |
| --- | --- | --- |
| Canonical flag | `--thinking`; `--reasoning-effort` rewritten to it before commander | alias, not a second public option |
| Value syntax | `--thinking <v>` and `--thinking=<v>`; bare flag rewritten to `medium` | bare form would eat a following prompt operand |
| Documented values | `none`, `low`, `medium`, `high`, `xhigh` | upstream vocabulary is seven levels; `minimal` and `max` are not CLI values |
| Case | trimmed and lowercased before comparison | upstream case-insensitivity, not a portable input |
| Empty value | silently ignored, selection stays unset | not a selection |
| Invalid value | stderr message, exit code 1, before both transports | the one shared fail-closed behavior |
| Omission | persisted provider reasoning, else model/provider default | not `off`, not `medium`; retain the existing Swallowtail path |
| ACP spawn | discarded; `buildConfig` hard-codes `thinking: false` | no dispatch surface |
| ACP first/later prompt | same hard-coded child config | nothing to repeat |
| ACP fresh replacement | a new child would re-pass an argv the child ignores | lifetime fit does not create a capability |
| Headless one run | applied to that child's `Config` only | scope is correct; entitlement is not |
| Headless persisted precedence | explicit CLI beats `providerSettings.reasoning` | correct at the CLI boundary only |
| Headless second persisted source | `providerConfig.reasoningEffort` fills an undefined effort below the CLI | dropped for `none` by the gateway short circuit |
| Headless acknowledgement | none on the selected argv; `run_start` is emitted only under `--verbose`, which the route does not pass | nothing to accept |
| Headless acknowledgement under an unselected `--verbose` | `run_start.thinking` is `"on"`/`"off"` | would still not confirm a tier |

## Route And Value Dispositions

| Route | Value | CLI parse | Applied by the exact route | Disposition | Reason |
| --- | --- | --- | --- | --- | --- |
| `cline.acp` | `none` | accepted | no | withheld | ACP returns before config; `buildConfig` hard-codes `thinking: false` |
| `cline.acp` | `low`, `medium`, `high`, `xhigh` | accepted | no | withheld | same discard path; accepted and silently ignored |
| `cline.headless` | `none` | accepted | yes, as `thinking: false` | withheld | not exact portable `off`: dropped unless the unselected model advertises off, and dropped for Cline-provider Claude Fable because reasoning is mandatory there |
| `cline.headless` | `low`, `medium`, `high` | accepted | yes, as an effort | withheld | clamped to the nearest advertised tier, removed for empty control lists, or converted to a derived budget, all keyed on the unselected model |
| `cline.headless` | `xhigh` | accepted | yes, as an effort | withheld | additionally substituted to `high` for any model with no advertised controls |
| both routes | `minimal`, `max` | rejected, exit 1 | no | not applicable | not documented Cline CLI values; they exist only in the upstream seven-level vocabulary |
| both routes | upstream case variants and `--reasoning-effort` | accepted upstream | n/a | withheld/invalid | upstream normalization is not a portable input |
| both routes | omitted | n/a | existing behavior | retain existing behavior | `cline --acp` and `cline --json --auto-approve false …` are unchanged |
| any package point other than exact `3.0.55` | all | n/a | n/a | evidence-gated | no retroactive or forward mapping; currentness stays in its standing lane |

No candidate is deliver-now. The deliver-now set is empty for both transports.

## Contract 040 Decision

`ReasoningSelection` needs one exact portable mode that the qualified route
dispatches without clamp, alias, default substitution, or model inference.

`cline.acp` fails at dispatch. There is nothing to dispatch: the exact package
discards the parsed value and always builds `thinking: false`. Binding a flag
the route ignores would claim a capability from argv acceptance alone.

`cline.headless` fails at qualification. The CLI-level application is exact,
but the route selects no provider and no model, and every value's survival
depends on the model that host settings or a catalogue happen to resolve.
Contract 040 forbids clamping to the nearest supported value, replacing an
unsupported value with a provider default, and inferring support from
catalogues. Exact `3.0.55` does the first two itself, and the selected argv
returns no acknowledgement of the value, so Swallowtail could claim dispatch
and nothing beyond it.

`none` is not exact portable `off` on either route. On ACP it is discarded. On
headless it survives only where the unselected model advertises off, and it is
explicitly discarded for Cline-provider Claude Fable models.

Sharing `cline@3.0.55` did not share a claim. The two transports fail for
different reasons and are recorded separately.

## Behavior Revision And Compatibility

No behavior revision is needed or proposed. `cline.acp.stdio-v1` and
`cline.headless.stdio-json-v1` are unchanged, the `cline.package` `3.0.55`
qualified point is unchanged, and the compatibility claim ceiling is unchanged.
Contract 029 currentness stays in its standing lane; this record qualifies no
newer point.

## Promotion

Research 190 promotes no deliver-now Cline thinking row for either route.

Card 117 must not bind `ReasoningSelection`. Card 118 has no dispatch to prove.
The Cline adapter, its fixtures, both route guides, and the unreleased
package API baseline are unchanged.

A later lane may reopen `cline.headless` only with an exact selected-model
route, or an upstream point that applies a named tier without model
entitlement and confirms the applied tier. `cline.acp` may reopen only if a
later package point actually carries a thinking selection into the ACP child.

Cline model, provider, plan mode, compaction, retries, timeout, permissions,
tools, teams, hub, worktree, and session load/resume remain out of scope.
