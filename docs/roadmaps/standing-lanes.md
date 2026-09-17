# Standing Lanes

Status: active
Owner: Tom
Updated: 2026-09-14

These lanes outlive a generation. They are not generation runway goals
and they do not keep a generation open.

## Contract 029 Currentness

Authority: Contract 029 Recurring Currentness Checkpoint and Upgrade
Workflow. Runbook:
[Version Currentness Checkpoint](../guides/version-currentness-checkpoint.md).
Execute through the repo skill `version-currentness`.

Currentness is a persistent serial queue. Continue it after a consumer defect
on an unverified-newer point, after official stables move, or when the operator
asks. It is not CI, a calendar job, or authority to qualify several families
at once.

A checkpoint writes research. It does not change claims and does not
need an active generation.

One-family qualification that changes a claim compiles into the
then-active generation. If none is active, stop and ask. Do not invent a
generation to house currentness.

Completed campaign family: Oh My Pi official npm/GitHub `18.1.22` (g05.079) is
the nineteenth and final family of Tom's authorized Research 308 campaign.
Research 327 re-probed npm and GitHub `18.1.22` (published 2026-09-14) and the
installed `omp/18.1.16`, then reproduced every registry `integrity` and
`shasum`, tarball, `dist/cli.js`, shipped file-count, and GitHub tag-commit for
all 36 published stables from `17.4.0` through `18.1.22` with one
deterministic per-hop shipped-tree inventory and a mutation-sensitive mapped
RPC source ledger. Both later `17.x` hops are additive extensions (`17.4.1`
adds an unmapped background-command callback and an accurate `agentInvoked`
field; `17.4.2` adds optional select `optionDetails` that `docs/rpc.md` says
hosts may ignore), so the retained `17.x` segment extends to
`17.2.9..=17.4.2` on the unchanged `oh-my-pi.rpc-v2-v17.2.9` revision. The
whole `18.x` line compiles as a distinct adapter-private segment
`18.0.0..=18.1.22` on the new `oh-my-pi.rpc-v2-v18.0.0` revision: every mapped
hop is byte-identical, unmapped, additive, client-library, or a resolved
session-scoped persistence change with unchanged `get_state` values.
`rpc-messages.ts`, `rpc-input.ts`, `host-uris.ts`, and `message-framing.ts` are
byte-identical across all 36 points and `docs/rpc.md` is blob `310b4470` from
`v17.4.2` through `v18.1.22`. Contract 029 revises the claim id to
`oh-my-pi.rpc.package-window-2`; the retained `17.x` segment reports
`Deprecated` while staying executable, `18.0.2` and `18.1.7` are explicit
exclusions, the npm-unpublished GitHub tags `17.4.3`/`17.4.4` stay
incompatible, and `18.1.23` stays permitted `UnverifiedNewer`. Research 217
reproduces with the `17.4.2` landing correction. `pi.package` stays separate;
no prompt, login, install, host update, downloaded-artifact execution, or
consumer mutation occurred. This closes the Research 308 campaign.

Completed campaign family: Gemini CLI official npm `0.59.0` (g05.076) is
the sixteenth family of Tom's authorized Research 308 campaign. Research 324
froze official npm and GitHub `0.56.0`, `0.57.0`, `0.58.0`, and `0.59.0`
with tarball, tag, tree, source-archive, and darwin-arm64 digests, and built
one deterministic tagged-source file inventory per point. No changed path lies
under `packages/cli/src/acp/**`; every selected ACP source is byte-identical
with the `@agentclientprotocol/sdk@0.16.1` pin and both selected ACP profiles
unchanged. Every selected stream-json event, terminal record, native exit
code, CLI option, and retention source is unchanged; only `geminiChat.ts`
provider-request retry, empty-part, and abort-rollback internals and the
`resolveWorktreeBaseSha` git helper move. Workspace-trust fail-closing, the
policy safety-checker declaration, macOS Seatbelt sandboxing, and the MCP
OAuth SSRF repair stay unmapped because the selected route sets neither
`GEMINI_RESTRICTED_MODE` nor `GEMINI_CLI_TRUST_WORKSPACE=false` and already
passes `--skip-trust`. Both axes advanced to maintained `0.51.0..=0.59.0`
with their baseline, claim ids, behavior revisions, and `AllowUnverified`
unchanged, and unpublished `0.59.1` stays visible `UnverifiedNewer`. Kimi
installed routes follow.

Completed campaign family: Antigravity official GitHub `1.2.2` (g05.075) is
the fifteenth family of Tom's authorized Research 308 campaign. Research 283
froze `1.1.17..=1.1.26` and stopped at `1.1.22`, where selected headless HTTP
502 failure became provider-managed retry without a published bound or disable
control. Tom ruled on 2026-09-15 that the official release notes are the
behavioural authority for the five further releases through `1.2.2`, and
Research 323 classified each hop per claim: the catalogue claim advances to
maintained `1.1.9..=1.2.2` because no published selected-path change touches
`agy models`, while the headless claim keeps `1.1.9..=1.1.17` because the
`1.1.22` retry stop stands and `1.1.28` plus `1.2.1` broaden provider-managed
retry with no published finite bound or disable control, leaving
`1.1.18..=1.2.2` as the named unqualified gap. Exhaustive binary scanning
stopped at the ruling and collected hashes are retained. Unknown retry stays
incompatible and the gap stays explicit. Gemini followed.

Completed campaign family: Deep Agents official npm `0.1.30` (g05.074) is
the fourteenth family of Tom's authorized Research 308 campaign. Research
322 froze the exact `0.1.25` baseline, all five published stable successors
through `0.1.30`, and the exact-pinned `deepagents` runtime dependency chain
`1.12.4..=1.13.4` as registry-verified tarballs whose baselines reproduce
Research 157 and Research 206. The selected no-extra-argv stdio wire is
byte-stable across the whole window and the sole CLI body change
(`0.1.27..0.1.28`, `ACPFilesystemBackend.read` pagination) is unreachable on
the selected route, so the exact `QualifiedOnly` point advanced to `0.1.30`
with the `deepagents.acp.stdio-v1` behavior revision, empty selected argv,
and no argv change. The selected `deepagents` deltas are additive and
decoder-invisible (builtin `delete` tool, required `write_file` content,
`read_file` pagination footer); the stale ACP registry `0.1.7` entry and
constructor-default `agentInfo.version` `0.0.1` stay outside the claim.
Antigravity followed.

Completed campaign family: Mistral Vibe official GitHub/PyPI `2.25.4`
(g05.073) is the thirteenth family of Tom's authorized Research 308
campaign. Research 321 froze the exact `2.24.2` baseline and all eight
published GitHub successors through `2.25.4` as exact source tarballs with
registry-matched sdist/wheel digests, kept GitHub-only `2.24.4` a named
PyPI packaging gap, and classified every selected input provider-free. The
streaming wire, limit middleware, upstream programmatic tests, public
history schema, builtin `plan` profile, trust and workdir authority,
missing-key failure, and cleanup are byte-stable across every hop, so the
exact `QualifiedOnly` point advanced to `2.25.4` with the
`mistral-vibe.headless.stdio-streaming-v1` behavior revision unchanged and
one mechanical adaptation: the selected argv adds adapter-private
`--legacy-harness` because `2.25.1` resolves the session harness from
flags, the ambient GrowthBook rollout cache, and native-module availability
(bundled in GitHub zips from `2.25.1`, never in PyPI distributions), and
the Unified Harness plus `--smart-approve` stay unmapped. The next family
was Deep Agents.

Completed campaign family: Kiro official manifest `2.21.4` (g05.072) is the
twelfth family of Tom's authorized Research 308 campaign. Research 320 froze
the exact `2.18.1` baseline and all eleven published stable successors
through `2.21.4` as exact official archives with per-hop BUILD-INFO and
executable digests plus read-only-mounted DMG identity, and classified every
selected ACP input provider-free. The selected surface is byte-stable across
every hop (library pins `sacp-11.0.0` and `agent-client-protocol-0.10.4`, argv `kiro-cli acp`, `prompt` field, update kinds, permission kinds, stop
reasons, cleanup), so the exact `QualifiedOnly` point advanced to `2.21.4`
with the `kiro.acp.stdio-v1` behavior revision unchanged. The `_kiro.dev`
extension set and the advertised `session/load` stay unmapped additions, and
Research 251/254's empty deliver-now sets stand. The next family was Mistral
Vibe.

Completed campaign family: Goose official GitHub `v1.50.1` (g05.081) is the
eleventh family of Tom's authorized Research 308 campaign. Research 328
re-probed the official release channel, froze `1.50.1` identity and the
`1.50.0..1.50.1` patch hop, and kept the selected ACP sources byte-identical
apart from an unmapped MCP protocol-version default. The exact `QualifiedOnly`
point advances to `1.50.1` on the new
`goose.acp.stdio-v2.auth-required` behavior revision. The route binds the
typed provider-authentication semantics: a chained ACP `AuthRequired` on
`session/new` or `session/prompt` surfaces as `auth_required`. The old
`1.46.0` untyped error text plus `end_turn` gave a consumer nothing to branch
on, so this adapter-local mapping refines an unclassified failure surface
without adding lifecycle or authority. Research 148's `1.46.0` decoder
specimens and Research 250/253's empty builtin/mode dispositions stand, and
advertised-only `session/delete`,
`recipeParameterScopes`, and thinking-effort menus stay independently gated.
The next family is Kiro.

Stopped campaign family: Qoder official npm `1.1.52` (g05.070) is the tenth
family of Tom's authorized Research 308 campaign. Research 318 froze the exact
`1.1.25` baseline and all 27 published stable successors through `1.1.52` from
the official registry with a complete shipped-tree ledger and a
mutation-sensitive fixture under the adapter, and classified every selected
input provider-free. The exact `QualifiedOnly` point stays at `1.1.25`: the
route's own `--max-turns 8` argv stops being historical inert history at the
exact hop `1.1.29..1.1.30`, where the CLI option gains a numeric argParser, both
headless mode entry points begin forwarding `argv.maxTurns` into the headless
session, and `driveQuery` loses its fixed `1000` fallback, so the argv value
becomes the AgentLoop turn ceiling. That is a selected run-lifecycle and
bounded-limit failure change. Tom ruled on 2026-09-17, having delegated the
rule choice to the Chatterbox, that the route accepts a declared turn bound
but not an inherited accident: `--max-turns 8` becoming the real ceiling means
the qualified claim must state a bound. Reopening the family requires the
adapter to pin an explicit, deliberate `--max-turns` value and to record the
chosen bound and the `error_max_turns` terminal shape in the Qoder prepared
guide; advancing the exact point on the unexamined inherited `8` is not
authorised. Research 151's `1.1.25`
decoder specimens stand, and Research 256's empty skill-visibility disposition
with the g05.039/g05.040 gate stays independent. The next family is Goose.

Completed campaign family: Command Code official npm `1.54.0` (g05.068) is the
ninth family of Tom's authorized Research 308 campaign. Research 317 froze the
installed `1.15.1` baseline and all 67 published stable successors with a
complete shipped-tree ledger: no removals, eight additive bundled references,
and a byte-identical entrypoint; selected invocation, AgentEvent, result,
usage, failure, and local lifecycle surfaces remain compatible. The exact
`QualifiedOnly` point advanced to `1.54.0` with the
`command-code.agent-event-ndjson-v1` behavior revision unchanged. Research 116
and 118 live evidence stays bound to `1.15.1`; authenticated completion/tool/
usage/credit and private continuation are gated pending the separately
authorized exact-`1.54.0` live requalification named in g05.069. The next
family is Qoder.

Completed campaign family: the Grok Build catalogue at official and installed
exact `1.0.30` (g05.067) is the eighth family of Tom's authorized Research
308 campaign. Research 316 reproduced the Research 314 platform and
executable identity for the previous exact `1.0.25` point and every later
published stable through `1.0.30`, and proved the catalogue argv grammar,
authentication preamble, shipped `*`/`-` bullet grammar,
`xai-grok-pager/src/models.rs` module path, and embedded default-model
document identical at every hop. The exact `QualifiedOnly` claim advanced to
`grok-build.catalogue.executable-1-0-30` with the
`grok-build.catalogue.models-text-v1` behavior revision unchanged. The
accepted observation is one prompt-free authenticated `--no-auto-update
models` process on the installed exact `1.0.30` executable that returned
ordered `grok-4.6` default then `grok-4.5` with zero stderr, no prompt,
session, inference, or tool, and joined cleanup. The Grok ACP window and the
`1.0.4`/`1.0.5` registered-tool courier stay independent. The next family is
Command Code.

Completed campaign family: Claude Agent SDK official npm `0.3.270`, carrying
native Claude `2.1.270` (g05.065), is the seventh family of Tom's authorized
Research 308 campaign. Research 315 froze all nine published wrapper points
after exact `0.3.259` (`0.3.260`, `0.3.261`, `0.3.263`, and `0.3.265..=0.3.270`;
`0.3.262` and `0.3.264` are unpublished gaps) with wrapper/native coupling,
tarball digests, native commits, platform payloads, and a complete 15-file
tree inventory; 42 declaration surfaces classified unmapped with the mapped
subset, lifecycle, and credential posture unchanged. The exact one-point
package/native tuple rebound to `0.3.270`/`2.1.270` with behavior revision,
wire, Node `22.23.2`, sidecar source-tag axes, claim ids, and QualifiedOnly
posture unchanged. Research 301 live registered-tool acceptance stays bound
to `0.3.259`/`2.1.259` and does not transfer: the compiled tuple projects
`registered_tools` and `consumer_tool_exchange` unqualified until a separately
authorized live requalification runs on the new tuple. The next family is the
Grok Build catalogue.

Completed campaign family: Grok Build ACP official npm stable `1.0.30`
(g05.064) is the sixth family of Tom's authorized Research 308 campaign.
Research 314 froze all 25 published stables `1.0.6..=1.0.30` after the
`1.0.5` ceiling with verified wrapper/platform integrity, tarball and
decompressed executable digests, a byte-identical mapped ACP literal presence
map, one unread model-document metadata delta, a persistent 62-module mapped
ACP core, and a complete shipped-file inventory. The compatible-extension
claim extends the maintained window `1.0.4..=1.0.30` on
`grok-build.acp-v1.cached-token-model-4-6-v3`; baseline, claim identity,
deprecated `0.2.114..=0.2.117`, gaps, `grok-4.6` binding, and
`AllowUnverified` stay, with alpha `1.0.31` and unpublished `1.0.32` visible
`UnverifiedNewer`. The exact `1.0.25` catalogue claim and the registered-tool
courier on the accepted live versions `1.0.4` and `1.0.5` stay independently
bounded. The next family is the Claude Agent SDK tuple.

Completed campaign family: Ollama official `0.34.0` (g05.063) is the fifth
family of Tom's authorized Research 308 campaign. Research 313 froze all
five published hops after the prior `0.32.15` ceiling with host client
`0.33.3` observed. The compatible-extension-with-stop claim qualifies
`0.33.0` through `0.33.2`; `0.33.3` and `0.34.0` stay permitted
`UnverifiedNewer` on the named strict-decoder cached-count reason with a
decoder-tolerance follow-up, and historical exclusions, claim identity,
behavior revision, and `AllowUnverified` stay. The campaign continued
serially with Grok Build ACP.

Completed campaign family: Cursor Agent official `2026.09.10-fd3934a`
(g05.062) is the fourth family of Tom's authorized Research 308 campaign.
Research 312 froze all three published hops after the prior
`2026.08.11-e8db854` ceiling and host `2026.08.04-aaa8809`. The
compatible-extension claim now qualifies catalogue, ACP, and headless through
`2026.09.10-fd3934a`; calendar dates between the seven exact points stay
incompatible, exact feature-specific sets stay on their probed points, and
historical milestones, claim identities, and `AllowUnverified` stay. The
campaign continued serially with Ollama.

Completed campaign family: Codex official npm and GitHub release `0.154.0`
(g05.061) is the third family of Tom's authorized Research 308 campaign.
Research 311 froze all six published stable hops after the prior `0.152.1`
ceiling and host `0.153.3`. The compatible-extension claim now qualifies
both exec and app-server through `0.154.0`; newly interior unpublished
`0.152.2` is pinned incompatible alongside `0.149.2`, `0.150.2`, and
`0.151.1`, while unpublished `0.154.1` stays permitted `UnverifiedNewer`,
feature-specific exact sets stay on the `0.147.0..=0.149.1` probed points,
and historical windows, claim identities, and `AllowUnverified` stay. The
campaign continued serially with Cursor Agent.

Completed campaign family: Qwen Code official npm and direct GitHub release
`0.23.3` (g05.060) is the second family of Tom's authorized Research 308
campaign. Research 310 froze the four published stable hops after the prior
`0.22.3` ceiling and host `0.21.2`. The compatible-extension claim now
qualifies `0.22.0..=0.23.3`; the exact Plan set remains bounded at `0.22.3`,
while exact `0.21.15` reasoning and budgets, historical gaps, and adjacent
Alibaba/Qwen families stay independently bounded. The next family is Codex.

Completed campaign family: Claude Agent ACP official npm, GitHub, and
ACP-registry stable `0.76.0` is a compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7` through the previous `0.73.0`
ceiling. Research 309 froze identity before the claim and classified all four
published hops `0.74.0`, `0.75.0`, `0.75.1`, and `0.76.0` from a complete dist
inventory; the mode, config-id, elicitation, tools, settings, utils, and
`dist/permissions/**` modules are byte-identical across every hop and the ACP
SDK `1.4.0` and Agent SDK `0.3.257` pins hold. Unpublished `0.58.0`,
`0.73.1`, `0.74.1`, `0.75.2`, and `0.76.1` stay incompatible; unpublished
`0.77.0` stays permitted `UnverifiedNewer`. Host `0.63.0` matches the frozen
`0.70.0` host digest and was observed only. Claude Code, the Claude Agent SDK
sidecar, and the watcher stay untouched. The campaign continues serially with
Qwen Code. No provider prompt, login, install, host update, release, or
consumer mutation occurred.

Current promoted family: Pi RPC `0.85.1` (g05.044). Research 302 froze
identity before the claim. Official npm and GitHub `0.85.1` plus published
`0.85.0` are compatible extensions of
`pi.rpc.strict-lf-v0.84.0-message-update-delta`. Baseline `0.80.10` and
unpublished `0.83.1`/`0.84.5` stay. Unpublished `0.85.2` stays permitted
`UnverifiedNewer`. Host `0.85.1` matches official `dist/bundle/cli.js` and
stays observation-only. `pi.sdk-sidecar` stays exact `0.84.2`. Downloaded
official artifacts were hashed and never executed. No provider prompt, live
session, login, install, host update, or sidecar widening.

Last one-family claim: OpenCode HTTP `1.18.29` (g05.037, cards
135-136). The existing `opencode.server` `surface-19` window extends through
exact npm `1.18.29` as a compatible extension. Baseline `1.14.48` and every
historical gap stay. Unpublished `1.18.30` stays permitted `UnverifiedNewer`.
Host `opencode` was not on `PATH` and was not installed. Downloaded official
artifacts were hashed and never executed. Selected HTTP/SSE route files and
OpenAPI are byte-identical with `1.18.28`. The only implementation-source
change is unmapped Codex OAuth model-id filtering. `AllowUnverified` remains.

Previous one-family claim: Codex exec and app-server `0.152.1` (g05.020, cards
048-049). The shared `codex.cli` window extends through exact npm `0.152.1`
as a compatible extension. Unpublished `0.149.2`, `0.150.2`, and `0.151.1`
stay incompatible. Unpublished `0.152.2` stays permitted `UnverifiedNewer`.
Host `0.150.1` remains inside the qualified range and stays observation-only.
Downloaded official binaries were hashed and never executed; the complete
shipped-tree and source-tree deltas are Guardian, test, and version-bump
bounded. Feature-specific exact sets remain on the `0.147.0..=0.149.1`
probed points. `AllowUnverified` remains.

Earlier one-family claim: Claude Code `2.1.257` (g05.019, cards 046-047),
later raised through official `2.1.270` (g05.058). Headless is
`2.1.220..=2.1.270` and response-only is `2.1.227..=2.1.270`. Unpublished
`2.1.244`, `2.1.249`, hop-skipped `2.1.253` through `2.1.256`, `2.1.262`,
and `2.1.264` stay incompatible. Unpublished `2.1.271` stays permitted
`UnverifiedNewer`. Host `2.1.258` matches official darwin-arm64 and stays
observation-only. Watcher stays exact `2.1.251` behind its mechanism-change
gate. Feature-specific exact sets remain on the `2.1.220..=2.1.241` probed
points. `AllowUnverified` remains.

Earlier one-family claim: Claude Agent ACP `0.73.0` (g05.018, cards
044-045), later raised through official `0.76.0` (g05.059). Maintained v7 is
now `0.66.0..=0.76.0`. Published intermediates `0.71.0`, `0.72.0`, and
`0.73.0` are qualified, as are `0.74.0`, `0.75.0`, `0.75.1`, and `0.76.0`.
Unpublished `0.58.0`, `0.73.1`, `0.74.1`, `0.75.2`, and `0.76.1` stay
incompatible. Unpublished `0.77.0` stays permitted `UnverifiedNewer`. Host
`0.63.0` stays observation-only Qualified Deprecated. `AllowUnverified`
remains.

Earlier one-family claim: Kimi Code installed harness `0.43.0` (g05.016,
cards 041-042; g05.077) plus the A2 ACP cap (g05.017, card 043). Headless v1 is
`0.29.0..=0.32.0` and headless v2 is `0.33.0..=0.43.0`, above which unpublished
`0.43.1` stays permitted `UnverifiedNewer`. ACP is `QualifiedOnly` at exact
`0.28.1` plus `0.29.0..=0.38.0` with exact `0.39.0` and `0.39.1` excluded as
recorded evidence; every point above `0.38.0` fails closed, including the
published `0.40.0..=0.43.0` gap. A shipped-artifact
identity run may reopen planning only if every invocation path fails closed
again for a terminal-less client, or upstream supplies a ProviderEnforced
boundary satisfying Contracts 017/023. The trigger authorizes a fresh
identity/claim decision, never automatic admission and never restoration of
AllowUnverified by itself.
`kimi-code.local-server` is `QualifiedOnly` at exact `0.28.1` plus
`0.29.0..=0.39.1` (g05.078, Research 326): the `0.39.x` Bash workspace
assertion is intact, while the uncontained `0.40.0` `cwd` widening persists
through `0.43.0`, so every point above `0.39.1` fails closed. A later
segment may reopen only on exact restored containment.

Earlier one-family claim: Pi RPC `0.84.4` (g05.015, cards 039-040).
Unpublished `0.83.1` stays incompatible. `pi.sdk-sidecar` stays exact
`0.84.2`. Claude Code headless and response-only later rose through official
`2.1.270` (g05.019 cards 046-047 and g05.058). g05.014 cards 037-038 remain
the `2.1.252` identity/claim. Unpublished `2.1.244`, `2.1.249`, hop-skipped
`2.1.253` through `2.1.256`, `2.1.262`, and `2.1.264` stay incompatible.
Watcher stays exact `2.1.251` behind its mechanism-change gate.
Feature-specific exact sets remain on the `2.1.220..=2.1.241` probed points.
Codex exec and app-server later rose through official `0.152.1` (g05.020,
cards 048-049). g05.013 cards 035-036 remain the `0.152.0` identity/claim.
Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay incompatible.
Feature-specific exact sets remain on the `0.147.0..=0.149.1` probed points.
Qwen headless `0.23.3` is qualified through g05.060 and Research 310; g05.004
cards 012-013 remain the prior `0.22.3` currentness record. Kimi
Code headless agent-core-v2 stream-json first qualified as exact `0.38.0`
under `kimi.headless.stream-json.v2` (g04.064, cards 179-180); g05.016
corrected that revision's baseline to `0.33.0` and extended it to
`0.33.0..=0.39.1`; g05.077 then extended it again to `0.33.0..=0.43.0` with the
selected argv, JSONL grammar, retry record, terminal shape, retention,
cancellation, and cleanup unchanged. The v1 ceiling is `0.32.0`, not the
`0.37.2` g04.064 recorded.

### Latest Completed Family

Claude Agent ACP `0.76.0` is a compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7`, the first family of the
Research 308 campaign. Research 309 and g05.059 freeze identity before the
claim. Host `0.63.0` matches the frozen `0.70.0` host digest and was not
installed, updated, replaced, or executed beyond `--version`. Official npm,
GitHub, and ACP-registry stable is `0.76.0` published
2026-09-09T21:16:32.982Z; published stables above the previous `0.73.0`
ceiling are exactly `0.74.0`, `0.75.0`, `0.75.1`, and `0.76.0`. Complete dist
inventory `0.73.0` (96 files) → `0.74.0` (99) → `0.75.0` (111) → `0.75.1`
(117) → `0.76.0` (123) is frozen with no removals; `dist/index.js`,
`dist/elicitation.js`, `dist/settings.js`, `dist/utils.js`, `dist/tools.js`,
`dist/session-mode.js`, `dist/session-config-ids.js`, and the complete
`dist/permissions/**` tree are byte-identical across every hop, so mode
ids/categories, `plan`/`acceptEdits`, permission option kinds, and the effort
config id are unchanged. The ACP SDK pin stays `1.4.0` and the Agent SDK pin
stays `0.3.257`. The `--hide-claude-auth` guard, the `authStatus` push
extension, the synthetic context-compaction tool call, usage Markdown, AIR
fork metadata, clear-context coordination, and capability-gated recommended
config values stay unmapped with reasons. Unpublished `0.58.0`, `0.73.1`,
`0.74.1`, `0.75.2`, and `0.76.1` stay incompatible; unpublished `0.77.0`
remains permitted `UnverifiedNewer`. Claude Code and the watcher stay
untouched. No provider prompt, live ACP initialize, login, install, host
update, or execution of downloaded official binaries was required.

### Previous Completed Family

Claude Code `2.1.270` is a compatible extension of the separate headless and
response-only stream-JSON axes. Research 307 and g05.058 freeze identity before
the claim. Host `2.1.258` matches the official darwin-arm64 binary and was not
installed, updated, or replaced. Official binaries were hashed and not
executed; mapped-surface evidence was recovered from the embedded
`// @bun @bytecode` source chunks of both platform builds. Official npm and
GitHub stable is `2.1.270` published 2026-09-12T18:52:44.937Z /
2026-09-12T19:45:44Z. All eleven published hops after `2.1.257` were retrieved
and classified; `2.1.262` and `2.1.264` are unpublished. Wrapper files except
`package.json` and `sdk-tools.d.ts` are byte-identical across all twelve
compared versions, and every `sdk-tools.d.ts` delta is SDK tool declaration
content. Host `2.1.258` help equals the frozen `2.1.257` help digest. Selected
mapped stream-JSON flags, format, effort, and permission enumerations, the wire
permission spelling, and the normalized `init`, `stream_event`, `hook_started`,
`result`, and `thinking_tokens` constructions stay. Changelog mapped-keyword
bullets name only unmapped interactive, VSCode, cloud, Remote Control, gateway,
hook, resume, permission-rule, plugin, telemetry, and sandbox surfaces.
Unpublished `2.1.244`, `2.1.249`, hop-skipped `2.1.253` through `2.1.256`,
`2.1.262`, and `2.1.264` remain incompatible; unpublished `2.1.271` remains
permitted `UnverifiedNewer`. Watcher stays exact `2.1.251` and is not
live-ready. No provider prompt, live session, login, install, or host update
was required.

Previous OpenCode HTTP `1.18.30` is a compatible extension of existing
`opencode.http-sse.surface-19`. Research 304 and g05.051 freeze identity before
the claim. Host `opencode` is `1.18.18` on `PATH` and stays observation-only; it
was not installed, updated, or invoked beyond `--version`. Official artifacts
were hashed and never executed. Official npm and GitHub stable is `1.18.30`
published 2026-09-09T03:33:55.588Z / 2026-09-09T03:34:27Z; the only published
stable after `1.18.29` is `1.18.30`. The published GitHub tags diverge, so the
hop is proved from complete repository-tree inventories: 6557 to 6561 files,
four added paths, no removals, and 101 changed files. Selected HTTP/SSE route
files and OpenAPI SHA-256 stay byte-identical. Bedrock model-id resolution,
GitLab reasoning-option shaping, the GPT-6 Astra system prompt, provider SDK
bumps, and the explicit-service-tier patch are unmapped or provider-facing.
Baseline `1.14.48`, claim id, historical gaps, and `AllowUnverified` stay.
Unpublished `1.18.31` remains permitted `UnverifiedNewer`. No provider prompt,
live session, login, install, or host update was required.

Earlier OpenCode HTTP `1.18.29` is a compatible extension of existing
`opencode.http-sse.surface-19`. Research 292 and g05.037 cards 135-136 freeze
identity before the claim. Host `opencode` was not on `PATH` and was not
installed, updated, or replaced. Official artifacts were hashed and never
executed. Official npm and GitHub stable is `1.18.29` published
2026-09-04T23:46:25.653Z / 2026-09-04T23:47:16Z; the only published stable
after `1.18.28` is `1.18.29`. npm `LICENSE`, `bin/opencode.exe`, and
`postinstall.mjs` are byte-identical; only `package.json` changes. Selected
HTTP/SSE route files and OpenAPI SHA-256 stay byte-identical. The only
`packages/opencode/src` change is unmapped Codex OAuth catalogue filtering in
`plugin/openai/codex.ts`. Baseline `1.14.48`, claim id, historical gaps, and
`AllowUnverified` stay. Unpublished `1.18.30` was the `UnverifiedNewer` point
until this claim. No provider prompt, live session, login, install, or host
update was required.

g05.037 cards 135-136 remain the `1.18.29` identity/claim. g05.028 cards
077-078 remain the `1.18.28` identity/claim. Each family adds only the next
published hop.

### Earlier Completed Family

Codex exec and app-server `0.152.1` are compatible extensions of their
existing maintained behaviors. Research 275 and g05.020 cards 048-049 freeze
identity before the claim. Host `0.150.1` keeps its recorded identity and
stays observation-only. Downloaded official binaries were hashed and never
executed. Official npm and GitHub stable is `0.152.1` published
2026-09-01T22:36:50.784Z; the only published stable after `0.152.0` is
`0.152.1`. The complete shipped-tree delta is `package.json`,
`codex-package.json`, and the two rebuilt binaries plus a darwin-only
vendored ripgrep and zsh refresh. The complete GitHub source delta between
the two release tag commits is exactly 12 files: the workspace version bump,
Guardian auto-review/node-REPL policy sources and tests, and test files. No
file feeding a selected surface changed, and every upstream-published schema
file is byte-identical to the frozen `0.152.0` corpus values. Selected
mapped exec flags and app-server methods stay. The new optional Guardian
`AutoReviewMessages.node_repl_policy` stays unmapped. Unpublished `0.149.2`,
`0.150.2`, and `0.151.1` remain incompatible; unpublished `0.152.2` remains
permitted `UnverifiedNewer`. No provider prompt, live session, login,
install, or host update was required.

Research 274 remaining family after Claude Code `2.1.257` was this family
alone and changed no claim.

### Older Completed Family

Claude Code `2.1.257` is a compatible extension of the separate headless and
response-only stream-JSON axes. Research 273 and g05.019 cards 046-047 freeze
identity before the claim; g05.058 later raised the ceiling to `2.1.270`. Host
`2.1.257` matches the official darwin-arm64
package and was not installed, updated, or replaced. Official binaries were
hashed and not executed. Official npm and GitHub stable is `2.1.257`
published 2026-09-01T17:15:33.223Z. The only published stable after
`2.1.252` is `2.1.257`; `2.1.253` through `2.1.256` are unpublished.
Wrapper installer files except `package.json` and `sdk-tools.d.ts` are
byte-identical to `2.1.252`. Official `--help` is not byte-identical to
frozen `2.1.252`; the dump adds `--system-prompt-snapshot` and expands
`--bg` resume wording. Selected mapped stream-JSON flags stay. Changelog
`2.1.257` extras stay unmapped. Unpublished `2.1.244`, `2.1.249`, and
hop-skipped `2.1.253` through `2.1.256` remain incompatible; unpublished
`2.1.258` was permitted `UnverifiedNewer` at identity. Research 274 later
observed official `2.1.258` as visible newer. Watcher stays exact `2.1.251`
and is not live-ready; official `2.1.257` is rejected at both
watcher admission seams. No provider prompt, live session, login, install,
or host update was required.

Research 271 remaining family after Claude Agent ACP `0.73.0` was this
family alone and changed no claim.

### Latest Checkpoint

Research 308 revalidated the operator-named currentness backlog on 2026-09-14
after Claude Code `2.1.270` closed. Tom directed Swallowtail to action all
nineteen families, explicitly including Kimi and Oh My Pi; that direction also
lifts the Gemini deferral and reopens Antigravity. The serial order is Claude
Agent ACP, Qwen, Codex, Cursor, Ollama, Grok ACP, Claude Agent SDK, Grok
catalogue, Command Code, Qoder, Goose, Kiro, Mistral Vibe, Deep Agents,
Antigravity, Gemini, Kimi installed routes, Kimi local server, then Oh My Pi.
Each family gets its own Contract 029 task from fresh canonical main. Shared
matrices and changelog stay serial. Ordinary provider-free recovery is
unbounded by artificial attempt counts. Existing containment rules remain:
Kimi and Antigravity must gain a bounded mechanism or fail closed, and Oh My
Pi `18.x` remains a major-line identity investigation rather than an inferred
extension. No release authority follows.

All nineteen families have now been actioned. Oh My Pi `18.1.22` (g05.079,
Research 327) closed the campaign on 2026-09-15: the npm/GitHub identity
ledger reproduced for all 36 published stables, the retained `17.x` segment
extended to `17.2.9..=17.4.2`, and the admitted `18.0.0..=18.1.22` range landed
on a distinct adapter-private behavior revision. Goose and Qoder keep their
exact `QualifiedOnly` points, with their failure-binding and turn-binding
rulings now recorded above; Gemini's
deferral was lifted for the campaign and its claims advanced through `0.59.0`.
The standing lane stays available for the next checkpoint after official
stables move; it is not a generation goal and does not keep g05 open.

Research 294 revalidated all 41 production solution rows on 2026-09-08. The
partition is 10 unchanged, 8 visible unverified-newer, and 23 record-only.
Cursor Agent `2026.09.02` ranks as the next one-family candidate, but the
checkpoint dispatches nothing and changes no claim, version pin, or route
matrix cell. Existing Gemini, Kimi, Antigravity, watcher, and exact-pin stops
remain. PR 286 merged the research-only record as `2c67f128`.

Research 284 revalidated all 41 production solution rows on 2026-09-04. The
partition is 9 unchanged, 8 visible unverified-newer, 23 record-only, and 1
material candidate. OpenCode HTTP `1.18.28` ranked first: the host is
`1.18.18`, inside the previously qualified `1.14.48..=1.18.20` range. Card 077
admitted all eight published hops as compatible `surface-19`; card 078 raises
the qualified ceiling through `1.18.28`. g05.037 cards 135-136 later raise
that same `surface-19` through official `1.18.29` and leave unpublished
`1.18.30` `UnverifiedNewer` under Contract 029. g05.051 then raises that same
`surface-19` through official `1.18.30` and leaves unpublished `1.18.31`
`UnverifiedNewer`.

Research 276 previously revalidated all 40 production families on 2026-09-02.
Kimi Code npm and GitHub stable moved to `0.40.1`; the
separate `kimi-code.local-server` family is the first post-release candidate,
not an active implementation task, because its AllowUnverified boundary ends
at `0.38.0`, the installed host is qualified at `0.34.0`, and Research 270
recorded the local/web protocol delta. The `0.40.0` Bash tool change removes
the workspace restriction on `cwd`, the same risk class as uncontained local
process authority; the later identity run must investigate it and cannot
assume a compatible extension or qualify it. The installed harness remains
split: headless is separately bounded, while ACP stays QualifiedOnly at
`0.38.0` under the A2 gate. Claude Code `2.1.258`, Antigravity `1.1.24`,
Cursor `2026.08.31-4057e58`, Ollama `0.33.2`, OpenCode `1.18.26`, and Grok
stable `1.0.13` are visible newer observations. PR 182 is closed/unmerged;
its Antigravity branch/evidence is parked post-release and is not merged
qualification. Codex remains closed at qualified `0.152.1`; watcher stays exact
`2.1.251`; Gemini remains deferred. The checkpoint changes no claim and opens
no roadmap task.

### Post-v0.4.0 Queue

g05.021 completed the bounded release-readiness runway and the operator
tagged annotated `v0.4.0` at `56f3913a` on 2026-09-04. The
feature/currentness freeze is lifted. Kimi Code local server ran through
g05.026 card 062, retargeted on 2026-09-04 from `0.40.1` to official latest
`0.41.0` after latest moved mid-run; card 062 stopped with the `0.38.0`
Bash `cwd` restriction removal as a Contract 017/023 authority question before
any segment shape, and it stopped: Research 282 froze `0.41.0` identity and
found the Bash `cwd` change uncontained, so the ceiling stays `0.38.0`.
Antigravity ran through g05.027 card 071 targeting official `1.1.26`; the
closed PR 182 branch was recomputed as evidence input. Card 071 stopped:
every hop `1.1.17..=1.1.26` is frozen, but `1.1.22` retries model-endpoint
HTTP 502 with no published bound or disable control, and Contract 023 keeps
provider-managed retry disabled unless separately accepted. The ceiling stays
`1.1.17` and card 072 is not admitted; Chatterbox recommends keeping that
ceiling until Antigravity exposes a bound. Contract 029's In-Run Latest
Movement rule (accepted 2026-09-04) applies to future lanes.

With both post-release families stopped and Research 276 two days old, the
next currentness step is the all-route checkpoint below.

### Checkpoint Manifest: Research 284

Promoted planning commit: the `main` commit that introduces this section.
This is a standing-lane checkpoint, not a numbered task; it opens no
roadmap and changes no claim.

| Field | Research 284 checkpoint |
| --- | --- |
| Readiness | ready |
| Prerequisites | Research 276, 282, and 283 on `main`; the `version-currentness` skill in checkpoint mode |
| Completion conditions | Research 284 records current official and host observations for all 40 production families, partitions them (unchanged, visible unverified-newer, record-only, material candidate), ranks the next one-family candidate with reasons, and changes no claim; research index line added; docs and Northstar gates green |
| Owned mutable paths | `docs/research/284-*.md`; `docs/research/README.md` (one index line); `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | this file's Latest Checkpoint paragraph, `docs/roadmaps/README.md`, `docs/logs/README.md`; the coordinator edits these at closeout |
| Forbidden paths | every `crates/**` path; every `selection.rs`; matrices; guides; `CHANGELOG.md`; contracts; roadmaps |
| Approved concurrent siblings | g05.009 cards 073 and 065 |
| Serial edges | the ranked candidate becomes a family lane only after Chatterbox compiles it |
| Worker capability class | evidence-first observation worker; registry and release-stream probing; no downloads executed; no provider credentials |
| Acceptance evidence | one observation row per family with source URL and timestamp; host observation without install or update; explicit rank rationale |
| Review oracle | no claim edit; no family treated as qualified from `latest`; Gemini stays deferred; Kimi ACP and local-server and Antigravity stops restated with their reopen conditions |
| Stop conditions | an official channel is unreachable for a family (record the gap, do not guess); a host binary would need execution to observe |
| Escalation owner | operator (Tom) via Chatterbox; coordinator for mechanical blockers |
Claude Code `2.1.258`, Cursor, OpenCode, Ollama, Grok, and other visible newer
observations stay recorded without implementation. Gemini stays deferred,
watcher stays exact `2.1.251`, and `kimi-code.acp` stays QualifiedOnly at the
A2 `0.38.0` cap.

### Earlier Completed Family

Claude Agent ACP `0.73.0` is a compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7`; g05.059 later raised the
same segment through official `0.76.0`. Operator restart after
official latest moved during the unmerged `0.72.0` family. Research 272 and
g05.018 cards 044-045 freeze identity before the claim. Host `0.63.0`
matches the frozen `0.70.0` host digest and was not installed, updated,
replaced, or executed beyond `--version`. Official npm and GitHub stable at
identity was `0.73.0` published 2026-09-01T20:27:53.428Z; published stables
above the previous `0.70.0` ceiling are exactly `0.71.0`, `0.72.0`, and
`0.73.0`. Mapped `dist/index.js`, `dist/elicitation.js`, `dist/lib.js`,
`dist/settings.js`, and `dist/utils.js` are byte-identical to `0.70.0`.
Complete dist inventory `0.70.0` (33 files) → `0.71.0` (96) → `0.72.0`
(96) → `0.73.0` (96, only `package.json` changed) is frozen; every
`dist/**` file is byte-identical `0.72.0` to `0.73.0`. Remaining named
files stay unmapped with reason. `#1004` keeps mapped mode ids/categories
and `plan`/`acceptEdits`; `#1045` leaves steering unmapped and the
permission callback observable contract unchanged. `0.72.0` effort, result
attribution, PostModelSwitch, and PreModelSwitch stay classified;
Swallowtail still fails closed on explicit `set_config_option` + confirm.
The `0.73.0` Agent SDK pin `0.3.252`→`0.3.257` is unmapped. Five new
emitted update kinds stay unmapped. Unpublished `0.58.0` stays
incompatible; unpublished `0.74.0` remains permitted `UnverifiedNewer`.
Claude Code and the watcher stay untouched. No provider prompt, live ACP
initialize, login, install, host update, or execution of downloaded
official binaries was required.

Research 271 selected this family alone after the Kimi A2 cap when latest
was `0.72.0` and changed no claim.

### Earlier Completed Family

Kimi Code installed harness `0.39.1` produced a split outcome, and a
same-family claim correction. Research 270 and g05.016 cards 041-042 freeze
identity before the claim. Host `0.34.0` is byte-identical to the official
`0.34.0` darwin-arm64 extracted artifact and was not installed, updated,
replaced, or executed. Official npm and GitHub stable is `0.39.1`; published
stables above the previous `0.38.0` ceiling are exactly `0.39.0` and `0.39.1`.

Re-testing the routing premise inherited from Research 179 and 211 moved the
boundary. `experimental-v2.ts` redefines `isKimiV2Enabled()` at `0.33.0` from
`KIMI_CODE_EXPERIMENTAL_FLAG` truthy to `KIMI_CODE_LEGACY_FLAG` not truthy, so
the default `kimi -p` engine is agent-core-v2 from `0.33.0`, not `0.38.0`, and
the same release makes naked `kimi acp` run `packages/acp-server`. Production
had claimed `0.33.0..=0.37.2` as qualified `kimi.headless.stream-json.v1`
while those releases emit the `system.version` preamble the v1 decoder
rejects. `kimi.headless.stream-json.v1` corrects down to `0.29.0..=0.32.0`;
`kimi.headless.stream-json.v2` corrects down and extends to
`0.33.0..=0.39.1`; host `0.34.0` reclassifies to qualified Maintained v2.
Unpublished `0.39.2` remains permitted `UnverifiedNewer` on that axis.

`kimi-code.acp` stops at `0.38.0` under `QualifiedOnly`. From `0.39.0` the
agent-core-v2 ACP terminal runner replaces two fail-closed errors with a local
host-process spawn in the leased working resource, and the route always
advertises `terminal: false`, so that branch always applies. The containment
trace found none: `HarnessIsolation::AmbientHost` makes no isolation claim,
Contract 015 denies filesystem containment from process ownership and treats a
terminal request from a terminal-less client as scope-stopping, and no adapter
or runtime control mediates the spawn. Exact `0.39.0` and `0.39.1` stay
excluded as recorded evidence; unpublished `0.38.1`, unpublished `0.39.2`, and
farther `0.40.x` fail closed with them. No new ACP behavior revision was
created.

`kimi-code.local-server` stays a separate family and is unchanged; its `kimi
web` deltas are recorded as observations only. No provider prompt, model
request, authentication, catalogue or session work, live session, install,
host update, or execution of downloaded official binaries was required.

Operator A2 is recorded. A shipped-artifact identity run may reopen planning
only if every invocation path fails closed again for a terminal-less client,
or upstream supplies a ProviderEnforced boundary satisfying Contracts 017/023.
The trigger authorizes a fresh identity/claim decision, never automatic
admission and never restoration of AllowUnverified by itself. The lane must
not adopt a growing exclusion set as a substitute for the posture.

Research 325 (g05.077) then ran the artifact-level trigger through official
npm and GitHub `0.43.0`. `acpTerminalRunner.ts` is git blob `9016d48b` at every
point from `0.39.1` through `0.43.0`, and the bundled `AcpProcessService` is
the frozen `7c58e045` digest at all six points, so ACP stays capped at `0.38.0`
with the published `0.40.0..=0.43.0` gap posture-rejected and the exclusion set
unchanged. The same run extended `kimi.headless.stream-json.v2` to
`0.33.0..=0.43.0`; unpublished `0.43.1` is the first later stable. The separate
`kimi-code.local-server` family stays Research 282's and did not move.

Research 271 revalidated all 40 families after the Kimi A2 cap. Headless
`0.39.1` was on-ceiling then; g05.077 and Research 325 later extended it to
`0.43.0`. `kimi-code.acp` stays `QualifiedOnly` at `0.38.0` and
is not reopened. Claude Agent ACP `0.72.0` was the sole next-family
candidate. Operator restart compiled official `0.73.0` in g05.018.

### Earlier Completed Family

Pi RPC `0.84.4` is a compatible extension of
`pi.rpc.strict-lf-v0.84.0-message-update-delta`. Research 268 and g05.015
cards 039-040 freeze identity before the claim. Current host `0.83.0`
matches the previously frozen `0.84.2` host digest. Official npm and GitHub
stable is `0.84.4`. Mapped `jsonl.ts`, `session-cwd.ts`, `json-event.ts`, and
`args.ts` are byte-identical to `0.84.3`. `clear_queue` stays unmapped.
Unpublished `0.83.1` remains incompatible; unpublished `0.84.5` remains
permitted `UnverifiedNewer`. `pi.sdk-sidecar` stays exact `0.84.2`. No
provider prompt, live session, login, install, host update, or execution of
downloaded official binaries was required.

Research 269 revalidated all 40 families after Pi RPC `0.84.4` and selected
Kimi Code installed harness `0.39.1` as the sole next-family candidate. That
checkpoint changed no claim; g05.016 then compiled the qualification.

Claude Code `2.1.252` is a compatible extension of the separate headless and
response-only stream-JSON axes. Research 266 and g05.014 cards 037-038 freeze
identity before the claim. Current host `2.1.251` matches the previously frozen
official darwin-arm64 package. Official npm and GitHub stable is `2.1.252`.
Official extracted help is byte-identical to frozen `2.1.251`. Unpublished
`2.1.244` and `2.1.249` remain incompatible; unpublished `2.1.253` remains
permitted `UnverifiedNewer`. Watcher stays exact `2.1.251` and is not
live-ready. No provider prompt, live session, login, install, or host update
was required.

Research 267 revalidated all 40 families after Claude Code `2.1.252` and
selected Pi `0.84.4` as the sole next-family candidate. That checkpoint
changed no claim.

### Do Not

- treat currentness as a reason to keep a generation active
- bulk-bump from registry `latest`
- leave the current host or official stable `UnverifiedNewer` without a
  named incompatible reason

## Next Task

Implement the active task named by the
[roadmaps front door](README.md#next-task). This file records standing lanes;
the sole actionable pointer stays in the front door. The authorized Research
308 campaign completed at g05.079 (Oh My Pi `18.1.22`), so no currentness
family remains ready; the next move is the operator conversation recorded by
the front door, not a new lane.
