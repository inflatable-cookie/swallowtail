# 278 Claude Agent SDK Route Evidence

Status: evidence gate; contract proposal in
`../triage/2026-09-02-claude-agent-sdk-route-contract-gate.md`
Owner: Tom
Date: 2026-09-02
Card: g05 batch 053
Authority: Research 277; official Anthropic Help Center and npm registry;
the frozen `@anthropic-ai/claude-agent-sdk` `0.3.258` tarball

## Question

Can Swallowtail add a distinct `claude-agent.sdk` route that drives the
official TypeScript Claude Agent SDK through a bounded Node sidecar on one
user's own Claude subscription, without Swallowtail possessing the credential
and without an unjoined process surviving session cleanup?

Answer: yes for the credential and route-identity halves, which current
contracts already fix. The lifecycle half needs a named contract extension
and new sidecar behavior: the real topology is a descendant tree, not one
child, and the SDK provides no joined stop at all — only a bounded wait
attempt whose outcome is discarded. No stop condition fired.

## Method

Compared official npm registry metadata, the extracted `0.3.258` package
tree, the shipped TypeScript declarations, the shipped `manifest.json`, the
shipped `sdk.mjs` implementation, and the public GitHub repository against
Contracts 010, 014, 017, 019, 023, 029, 038, 041, 047, 049, 051, and 057.

Retrieval: npm metadata and tarball `2026-09-02T18:29:34Z`; Help Center
article rechecked immediately before freeze in the same window; GitHub
repository and tag metadata in the same window.

Nothing downloaded was executed. No `claude` binary was fetched, installed,
or run. No provider turn, login, OAuth flow, token read, or authenticated
probe. No installed host was mutated. Artifacts stayed in `/tmp` and are not
committed. Declarations, shipped artifact, and runtime behavior are kept as
three separate evidence classes throughout; nothing below is a runtime claim.
§6 reads shipped `sdk.mjs` source because the declarations and the shipped
implementation disagree there; reading is not executing, and a read of minified
shipped source is still artifact evidence, not runtime proof.

## 1 Subscription authority

Source:
<https://support.claude.com/en/articles/15036540-use-the-claude-agent-sdk-with-your-claude-plan>

Rechecked `2026-09-02`, immediately before artifact freeze. Article title
"Use the Claude Agent SDK with your Claude plan"; article-stated update
`June 16, 2026`, covering a change previously announced for `June 15, 2026`.

Exact applicable statement, as the article leads:

> We're pausing the changes to Claude Agent SDK usage described below. For
> now, nothing has changed: Claude Agent SDK, `claude -p`, and third-party app
> usage still draw from your subscription's usage limits.

Announced-change boundary:

> When we have an update, we'll share it before anything takes effect.

- **Usage source:** the user's own subscription usage limits.
- **Eligible shape:** the paused credit scheme named Pro, Max, Team, and
  Enterprise. Those plan names describe the *paused* credit allocation, not
  the preserved subscription-draw statement, which the article states without
  a plan restriction. Do not restate the plan list as the eligibility rule for
  the current preserved behavior.
- **Third-party apps are explicitly named**, so this is first-party authority
  for a third-party application, not an inference from silence.

This is current but provisional. It is not an entitlement and does not
survive an Anthropic announcement.

**Currentness trigger.** Re-read this article and re-freeze this section
before any of: publishing a `claude-agent.sdk` support claim, a release that
first ships the route, or an Agent SDK family Contract 029 checkpoint. A
changed statement is a stop, not a downgrade. Because the policy is prose on a
Help Center page rather than a versioned artifact, this trigger is the only
currentness mechanism available; there is no digest to pin.

## 2 Artifact identity

Selected artifact: official npm `@anthropic-ai/claude-agent-sdk` `0.3.258`,
`dist-tags.latest` and `dist-tags.next` at retrieval.

| Surface | Value |
| --- | --- |
| Version | `0.3.258`, published `2026-09-01T22:24:14.573Z` |
| Tarball SHA-256 | `656cf237bc567cb172a007a0fd5b3958cf960d154c03ab390a755d2c3bdbb398` |
| npm integrity | `sha512-RxJ5fSPCGCxX5qO/b4IPXhldvtLHeYBAzTUJ4eOzO+gTrepZQSDmwSlQD6nnoEquKGJzOMHCjhdEtBfDjbDWUg==` |
| npm shasum | `7d44d24c5a09e55537286b6546dc02d3e1d9b88a` |
| File count / unpacked | 15 / 5 019 497 bytes |
| Declared Node range | `>=18.0.0` |
| `claudeCodeVersion` | `2.1.258` |
| Publisher | `wolffiex <wolffiex@anthropic.com>`, npm `11.17.0`, Node `24.19.0` |
| Registry signature | one npm key `SHA256:DhQ8wR5APBvFHLF/+Tc+AYvPOdTpcIDqOhxsBHRwC7U`, two signatures |

Total published versions: 285. Immediately preceding stables `0.3.257`
(`2026-09-01T17:15:32.460Z`) and `0.3.252` (`2026-08-31T17:08:17.209Z`).
Unpublished numbers inside `0.3.142..0.3.258` are exactly `0.3.151`,
`0.3.155`, `0.3.164`, `0.3.171`, `0.3.180`, `0.3.184`, `0.3.188`, `0.3.189`,
`0.3.192`, `0.3.194`, `0.3.230`, `0.3.244`, `0.3.249`, `0.3.253`, `0.3.254`,
`0.3.255`, and `0.3.256`. The release cadence is roughly daily, which is a
qualification-cost fact for Contract 029, not a defect.

### Complete package tree

All 15 files, SHA-256:

| File | SHA-256 | Bytes |
| --- | --- | --- |
| `LICENSE.md` | `8ce94b9478bb9868f9641f818e06cd722fbe55d4c22e2d2ed11971b20146173a` | 147 |
| `README.md` | `a923405f92c474ca40c62d0b5ffb2897aba56f779a7ea36807555b1553d0cdab` | 3 173 |
| `agentSdkTypes.d.ts` | `1005d810b1bec12c856632bb7d54087af7c516251a4b557eb05ae8d474fc5a93` | 25 |
| `bridge.d.ts` | `3aa177a4c5859dbacd768cead5e660b148e854b834d5bf430e0c489339785295` | 17 300 |
| `bridge.mjs` | `8f9274d517e9dcd3bc1a7d490b58ea3deb88d69fb97159453da4044953c43350` | 1 464 115 |
| `browser-sdk.d.ts` | `a8005d33d20f9b7ed540ad62653a94a5cea1a0678027a0198a2ee6bb4ac13bf1` | 4 246 |
| `browser-sdk.js` | `d3bd16de4cc9084c290381046afc4f87d91d98bde12ac36055bda19f762a682e` | 1 416 386 |
| `extractFromBunfs.d.ts` | `eedc9d6a3adb2e0f12984ddf3d2cc3a72f56906b4a4069660bd08fd36c3d2984` | 63 |
| `extractFromBunfs.js` | `1286171f98b1147a4ed46d48296a1ab8ee4f79cee2d893ace5210ebe361d8c27` | 6 606 |
| `manifest.json` | `ec53b2ce3d9f95d2c94c07aacf9202e6df9f79c695b71a1af2b4491081fba60f` | 1 778 |
| `manifest.zst.json` | `5b37e3c2629043628582167e690753ddf395753a25f4bebd411dad7b54b00b31` | 2 082 |
| `package.json` | `a8fb007bc424b11eca6a4f046f5e6bdf078c9f032a4c6f133f9bc5297e686a14` | 2 380 |
| `sdk-tools.d.ts` | `a8bb537bb1624e9e68d5aa7c620260027278a9f83ce81943906a9485b06d7c9d` | 165 764 |
| `sdk.d.ts` | `b209ed293b63bb92bde59a49fe8a6e3b06f92afe0d1d4987c937a3cfea90b6dc` | 422 172 |
| `sdk.mjs` | `4d9286bd9ca8f802e27c9be2cfa2e0769502dfabb693a6e3d16b62e4fbe3e69a` | 1 513 260 |

### Source authority

**The public GitHub repository is not a source authority for this artifact.**
Three independent facts:

1. npm metadata for `0.3.258` carries **no `gitHead`**. The frozen
   `@agentclientprotocol/claude-agent-acp` family (Research 272) does carry
   one, so this is a real difference in available evidence, not a gap in this
   run's method.
2. `_resolved` is
   `/home/runner/work/claude-cli-internal/claude-cli-internal/staged-npm/anthropic-ai-claude-agent-sdk-0.3.258.tgz`
   — the tarball is staged from a private monorepo, not built from the public
   repository.
3. The public repository `anthropics/claude-agent-sdk-typescript` at tag
   `v0.3.258` contains no SDK source. Its complete tree is
   `.claude/`, `.github/workflows/`, `CHANGELOG.md`, `LICENSE.md`,
   `README.md`, `examples/session-stores/{postgres,redis,s3,shared}`, and
   `scripts/`. Repository size 273 KB; no license metadata; `pushed_at`
   `2026-09-01T22:33:18Z`.

Consequence: the npm tarball digest is the **only** artifact identity for this
family. Do not corroborate a future version against a GitHub tag or diff, and
do not treat the `CHANGELOG.md` in that repository as a shipped-behavior
oracle. This narrows what a Contract 029 checkpoint for this family can prove
compared with the ACP family, and the checkpoint must say so.

### License

- Shipped `LICENSE.md`, in full: `© Anthropic PBC. All rights reserved. Use is
  subject to the Legal Agreements outlined here:
  https://code.claude.com/docs/en/legal-and-compliance.`
- npm `license` field: `SEE LICENSE IN README.md`. The shipped `README.md`
  contains **no license section** — its only legal text points at the
  Commercial Terms of Service, Privacy Policy, and data-usage policy. The
  declared pointer and the shipped file therefore disagree.

This is proprietary, all-rights-reserved, incorporating terms by reference. It
is not a license conflict for Swallowtail, because Contract 019 already
requires that *the application* provisions the runtime, sidecar entry point,
and SDK dependency. Swallowtail must not vendor, redistribute, mirror, or pin
the SDK or the platform binaries. Recorded as an evidence discrepancy so a
later reader does not resolve the npm field to an open-source license.

## 3 Runtime topology

The npm package is a **wrapper**, not the agent. The agent is a native
executable delivered by eight exact-pinned platform packages:

`@anthropic-ai/claude-agent-sdk-{linux,darwin,win32}-{x64,arm64}` plus
`linux-x64-musl` and `linux-arm64-musl`, each pinned to `0.3.258` in
`optionalDependencies`.

Shipped `manifest.json` describes them:

- `version` `2.1.258`; `commit` `b3cd543a1f6fcdf4d8fabc0f5e5538d2ee7f38e1`;
  `buildDate` `2026-09-01T22:02:56Z`
- `manifestSignatureEnforcement: "flag"`
- eight platform entries, each a `claude` / `claude.exe` binary with a SHA-256
  and a size between 199 027 600 and 218 507 936 bytes; `darwin-arm64` is
  `b63136194160791c27cfa7b0403060d85eb0752991625fde8c09f9acacb17c78`
- `sdkCompat.harnessSchema: 1`
- `sdkCompat.testedWrapperVersions`: exactly `0.3.217` through `0.3.227`

Peer dependencies, which the application must satisfy:
`@anthropic-ai/sdk >=0.93.0`, `@modelcontextprotocol/sdk ^1.29.0`,
`zod ^4.0.0`.

Two findings follow.

**The shipped compatibility declaration excludes the shipping wrapper.**
`testedWrapperVersions` tops out at `0.3.227` while the wrapper carrying it is
`0.3.258`. Whatever this field means upstream, it cannot be read as "this
wrapper is tested against this binary." It is a stale or differently-scoped
declaration, and it is the clearest single instance of the declaration-versus-
runtime split this card exists to police.

**The route is two processes deep.** Rust → Node sidecar → native `claude`.
The `extract` subpath (`extractFromBunfs`) confirms the binary is a Bun
single-file executable. Contract 019 was written for one sidecar process; §6
names the gap.

## 4 Public API inventory

Source: shipped `sdk.d.ts` (422 172 bytes) — a declaration inventory, not a
behavior claim. 245 exported symbols: 17 functions, 2 classes, 7 interfaces,
and the remainder types and constants.

Functions: `query`, `startup`, `tool`, `createSdkMcpServer`, `resolveSettings`,
`filterEscalatingDefaultMode`, `foldSessionSummary`, `importSessionToStore`,
`listSessions`, `getSessionInfo`, `getSessionMessages`, `listSubagents`,
`getSubagentMessages`, `forkSession`, `renameSession`, `tagSession`,
`deleteSession`.

Interfaces and classes: `Query`, `WarmQuery`, `Transport`, `SpawnOptions`,
`SpawnedProcess`, `Settings`, `HookCallbackMatcher`, `InMemorySessionStore`,
`AbortError`.

`Query` extends `AsyncGenerator<SDKMessage, void>` and declares 28 methods.
`Options` declares 65 keys.

### Ledger against the Research 277 product target

`Present` = declared in `sdk.d.ts`. `Conditional` = declared with a stated
precondition, capability gate, or instability marker. `Missing` = no
declaration. `Undocumented` = the declaration exists but its runtime contract
is not stated, or the declaration is visibly incomplete.

| Target | Class | Exact declaration |
| --- | --- | --- |
| Persistent streaming session | Present | `query({prompt: AsyncIterable<SDKUserMessage>})`; `Query.streamInput`; `Options.includePartialMessages` |
| Read/write tools | Present | `Options.tools`, `allowedTools`, `disallowedTools`, `toolAliases`, `toolConfig`, `additionalDirectories` |
| `canUseTool` | Conditional | `Options.canUseTool`; every `Query` control method is declared "only supported when streaming input/output is used" |
| Permission mode change | Present | `Query.setPermissionMode`; `PermissionMode` = `default`, `acceptEdits`, `bypassPermissions`, `plan`, `dontAsk`, `auto` |
| Per-MCP permission override | Conditional | `Query.setMcpPermissionModeOverride`; declared tighten-only, accepts `default`/`auto`/`null`, applies only where the session mode already auto-allows |
| Interrupt | Conditional | `Query.interrupt()` resolves a receipt **only** on CLIs advertising `interrupt_receipt_v1`; older CLIs resolve `undefined`. `cancel_queued` needs `interrupt_cancel_queued_v1` |
| Model control | Present | `Query.setModel`; `Options.model`, `fallbackModel`; `Query.supportedModels` |
| Effort control | Present | `Options.effort`; `EffortLevel` = `low`, `medium`, `high`, `xhigh`, `max`; `max` is session-scoped and never persisted |
| Thinking control | Present | `Options.thinking` (`adaptive`/`enabled`/`disabled`); `Query.setMaxThinkingTokens` is **deprecated** in favour of it |
| Resume | Present | `Options.resume`, `continue`, `sessionId`, `resumeSessionAt`, `resumeDropsTurn`, `loadTimeoutMs` |
| Fork | Present | `Options.forkSession`; `forkSession()`; `ForkSessionResult` |
| MCP | Present | `Options.mcpServers`, `strictMcpConfig`; `Query.setMcpServers`, `mcpServerStatus`, `reconnectMcpServer`, `toggleMcpServer`; stdio/SSE/HTTP/in-process configs |
| Checkpoint / rewind | Conditional | `Query.rewindFiles`; requires `Options.enableFileCheckpointing` |
| Hooks | Present | `Options.hooks`; `HOOK_EVENTS` has exactly 33 members; `HookPermissionDecision` = `allow`/`deny`/`ask`/`defer` |
| Subagents | Present | `Options.agents`; `Query.supportedAgents`; `listSubagents`, `getSubagentMessages`; `forwardSubagentText`; `SubagentStart`/`SubagentStop` hooks |
| Plugins | Present | `Options.plugins`; `Query.reloadPlugins`; `SdkPluginConfig` |
| Commands | Present | `Query.supportedCommands`; `SlashCommand`; `SDKSystemMessage.slash_commands` and `terminal_slash_commands` |
| Skills | Present | `Options.skills`; `Query.reloadSkills` |
| Account state | Present | `Query.accountInfo()` → `AccountInfo`; also on `SDKControlInitializeResponse.account` |
| Usage state | Conditional | `Query.usage_EXPERIMENTAL_MAY_CHANGE_DO_NOT_RELY_ON_THIS_API_YET()`; self-declared unstable, may change or be removed in any release, method name will change |
| Context usage | Present | `Query.getContextUsage({detail})` |
| Authentication readiness | Present | `AccountInfo.apiProvider`/`subscriptionType`; `SDKSystemMessage.apiKeySource`; `SDKAuthStatusMessage` |
| Sign-in / login | **Missing** | no exported login, logout, OAuth, or device-flow function. See §5 |
| Background tasks | Present | `Query.backgroundTasks`, `stopTask`; `SDKBackgroundTasksChangedMessage` |
| Bash / terminal | Present as a tool name only | admission needs Contract 023 process authority and Contract 041 mediation; declaration alone is not admission |

Two honest limits on this inventory:

- **The declaration is visibly incomplete.** `sdk.d.ts` contains blank runs
  inside `Query` (between `seedReadState` and `reconnectMcpServer`, and again
  before `setMcpServers`) and inside `SDKControlInitializeResponse`. Members
  were stripped before publication. The public control surface is therefore a
  *subset* of the real one, and "not in `sdk.d.ts`" does not prove "not in the
  protocol." All `Missing` rows above mean *not publicly declared*.
- **Feature detection is runtime-only.** `SDKSystemMessage.capabilities?:
  string[]` is documented as the open set "so SDK consumers can feature-detect
  instead of version-sniffing." Named capabilities observed in declarations:
  `interrupt_receipt_v1`, `interrupt_cancel_queued_v1`. No declared capability
  may be claimed as supported without observing it in a real `system/init`.

## 5 Credential non-custody

### Flow

1. The user runs the official Claude Code login out of band. Swallowtail does
   not perform, wrap, or drive it. The SDK exposes no login function.
2. Credentials live in the official Claude credential store, reachable only by
   the native `claude` binary.
3. The Node sidecar calls `query()` with an explicit `Options.env`. It passes
   no credential.
4. The native binary authenticates itself and makes provider requests.
   Swallowtail never sees a request header, token, or refresh.
5. Swallowtail observes **typed readiness only** and maps it to Contract 057
   `Authenticated Subject` and Contract 047 posture.

### Readiness observations, and only these

| Observation | Type | Content |
| --- | --- | --- |
| `AccountInfo.apiProvider` | `'firstParty' \| 'bedrock' \| 'vertex' \| 'foundry' \| 'anthropicAws' \| 'anthropicGoogleCloud' \| 'mantle' \| 'gateway'` | subscription route requires `firstParty`; declared "Anthropic OAuth login only applies when `firstParty`" |
| `AccountInfo.subscriptionType` | `string?` | plan label |
| `AccountInfo.email` / `.organization` | `string?` | Contract 057 restricted, redacted by default, never a routing key |
| `AccountInfo.tokenSource` / `.apiKeySource` | `string?` | source **names** |
| `SDKSystemMessage.apiKeySource` | `ApiKeySource` | closed set: `ANTHROPIC_API_KEY`, `apiKeyHelper`, `/login managed key`, `none`, `user`, `project`, `org`, `temporary`, `oauth` |
| `SDKAuthStatusMessage` | `{isAuthenticating, output: string[], error?}` | readiness edge |

`ApiKeySource` is a set of nine **provenance labels**. Not one of its members
is a secret value. `oauth` is the expected value for the subscription route.

### Negative searches over the public declarations

`sdk.d.ts`, 422 172 bytes, searched for
`accessToken|refreshToken|access_token|refresh_token|bearer|Bearer|oauthToken|authToken|credentialsJson|clientSecret`:
**three hits, all prose, none a declared value.** Two are the identical doc
comment defining `ApiKeySource` provenance labels (lines 125, 5062); one is
the MCP `headers` doc showing `"Authorization": "Bear…"` env-substitution
syntax (line 6064). No exported type, function, parameter, or return in the
default entry point carries token material.

### The withheld subpaths — this is the real risk, and it is exact

Non-custody is **not** a property of the package. It is a property of which
entry point is imported. Two subpaths take raw credentials:

- `@anthropic-ai/claude-agent-sdk/bridge` declares
  `createCodeSession(baseUrl, accessToken: string, …)` and
  `fetchRemoteCredentials(sessionId, baseUrl, accessToken: string, timeoutMs,
  trustedDeviceToken?)`, returning
  `RemoteCredentials = {worker_jwt: string, api_base_url, expires_in,
  worker_epoch}`. That is a minted bearer token in process memory.
- `@anthropic-ai/claude-agent-sdk/browser` declares
  `OAuthCredential = {type: 'oauth', token: string}` and
  `AuthMessage = {type: 'auth', credential: OAuthCredential}`, carried on
  `WebSocketOptions.authMessage`.

Both are remote/hosted Claude session paths and both are outside this route.

**Rule.** The `claude-agent.sdk` route imports the `.` entry point
(`sdk.mjs` / `sdk.d.ts`) only. Importing `/bridge` or `/browser`, or the
`bridge.mjs` / `browser-sdk.js` files directly, is a contract violation.

**Falsifier.** Grep the sidecar source and its lockfile for
`claude-agent-sdk/bridge`, `claude-agent-sdk/browser`, `bridge.mjs`,
`browser-sdk.js`, `fetchRemoteCredentials`, `createCodeSession`,
`worker_jwt`, and `OAuthCredential`. Any hit fails the gate. This is a
mechanical CI check, not a review judgement.

### Three further prohibitions

- **`Settings.apiKeyHelper`** is declared "Path to a script that outputs
  authentication values." Swallowtail must never set it, nor
  `awsAuthRefresh`, nor `gcpAuthRefresh`. Setting it would make Swallowtail a
  credential producer.
- **`Options.env` must always be set explicitly.** Declared: "When omitted,
  the subprocess inherits `process.env`." Silent inheritance is already
  forbidden by Contract 019 §Explicit SDK Configuration, and inheriting
  `ANTHROPIC_API_KEY` would also silently switch the access profile away from
  the subscription. The route sets `env` on every launch and never forwards
  `ANTHROPIC_API_KEY`.
- **`SDKAuthStatusMessage.output: string[]`** is the login flow's own text.
  Treat it as opaque and unlogged: map to a typed readiness state, never
  persist or forward the strings.

### Contract fit

This posture needs no new credential contract. Contract 010 §Credential
Service already admits "delegated authentication owned by a harness, SDK,
cloud environment, or credential helper" that "exposes no secret." Contract
017 §Delegated Authentication And Sign-In already makes an advertised login a
supported action, not permission to execute. Contract 057 §Sign-In Loop
already enumerates "delegated CLI login," and §Authenticated Subject already
governs the email/plan disclosure. Contract 047 §Credential And Availability
Posture takes the readiness result.

**No token-custody stop fires.** No secret-bearing fixture or log is
introduced by this run; no fixture was written at all.

## 6 Sidecar protocol and lifecycle

Contract 019 §Foreign-Language SDK Sidecars already governs this route. What
follows maps its clauses onto exact declarations and names the two places it
does not reach.

### Launch authority and binding

| Clause | Mechanism |
| --- | --- |
| Application provisions runtime and SDK | `engines.node >=18.0.0`; the three peer dependencies; the platform package. Swallowtail never installs, upgrades, or repairs them |
| Explicit runtime | `Options.executable: 'bun' \| 'deno' \| 'node'` — pin `node`; never rely on the declared auto-detection |
| Explicit binary | `Options.pathToClaudeCodeExecutable` — host-approved absolute path, verified against the `manifest.json` platform checksum before launch |
| Explicit cwd | `Options.cwd`; without it the SDK defaults to `process.cwd()` |
| Explicit environment | `Options.env`, always set, replacing rather than inheriting |
| Reachable roots | `Options.additionalDirectories` |

### Ambient suppression

Contract 019 requires suppressing ambient settings, skills, prompts, context,
model aliases, and discovery. The exact levers, with their declared defaults:

- `Options.settingSources` — "When omitted, all sources are loaded (matches
  CLI defaults). Pass `[]` to disable filesystem settings (SDK isolation
  mode)." The route passes `[]` unless a host authority admits a source.
  Note the declared coupling: `'project'` is required to load `CLAUDE.md`, so
  isolation and project instructions cannot both be had by default.
- `Options.skills` — "omitted (default): no SDK auto-configuration. The CLI's
  own defaults still apply, so this is **not** 'skills off.'" Omission is
  therefore not suppression; the route must pass an explicit list.
- `Options.plugins`, `mcpServers`, `strictMcpConfig`, `agents`,
  `systemPrompt`, `managedSettings` — all set explicitly.
- `Options.persistSession` (`@default true`) writes transcripts to
  `~/.claude/projects/`. The route sets it deliberately; it is a Contract 017
  durable-state decision, not a default to inherit.

Declared but **withheld**: `Options.sessionStore` (`@alpha`, dual-write
transcript mirroring) and `Options.sandbox`. The SDK's own sandbox settings
are not a substitute for Contract 023 native containment and are not evidence
of it.

### Framing and backpressure

The sidecar wire is private to the driver, per Contract 019. Nothing needs
inventing: the SDK's own transport shape is the model. `Transport` declares
`write`, `close`, `isReady`, `readMessages(): AsyncGenerator<StdoutMessage>`,
`endInput`, optional `expectControlResponse(requestId)`, optional
`waitForExit()`, and optional `[Symbol.dispose]`.

Backpressure is real and asymmetric: `Query` is an `AsyncGenerator`, so
consumer pull rate governs the read side. The Rust wire must apply the same
discipline — bounded channels, no unbounded buffering of `SDKMessage` — or the
sidecar becomes the buffer for a stalled Rust consumer.

Callbacks that cross the wire and must be correlated and bounded:
`canUseTool`, `hooks`, `onElicitation`, `onUserDialog`, `stderr`, and SDK MCP
tool handlers.

### Cancellation and close — the SDK offers no joined stop

This is the lifecycle core. The declarations read as though a bounded join
exists; the shipped implementation is weaker, and the difference is the whole
reason this section exists. Evidence class here is **shipped artifact**:
`sdk.mjs`, SHA-256 `4d9286bd9ca8f802e27c9be2cfa2e0769502dfabb693a6e3d16b62e4fbe3e69a`,
read but never executed.

**What the declarations say.**

- `Query.close(): void` — "forcefully ends the query, cleaning up all
  resources including … the CLI subprocess." Returns `void`. Not awaitable.
- `Query.return()` / `AsyncDisposable` — runs `performCleanup()`, documented
  to await `Transport.waitForExit()` "(bounded)" so cleanup does not "resolve
  while the child is still draining the stdin EOF that `close()` just sent."

**What the shipped code does.** `performCleanup` ends with:

```js
if (this.transport.waitForExit) {
  let t = new AbortController;
  try { await Promise.race([this.transport.waitForExit(), Lr(2000, t.signal)]) }
  catch {}
  finally { t.abort() }
}
```

Three properties, each independently disqualifying:

1. It is a **race against a 2 000 ms timer**, not a wait for exit.
2. The `catch {}` **swallows** the outcome. `waitForExit()` is written to
   *reject* on non-zero exit or signal termination — that rejection is
   discarded.
3. **No result is captured.** Nothing downstream can distinguish "the child
   exited" from "two seconds elapsed" from "the child exited with an error."

So `Query.return()` can resolve, reporting nothing, while the native child is
still alive. **The SDK provides a bounded wait attempt, not a joined stop.**
Neither close path is a join, and the swallowed timeout can never be the proof
that a process stopped.

The SDK's own escalation is best-effort too. `ProcessTransport.close()`
schedules SIGTERM after the same 2 000 ms grace and SIGKILL 5 000 ms later
(on Windows, SIGKILL only), and a module-level registry sends SIGTERM to
tracked children on Node's `exit` event. **Every one of these timers is
`.unref()`'d**, so none holds the event loop open: if the sidecar exits or is
killed first, the escalation never runs. All of it targets the direct child
only — nothing in the SDK reaches the native binary's own descendants.

**Therefore the joined stop is sidecar work, not an SDK guarantee.** To close
a session the route must, in the sidecar, retain a process handle it can join
independently of the SDK, and return an explicit close state over the private
wire. That state is three-valued and must never collapse:

| Close state | Meaning |
| --- | --- |
| `graceful` | the process exited on its own after stdin EOF, observed |
| `escalated` | exit observed, but only after host-owned termination |
| `unconfirmed` | no exit was observed; the process may still be running |

`unconfirmed` is a cleanup failure under Contract 017 ("provider completion
never hides cleanup degradation or failure"), not a slow success. Close order:
`interrupt()` if a turn is live → end input → await the sidecar's own join to
its declared bound → on expiry escalate through host termination authority →
re-join → report one of the three states → release provider state,
credentials, and host resources in Contract 017 owner order.

`SpawnOptions.signal` is a **forwarded** signal owned by `ProcessTransport`,
not `Options.abortController.signal`; it fires only after stdin EOF plus the
same ~2 s grace. It is therefore also unusable as a stop primitive.

### Crash, disconnect, and resume

- `Options.stderr` is the only declared stderr channel; `debug`/`debugFile`
  are opt-in and belong under Contract 053 debug observation, never on by
  default.
- `SpawnedProcess` declares `on('exit')`, `once`, `off`, `kill(signal)`,
  `killed`, `exitCode`, `signalCode?`. The built-in local spawn delivers
  `exit` only after stderr closes; **custom spawners emit plain process exit**
  — so a Swallowtail-supplied spawner loses the stderr-tail guarantee and must
  reconstruct it. Shipped `sdk.mjs` confirms both halves: the built-in spawn
  remaps `exit` to an internal `sdk-exit-after-stderr-drained` event with a
  200 ms drain grace, and setting `spawnClaudeCodeProcess` also suppresses the
  SDK's default `--debug-file` argument. Both are costs of that mechanism, not
  of the route.
- `Query.reinitialize()` is the declared transport-gap recovery: it re-sends
  `initialize`, redelivers blocked `can_use_tool` / `request_user_dialog`
  requests, and re-registers hooks. Its declared cost is explicit — callbacks
  must be **idempotent per `request_id`**, and "expect one denied-then-retried
  tool call or one prompt to re-send if the call races an unanswered hook."
  Contract 041 admission must therefore be idempotent, not merely correlated.
- `SDKBackgroundTasksChangedMessage` is declared **per-process**: "nothing is
  emitted at startup, so consumers must reset to the empty set whenever the
  session's CLI process (re)starts." Restart resets background-task state.
- Resume binding is `Options.resume` + `cwd`. A resumed session that binds a
  different cwd or a different account is the Review Oracle counterexample:
  the route re-reads `SDKSystemMessage.cwd` and `AccountInfo` after resume and
  fails closed on mismatch rather than trusting the resume id.

### Failure model

Declared vocabulary to map onto Contract 051:

- `SDKAssistantMessageError` — `authentication_failed`, `oauth_org_not_allowed`,
  `account_on_hold`, `billing_error`, `rate_limit`, `overloaded`,
  `invalid_request`, `model_not_found`, and further members.
- `TerminalReason` — `blocking_limit`, `rapid_refill_breaker`,
  `prompt_too_long`, `image_error`, `model_error`, `api_error`,
  `malformed_tool_use_exhausted`, `aborted_streaming`, and further members.
- `USAGE_LIMIT_ERROR_PREFIXES`, `USAGE_WARNING_PREFIXES`,
  `USAGE_TRANSITION_PREFIXES`, `ORG_POLICY_LIMIT_PREFIXES` — **prefix-matched
  English strings**, several marked `@alpha`. Usable as presentation hints
  only. They must not become a classification mechanism; Contract 051 origin
  and kind come from typed fields. Research 277 requires provider usage limits
  to stay visible and binding, and the typed rate-limit path
  (`SDKRateLimitEvent`, `SDKRateLimitInfo`) is that path.

### The gap in Contract 019, and the obligation it creates

**The contract gap.** Contract 019 says cancellation and close "join the
sidecar process" — one process, no bound, no escalation path, no reporting
duty. The real topology is a **descendant tree**: Rust → Node sidecar →
native `claude` → whatever that binary spawns, including Bash tool
subprocesses. The grandchild holds provider state and the credential reach.
The clause as written is satisfied by joining the Node process alone while
the rest of the tree survives, which is precisely the card 053 Review Oracle
counterexample. Contract 017's "provider completion never hides cleanup
degradation or failure" establishes a reporting duty but says nothing about
what a join must prove or what to do when it cannot.

The durable fix is an invariant about tree ownership and join outcome, not a
selection of any particular provider callback. It is stated in the contract
gate.

**The implementation obligation, which is not a contract matter.** Because
the SDK exposes no joined stop, the route cannot satisfy that invariant by
calling the SDK correctly. New sidecar behavior is required: retain a process
handle that can be joined independently of the SDK's own cleanup, and return
the three-valued `graceful` / `escalated` / `unconfirmed` close state over the
private wire. This belongs to the implementation card.

`Options.spawnClaudeCodeProcess` is **one** route-local way to obtain that
handle — the callback runs inside Node and returns a Node `SpawnedProcess`,
so what it yields is a sidecar-held handle that the sidecar must then report
over the wire. It does not hand Rust a PID, and nothing in the current wire
does. Its costs are recorded in §8 and the contract gate. A host-created POSIX
process group or a Windows job object that captures descendants is another
route, and has the advantage of covering the tree rather than one child.
Choosing between them is implementation work; both must satisfy the same
invariant.

Everything else in §6 is already fixed by Contracts 010, 017, 019, 023, 041,
and 049.

## 7 Route identity

Proposed new route id: **`claude-agent.sdk`**. Additive. It aliases nothing.

Existing Claude identities in the repository, unchanged by this research:

| Route | Transport | Current claim state |
| --- | --- | --- |
| `claude-agent.acp` | ACP v1 stdio via `@agentclientprotocol/claude-agent-acp` | `claude-agent.acp.window-2`, baseline `0.53.0`, `0.58.0` excluded, latest qualified `0.73.0` |
| `claude-code.headless` | `claude` CLI stream-json stdio | `claude-code.headless.window-1`, `stream-json.v1` |
| `claude-code.response-only` | `claude` CLI stream-json, response-only | `claude-code.response-only.window-1` |
| `claude-agent.sdk` | Node sidecar over a private wire, driving the official SDK, which drives the native `claude` | proposed; no claim in this research |

### Why it cannot reuse an existing identity

- **Not `claude-agent.acp`.** Different wire (private sidecar protocol versus
  ACP v1 JSON-RPC), different package (`@anthropic-ai/claude-agent-sdk` versus
  `@agentclientprotocol/claude-agent-acp`), different versioning axis, and a
  strictly larger control surface. The ACP bridge *depends on* the Agent SDK —
  Research 272 records its pin moving `0.3.252` → `0.3.257` — but Research 272
  explicitly classifies that pin as unmapped, "Swallowtail maps ACP stdio, not
  the Agent SDK package." Reusing the identity would retroactively make that
  correct classification wrong.
- **Not `claude-code.headless` or `.response-only`.** Those drive the `claude`
  CLI's stream-json interface directly and are versioned on the Claude Code
  axis (`2.1.257`). The SDK route reaches the same binary only through the
  wrapper, and is versioned on the SDK axis (`0.3.258`). Contract 029
  §Separate Version Axes requires them separate. Note the axes are *coupled
  but not equal*: `0.3.258` declares `claudeCodeVersion: 2.1.258`, so a Claude
  Code qualification does not transfer to the SDK route or vice versa.

### Version axes for the new route

Contract 029 requires each axis observed separately:

1. SDK wrapper package version — `0.3.258`.
2. Native binary version — `manifest.json` `2.1.258`, commit
   `b3cd543a1f6fcdf4d8fabc0f5e5538d2ee7f38e1`.
3. Sidecar source-tagged revision and launch recipe — Swallowtail-owned.
4. Sidecar wire and behavior revision — Swallowtail-owned.
5. Node runtime — application-provisioned, `>=18.0.0`.
6. Runtime-advertised `system/init` `capabilities` — the only axis that is
   behavior rather than declaration.

### Vocabulary partition

**Portable candidates** — a second provider already proves each of these in
Swallowtail, so they belong to existing contracts, not to a Claude-local
vocabulary: session new/resume/fork/close; turn cancel; permission mode;
model and effort selection; tool admission callbacks; MCP server sets;
usage and rate-limit observation; typed failure classification; account
readiness posture.

**Route-local, held provider-specific until a second provider proves a
portable abstraction** — per Research 277: the 33-member `HOOK_EVENTS` set;
plugins; skills; `rewindFiles` file checkpointing; `backgroundTasks` /
`stopTask`; `setMcpPermissionModeOverride`; `applyFlagSettings` /
`updateSettings`; `getContextUsage` category breakdown; `seedReadState`;
`readFile`; output styles; `SdkBeta`.

Naming any of these as shared vocabulary is orchestrator integration work
after card 054, and is deliberately not decided here.

## 8 Smallest implementation foundation

Not implemented in this card. Recorded so the split is visible and later
parity layers are not smuggled into layer 1.

**Layer 1 — the smallest thing that proves the invariant.** Sidecar launch
with explicit runtime, binary, cwd, and env; `query()` with streaming input;
`SDKSystemMessage` and `capabilities` capture; `AccountInfo` readiness
observation; `canUseTool` admission; `interrupt()`; a sidecar-owned joined
close returning `graceful` / `escalated` / `unconfirmed`, built on an
independently joinable process handle rather than on SDK cleanup; typed
failure mapping. No tools beyond read-only. No Bash.

The close path is layer 1, not a later hardening step. It is the only thing
that proves the Review Oracle invariant, and §6 shows the SDK does not supply
it.

**Layer 2.** Permission mode changes; model/effort/thinking; resume and fork
with cwd and account rebinding checks; `supportedCommands` / `supportedModels`
/ `supportedAgents`.

**Layer 3.** MCP server sets and status; hooks; subagents; background tasks;
context and usage observation.

**Layer 4, each requiring its own evidence.** Bash and terminal, gated on
Contract 023 process authority and Contract 041 mediation — capability
advertisement is not admission. File checkpointing and rewind. Plugins and
skills. Anything marked `@alpha` or `EXPERIMENTAL` upstream.

## 9 Falsification

Every exact, universal, and negative claim above, with the test that would
break it.

| Claim | Falsifier | Result |
| --- | --- | --- |
| `0.3.258` is `latest` at retrieval | `npm view … dist-tags` | held; also `next` |
| Tarball digest | `shasum -a 256` on the fetched tarball | `656cf237…b398` |
| Package holds exactly 15 files | full extract and enumerate | held; all 15 digested above |
| No token material in the default entry point | 10-pattern grep over `sdk.d.ts` | 3 hits, all prose; enumerated in §5 |
| No public login function | enumerate all 17 exported functions | held; none authenticate |
| `/bridge` and `/browser` take raw credentials | read `bridge.d.ts`, `browser-sdk.d.ts` | held; `accessToken`, `worker_jwt`, `OAuthCredential` |
| GitHub is not a source authority | `gitHead` absent; `_resolved`; tag tree | held; three independent facts |
| npm license field disagrees with shipped file | read both | held; README has no license section |
| `testedWrapperVersions` excludes the shipping wrapper | read `manifest.json` | held; tops out at `0.3.227` |
| `Query.close()` is not a join | read the declaration | held; returns `void` |
| The SDK offers a joined stop | read shipped `performCleanup` in `sdk.mjs` | **refuted**; a `Promise.race` against a 2 000 ms timer, in `try{}catch{}`, discarding the outcome |
| A caller can tell exit from timeout | inspect what `performCleanup` returns | **refuted**; no result is captured or propagated |
| The SDK escalates reliably | read `ProcessTransport.close()` timers | **refuted**; SIGTERM/SIGKILL timers and the exit-registry are all `.unref()`'d and target only the direct child |
| `Options.env` inherits when omitted | read the declaration | held; stated verbatim |
| `settingSources` loads everything when omitted | read the declaration | held |
| Omitting `skills` is not "skills off" | read the declaration | held; stated verbatim |
| The declaration is complete | inspect `Query` and `SDKControlInitializeResponse` | **refuted**; blank runs prove stripped members |
| Subscription use is still supported | re-read the article | held at `2026-09-02` |

### Adversarial counterexamples, and the answer to each

1. *A token appears in a sidecar request, fixture, or log.* Only reachable by
   importing `/bridge` or `/browser`, or by setting `apiKeyHelper`. Both are
   prohibited in §5 with a mechanical grep falsifier. No fixture was written.
2. *A detached process survives Rust session cleanup.* Open on current
   evidence, and the sharpest finding in this research. Neither SDK close path
   is a join: `Query.close()` returns `void`, and `performCleanup` races
   `waitForExit()` against a 2 000 ms timer inside `try{}catch{}` without
   capturing the outcome, so it can resolve silently while the native child
   lives. The SDK's own SIGTERM/SIGKILL escalation is `.unref()`'d and reaches
   only the direct child, never the descendant tree. Closing this needs both a
   Contract 019 invariant about tree ownership and join outcome, and new
   sidecar behavior returning an explicit three-valued close state. Both are in
   §6 and the contract gate.
3. *A resumed session binds a different cwd or user.* `Options.resume` does
   not re-verify either. Answer: re-read `SDKSystemMessage.cwd` and
   `AccountInfo` after resume and fail closed. Stated in §6.
4. *An SDK declaration is treated as runtime proof.* The strongest instance is
   `testedWrapperVersions`, which is stale in the shipping artifact. The rule
   is that only `system/init` `capabilities` may be treated as runtime
   evidence — and this research makes no runtime claim at all.
5. *The route silently becomes an API-key route.* `Options.env` inheritance
   would forward `ANTHROPIC_API_KEY`. Answer: always set `env`; require
   `apiProvider === 'firstParty'`; check `apiKeySource`.

## 10 Withheld

No production Rust or TypeScript, no manifest change, no package pin, no
route claim, no compatibility claim, no fixture, no matrix, no workflow edit.
No `claude-agent.sdk` claim id or behavior revision is minted here.

Withheld upstream surfaces, with reason:

- `/bridge` and `/browser` subpaths — credential custody
- `Options.sandbox` — not evidence of Contract 023 containment
- `Options.sessionStore` — `@alpha` transcript dual-write
- `usage_EXPERIMENTAL_MAY_CHANGE_DO_NOT_RELY_ON_THIS_API_YET` — upstream says
  it may be removed in any release and will be renamed
- the four usage/policy prefix constants as a classification mechanism —
  English prefix matching, several `@alpha`
- Bash and terminal execution — needs Contract 023 and 041 evidence
- `Options.debug` / `debugFile` by default — Contract 053 opt-in only
- every `Missing` row in §4 — not publicly declared, and §4 states that this
  does not prove absence from the protocol

## Decision

The `claude-agent.sdk` route is **admissible as an additive route** on current
evidence. Policy authority is current and provisional with a named recheck
trigger. Artifact identity is exact and frozen on the npm digest alone.
Credential non-custody is proved for the `.` entry point and reduces to one
mechanically checkable import rule. Route identity cannot be flattened onto
any existing Claude route.

One Contract 019 lifecycle gap is real and is not fixed by current contracts:
close is specified against a single sidecar process, with no bound, no
escalation, and no required join outcome, while the route owns a descendant
tree. The proposed provider-neutral invariant is in
`../triage/2026-09-02-claude-agent-sdk-route-contract-gate.md`.

That gap cannot be closed by using the SDK correctly. The SDK supplies a
bounded wait attempt whose outcome is discarded, not a joined stop, so the
route carries a matching implementation obligation: an independently joinable
process handle held in the sidecar and an explicit `graceful` / `escalated` /
`unconfirmed` close state on the private wire. That obligation is layer 1.

This is not an honest stop: nothing here blocks on an operator product, API,
persistence, or security choice. It is a completed evidence gate with one
scoped Contract 019 amendment and one Contract 029 family note pending, plus a
route-local implementation obligation that needs no decision.
