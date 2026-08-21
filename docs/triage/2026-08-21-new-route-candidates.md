# 2026-08-21 New Route Candidates

Status: draft
Owner: Tom

## Context

Research triage of official 2026 harnesses, agent APIs, and similar
machine-facing surfaces that are **not** Swallowtail production routes.
The question is whether any of them could expand the production matrix as
**new families**. This is not an implementation card, not a matrix edit,
and not a product change.

Recommendations below are not decisions. Tom's local orchestrator
schedules any later inventory or implementation. Do not flatten families
that share a vendor. A Yes on one route never promotes another. Swallowtail
is a library, not a server. Hosted URL-open plus loopback stays parked
until a named production route requires it. Do not reclassify installed
or delegated logins as that first-proof. Generation-controls forbids
translating arbitrary UI labels into provider strings.

Method: repo inventory first (route matrix, feature CSV, guide map,
prepared guides, `crates/swallowtail-adapter-*`, Research 143/153/158/170/171,
backlog). Then official vendor docs, CLIs, and the 2026-08-21 ACP
registry snapshot. No executable was installed. No provider account,
login, prompt, or live session was used. Observed versions are not
qualified claims. If a surface cannot be confirmed from an official
source, this note says so.

ACP registry membership remains discovery only. It is not a compatibility
claim, version range, or implementation approval.

## Existing Swallowtail Coverage

The provider route matrix is 47 production routes. Remote ACP is a
transport, not a 48th row. Every listed family already has an adapter
crate except where noted.

**Production installed / attached harnesses.** Codex exec and app-server;
Claude Agent ACP plus Claude Code headless and response-only; Cursor
catalogue, ACP, and headless; Gemini CLI ACP and headless; Grok Build
ACP; Kimi Code ACP, headless, and local-server; Muse; Command Code;
Cline ACP and headless; Goose ACP; Kiro ACP; Deep Agents ACP; Copilot
CLI ACP; Mistral Vibe headless; Qoder headless; DeepSeek Harness
JSON-RPC and local-server; ZCode app-server; Oh My Pi RPC; Pi RPC;
Qwen headless; OpenCode HTTP; Antigravity catalogue and headless.

**Production hosted / SDK / local.** Anthropic Messages and Managed
Agent; Kimi Platform chat; DeepSeek continuation; Alibaba conversations;
OpenAI background Responses and Realtime; xAI Responses WebSocket;
Gemini Live; Bedrock runtime and catalogue; Ollama attached; llama.cpp
attached and owned.

**Adapters that exist without a production row.**
`swallowtail-adapter-openhands` is deferred. Live HTTP/WebSocket stays
unwired. Research 155 and
`docs/roadmaps/backlog/openhands-agent-server-production-wiring.md`.

**Already inventoried, not production.** Research 143 selected the
primary/secondary waves. Research 153/158 closed the watchlist. Backlog
holds Aider headless, Kiro headless, OpenHands production wiring, and
hosted interactive OAuth. Python Kimi CLI headless is declined. Research
170/171 inventory **existing** production rows for Contract 057 addable
descriptors. They are not new families. Do not re-propose them here.

**Parked / deferred that are not new routes.** Hosted URL-open OAuth.
Gemini CLI range requalification is currentness, not a new family.
ACP siblings of already-selected print or HTTP routes (Vibe ACP, Qoder
ACP, OpenCode ACP, Qwen ACP / `qwen serve`) stay unflattened.

Do not recommend adding any production row. If a surface is already
deferred, research-only, or parked, the notes below mark it
**already inventoried** unless official status has clearly changed.

## Per-candidate notes

Count for this note: **43 named surfaces**. **24 already inventoried.**
**19 genuinely new** (official, not production, not previously inventoried
as this surface). Three seed surfaces were checked and dropped. Registry
leftovers are one already-inventoried bucket, not fourteen new families.

### Already inventoried

#### OpenHands Agent Server

- **Official name / CLI or API.** OpenHands Agent Server.
  `python -m openhands.agent_server --host 127.0.0.1 --port 8000`.
  Health `GET /health`. Conversations over HTTP plus WebSocket.
  Optional `X-Session-API-Key` / `OH_SESSION_API_KEYS_*`.
  Official: [Agent Server](https://docs.openhands.dev/sdk/arch/agent-server).
- **Vendor.** OpenHands / All Hands. Do not flatten onto OpenCode,
  Kimi local-server, or Contract 035 remote ACP.
- **Composer flags.** Host, port, optional session API key, workspace
  root, conversation id. Docker sandbox, hosted runtime API, and Agent
  Canvas stay unmapped.
- **Adjacent Swallowtail.** Adapter crate `swallowtail-adapter-openhands`.
  No production row. No prepared guide. Research 153–155. Backlog
  `openhands-agent-server-production-wiring.md`.
- **Gap.** Package exists. `start_run` fail-closes. Live loopback
  conversation is unwired. Host Python must be `>=3.12`.
- **Composer-relevant.** Yes, after a production row exists. Unique
  loopback ownership and optional session key.
- **Named incompatible reason.** Library, not a server. Do not mint
  `OH_SECRET_KEY` or log in. Hosted OAuth is not this proof. A Yes on
  OpenCode HTTP does not promote OpenHands.
- **Recommendation.** Already inventoried. Keep deferred until live
  HTTP/WebSocket evidence exists. Not a new-family candidate.

#### Aider headless

- **Official name / CLI or API.** Aider. `aider --message` /
  `--message-file`. Optional `--yes`. Auto-commits by default.
  Official: [scripting](https://aider.chat/docs/scripting.html).
  PyPI `aider-chat`. Python `Coder` API is unsupported.
- **Vendor.** Aider / Paul Gauthier. Do not flatten onto ACP or a
  generic print route.
- **Composer flags.** Message, yes/auto-commit, model, git worktree.
- **Adjacent Swallowtail.** None. Backlog `aider-headless-route.md`.
  Research 143/153.
- **Gap.** Text/Git mutation, no structured event protocol. Last
  observed PyPI `0.86.2` (2026-02-12). Official status has not changed.
- **Composer-relevant.** Weak. Composer would need Git-mutation
  controls that no current print route owns.
- **Named incompatible reason.** Do not pretend this is structured ACP
  or a typed structured-run.
- **Recommendation.** Already inventoried. Stay deferred until official
  JSON/NDJSON or an equivalent bounded event surface exists.

#### Kiro headless

- **Official name / CLI or API.** Kiro CLI. `kiro-cli chat --no-interactive`
  plus `KIRO_API_KEY`. Trust flags / `permissions.yaml` depending on
  docs generation. Official: [headless](https://kiro.dev/docs/cli/headless/).
- **Vendor.** Kiro / Amazon. Do not flatten onto `kiro.acp` or
  `--cloud`.
- **Composer flags.** Model, trust/permissions, API key vs local
  account, `--cloud` (stay unmapped).
- **Adjacent Swallowtail.** Production `kiro.acp`
  (`swallowtail-adapter-kiro`, `kiro-acp-prepared-integration.md`).
  Backlog `kiro-headless-route.md`. Research 153.
- **Gap.** ACP was selected as the first Kiro route. Headless is
  text-to-stdout. Official status has not changed.
- **Composer-relevant.** Yes, as a sibling print route, if admitted.
- **Named incompatible reason.** A Yes on `kiro.acp` never promotes
  headless. Do not flatten.
- **Recommendation.** Already inventoried. Stay deferred.

#### Crush

- **Official name / CLI or API.** Charm Crush. Official `crush run`
  is non-interactive text. Official `crush acp` is still PR `#2450`,
  not a released wire (still open as of this note). Community
  `willbnu/crush-acp` wraps `crush run`.
  Official: [Charm Crush](https://github.com/charmbracelet/crush).
- **Vendor.** Charmbracelet. Do not wrap the community adapter.
- **Composer flags.** Prompt, model, session. Structured events are
  unofficial.
- **Adjacent Swallowtail.** None. Research 158: defer.
- **Gap.** Official ACP unreleased. Official status has not changed.
- **Composer-relevant.** Not until Charm ships maintained `crush acp`
  or structured `crush run` events.
- **Named incompatible reason.** Do not wrap `crush-acp`.
- **Recommendation.** Already inventoried. Wait for official ACP or
  structured events.

#### Continue CLI

- **Official name / CLI or API.** Continue CLI `cn`. Headless
  `cn -p` / `--format json`. Also `cn serve`, `cn login`,
  `CONTINUE_API_KEY`. Official: [CLI](https://docs.continue.dev/guides/cli).
  npm `@continuedev/cli` last observed `1.5.47` (2026-06-18).
- **Vendor.** Continue Dev. Do not flatten onto Cline or OpenCode.
- **Composer flags.** `-p`, `--config`, `--resume`, `--auto`,
  `--readonly`, `--allow` / `--ask` / `--exclude`, `--model`, `--mcp`.
  Browser login vs API key.
- **Adjacent Swallowtail.** None. Research 158: defer.
- **Gap.** Real headless JSON. Overlaps existing print routes.
  Account/platform coupling. No first-party ACP. Official status has
  not changed.
- **Composer-relevant.** Yes, if a later identity card proves a
  distinct attached-server or print shape.
- **Named incompatible reason.** Browser login is not the hosted-OAuth
  first-proof. Do not reclassify `cn login` as that.
- **Recommendation.** Already inventoried. Stay deferred until
  first-party ACP or a distinct attached-server identity is justified.

#### Auggie CLI

- **Official name / CLI or API.** Augment Code Auggie.
  `auggie --acp`, `auggie --print`, `auggie --ask`, `auggie login`.
  Official: [ACP](https://docs.augmentcode.com/cli/acp/agent),
  [reference](https://docs.augmentcode.com/cli/reference).
  Registry `auggie` `0.35.0`, npx `@augmentcode/auggie@0.35.0 --acp`.
- **Vendor.** Augment Code. Do not flatten onto Cursor or Copilot.
- **Composer flags.** `--acp`, `--print`, `--quiet`, `--ask`,
  `--output-format json`, `--queue`, model, auth.
- **Adjacent Swallowtail.** None. Research 158 named first-party lead.
- **Gap.** First-party ACP exists. No Swallowtail identity corpus.
  Official docs are stronger than the 2026-08-19 snapshot; the surface
  is the same.
- **Composer-relevant.** Yes.
- **Named incompatible reason.** Installed/delegated Augment login is
  not hosted OAuth first-proof.
- **Recommendation.** Already inventoried. Worth a later identity
  inventory card after operator review. Not automatically addable.

#### Devin CLI ACP

- **Official name / CLI or API.** Devin CLI. `devin acp`,
  `devin auth login`. Credentials from `WINDSURF_API_KEY` or stored
  login. Flags `--agent-type`, `--model` / `DEVIN_MODEL`.
  Official: [commands](https://docs.devin.ai/cli/reference/commands).
  Registry `devin`.
- **Vendor.** Cognition. Do not flatten onto Windsurf Cascade or
  Cursor. Cognition now owns Windsurf; keep Devin CLI and Cascade
  separate.
- **Composer flags.** Agent type, model, workspace add/remove,
  `/ask` `/plan` slash commands advertised over ACP.
- **Adjacent Swallowtail.** None. Research 158 named first-party lead.
- **Gap.** Official first-party ACP is now documented. Still no
  Swallowtail identity corpus. Status strengthened, surface already
  inventoried.
- **Composer-relevant.** Yes.
- **Named incompatible reason.** `devin auth login` / browser
  authenticate is not hosted OAuth first-proof unless a named
  production route requires URL-open plus loopback.
- **Recommendation.** Already inventoried. Later identity card after
  operator review. Do not treat registry presence as admission.

#### Factory Droid

- **Official name / CLI or API.** Factory Droid CLI. Interactive
  `droid`. Headless `droid exec`. ACP
  `droid exec --output-format acp`. Auth `FACTORY_API_KEY` or
  `droid` login. Official:
  [IDE integrations](https://docs.factory.ai/ide-integrations).
  Registry `factory-droid`.
- **Vendor.** Factory AI. Do not flatten onto Amp, Codex, or Cursor.
- **Composer flags.** `exec`, `--output-format` (`text` / `json` /
  `stream-json` / `acp`), session resume, model, API key vs login.
- **Adjacent Swallowtail.** None. Research 158 named first-party lead.
- **Gap.** Official docs now name native ACP. Still no identity
  corpus. Surface already inventoried.
- **Composer-relevant.** Yes. Unique output-format and Factory auth.
- **Named incompatible reason.** Do not flatten ACP-daemon exec onto
  a generic print route. Factory login is not hosted OAuth first-proof.
- **Recommendation.** Already inventoried. Later identity card after
  operator review. ACP and `droid exec` print stay separate if either
  is admitted.

#### Junie CLI

- **Official name / CLI or API.** JetBrains Junie.
  Install `curl -fsSL https://junie.jetbrains.com/install.sh | bash`
  or npm `@jetbrains/junie`. Auth: JetBrains Account OAuth, Junie API
  key, or BYOK. Official: [Junie](https://junie.jetbrains.com/),
  [repo](https://github.com/jetbrains/junie). Registry `junie`.
- **Vendor.** JetBrains. Do not flatten onto Copilot or a generic IDE
  agent.
- **Composer flags.** Channel (`--eap` / `--nightly` / `--release`),
  auth method, model/BYOK provider.
- **Adjacent Swallowtail.** None. Research 158 named first-party lead.
- **Gap.** Official CLI exists. No Swallowtail corpus. JetBrains
  Account OAuth would hit the parked hosted-OAuth gate if the first
  row needs URL-open plus loopback.
- **Composer-relevant.** Yes.
- **Named incompatible reason.** Hosted OAuth stays parked. Prefer a
  Junie API-key or BYOK profile if a later card starts, unless the
  operator names OAuth as the proof.
- **Recommendation.** Already inventoried. Later identity card after
  operator review.

#### CodeBuddy Code and Cortex Code

- **Official name / CLI or API.** Tencent Cloud CodeBuddy
  (`npx @tencent-ai/codebuddy-code --acp`). Snowflake Cortex Code
  (registry binary). Registry `codebuddy-code` observed `2.137.1`
  (was `2.106.7` on 2026-08-19). `cortex-code` still `1.0.73`.
- **Vendor.** Tencent Cloud and Snowflake. Do not flatten onto each
  other or onto Copilot.
- **Composer flags.** `--acp` for CodeBuddy. Cortex install is
  binary-only in the registry snapshot.
- **Adjacent Swallowtail.** None. Research 158 named first-party leads.
- **Gap.** Version drift on CodeBuddy is currentness of a watchlist
  row, not a new family. No Swallowtail corpus.
- **Composer-relevant.** Possibly, after identity.
- **Named incompatible reason.** Registry membership is not admission.
- **Recommendation.** Already inventoried. Stay deferred.

#### Official Codex ACP package

- **Official name / CLI or API.** `@agentclientprotocol/codex-acp`.
  Registry `codex-acp`. Official ACP-project wrapper around Codex.
- **Vendor.** Agent Client Protocol / OpenAI-adjacent. Do not flatten
  onto `codex.exec` or `codex.app-server`.
- **Composer flags.** ACP stdio. Codex profile still lives on the
  native CLI.
- **Adjacent Swallowtail.** Production `codex.exec` and
  `codex.app-server` (`swallowtail-adapter-codex`,
  `codex-prepared-integration.md`). Research 158.
- **Gap.** Codex already has two production transports. ACP-native
  Codex is a distinct wire, not a missing family.
- **Composer-relevant.** Only if ACP-native Codex is materially
  distinct from app-server.
- **Named incompatible reason.** A Yes on app-server never promotes
  `codex-acp`.
- **Recommendation.** Already inventoried. Do not add. Revisit only
  if ACP-native Codex is proved distinct.

#### Moonshot kimi-cli (Python)

- **Official name / CLI or API.** Moonshot `kimi-cli`. `kimi acp`.
  Official README now says it is evolving into Kimi Code CLI and will
  be wound down. [kimi-cli](https://github.com/MoonshotAI/kimi-cli).
  Registry `kimi`.
- **Vendor.** Moonshot. Do not flatten onto `kimi-code.*` or
  `kimi-platform.chat`.
- **Composer flags.** `kimi acp`, `kimi login`.
- **Adjacent Swallowtail.** Production Kimi Code ACP/headless/local-server
  and Kimi Platform chat. Backlog declined
  `python-kimi-cli-headless-route.md`. Research 158.
- **Gap.** Official successor is the TypeScript Kimi Code already
  adapted. Official status changed toward wind-down, not a new family.
- **Composer-relevant.** No.
- **Named incompatible reason.** Do not flatten. Do not reopen the
  declined Python route.
- **Recommendation.** Already inventoried. Do not add.

#### ACP siblings of selected print or HTTP routes

- **Official names.** Mistral Vibe ACP (`mistral-vibe` registry
  `2.24.1`), Qoder ACP (`qoder` registry `0.2.14`), OpenCode ACP
  (`opencode` registry), Qwen Code `qwen --acp` and experimental
  `qwen serve` HTTP+SSE daemon.
  Official Qwen: [architecture](https://qwenlm.github.io/qwen-code-docs/en/developers/architecture/),
  [headless](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/).
- **Vendors.** Mistral, Alibaba Qoder, OpenCode, Qwen / Alibaba.
  Keep each sibling unflattened.
- **Composer flags.** ACP vs print vs HTTP daemon are separate
  selections. Qwen `--output-format`, `--continue`, `--resume`,
  budgets.
- **Adjacent Swallowtail.** Production `mistral-vibe.headless`,
  `qoder.headless`, `opencode.http`, `qwen.headless`. Research 158.
- **Gap.** Sibling transports were deliberately not selected. Official
  Qwen daemon remains experimental.
- **Composer-relevant.** Only after a named sibling card.
- **Named incompatible reason.** A Yes on the selected transport never
  promotes the sibling.
- **Recommendation.** Already inventoried. Do not add from this triage.

#### Rejected community wrappers

- **Official name / CLI or API.** None. Registry `amp-acp` (community
  wrapper for Amp), `glm-acp-agent` (community GLM/Zhipu), `pi-acp`
  (already rejected, Research 152). Additional unofficial Amp adapters
  (`acp-amp`) exist. Official Amp still has no native ACP.
- **Vendors.** Community authors, not Amp / Zhipu / Pi.
- **Adjacent Swallowtail.** Production `pi.rpc`. Amp official CLI is
  a **separate** new candidate below.
- **Gap.** Wrappers. Official status unchanged.
- **Composer-relevant.** No.
- **Named incompatible reason.** Same collapse class as `pi-acp`.
- **Recommendation.** Already inventoried. Stay rejected. Do not wrap.

#### Watchlist overlap (MiMo, Kilo, Roo)

- **Official names.** Xiaomi MiMo Code (`@xiaomi-mimo/cli`, still
  `0.3.0-alpha.0`, GPL-2.0). Kilo Code official `kilo acp`
  (`@kilocode/cli`). Roo Code: no first-party ACP registry row.
- **Vendors.** Xiaomi, Kilo-Org, Roo. Do not flatten onto OpenCode or
  `cline.*`.
- **Adjacent Swallowtail.** `opencode.http`, `cline.acp` /
  `cline.headless`. Research 158.
- **Gap.** Alpha, Cline-family overlap, or no distinct machine-facing
  row. Official status has not clearly changed.
- **Composer-relevant.** Not until identity proves a material wire
  divergence.
- **Named incompatible reason.** Do not add a third Cline-shaped ACP
  from registry presence.
- **Recommendation.** Already inventoried. Stay deferred.

#### ACP registry leftovers

- **Official names.** Snapshot
  `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
  version `1.0.0`, fetched 2026-08-21. Still-deferred IDs from
  Research 158: `autohand`, `corust-agent`, `crow-cli`, `dimcode`,
  `dirac`, `fast-agent`, `harn`, `minion-code`, `nova`, `poolside`,
  `sigit`, `stakpak`, `vtcode`. `agoragentic-acp` stays rejected
  (marketplace + USDC settlement, wrong shape).
- **Vendors.** Mixed. Do not flatten any of them onto an existing
  family.
- **Adjacent Swallowtail.** None as production rows. Research 158.
- **Gap.** Discovery only. No Swallowtail corpus. Official first-party
  docs were not independently confirmed for each leftover in this pass.
- **Composer-relevant.** Not yet.
- **Named incompatible reason.** Registry membership is not admission.
- **Recommendation.** Already inventoried. Stay deferred unless
  transport, authority, install, and lifecycle evidence is strong
  enough for a named roadmap after operator review.

**New registry row since Research 158:** `antigravity-acp` (Google LLC
authors, `dl.google.com` `agy_acp_server` binaries). Treated as a
genuinely new candidate below. Do not confuse it with community
`agy-acp` wrappers.

### Genuinely new

#### Amp official CLI execute / stream-json

- **Official name / CLI or API.** Amp CLI. `@ampcode/cli`.
  `amp -x` / `--execute`, `--stream-json`, `--stream-json-input`,
  `--stream-json-thinking`, `amp threads continue -x`, `--fast`,
  `--dangerously-allow-all`, `amp login`. Official:
  [Owner's Manual](https://ampcode.com/manual),
  [amp -x](https://ampcode.com/news/amp-x),
  [appendix stream JSON](https://ampcode.com/manual/appendix).
  SDK `@ampcode/sdk` wraps the CLI (`execute()`).
- **Vendor.** Amp / Sourcegraph-origin. Do not flatten onto the
  rejected `amp-acp` wrapper or onto Codex / Claude print routes.
- **Composer flags.** Execute vs TUI, stream-json vs text, thinking
  blocks, continue-thread, fast/features, MCP servers, dangerously
  allow all, IDE/JetBrains attach. Auth is Amp subscription login or
  access token. ChatGPT-subscription linking is a vendor billing
  overlay, not a Swallowtail route.
- **Adjacent Swallowtail.** None. Research 158 inventoried only the
  community ACP wrapper.
- **Gap.** First-party installed harness with Claude-compatible
  stream-json. No Swallowtail family. Official native ACP still
  absent.
- **Composer-relevant.** Yes. Unique Amp login, thread continue, and
  stream-json-thinking.
- **Named incompatible reason.** Amp login is installed/delegated, not
  hosted OAuth first-proof. Do not wrap community ACP. Generation
  controls stay typed; do not turn Amp UI labels into provider
  strings.
- **Recommendation.** Worth a later inventory card for
  `amp.execute` / stream-json. Do not start from `amp-acp`.

#### OpenAI Agents SDK / Sandbox Agents

- **Official name / CLI or API.** OpenAI Agents SDK. Python
  `openai-agents`, JS `@openai/agents`. 2026-04-15 harness plus
  native sandbox (`SandboxAgent`, `Manifest`, snapshot/reconnect).
  Official: [next evolution](https://openai.com/index/the-next-evolution-of-the-agents-sdk/),
  [JS repo](https://github.com/openai/openai-agents-js).
  Works with Responses and Chat Completions. Sandbox agents are
  documented as beta on the JS side.
- **Vendor.** OpenAI. Do not flatten onto `openai.background`,
  `openai.realtime`, or Codex.
- **Composer flags.** Agent instructions, sandbox client (local Unix,
  Docker, or hosted sandbox vendors), Manifest mounts, capabilities
  (fs/shell/skills/memory), `runAs`, session/snapshot restore.
- **Adjacent Swallowtail.** Production `openai.background` and
  `openai.realtime`. Codex exec/app-server. Research 013 already
  rejected in-process foreign SDKs as a Rust embed.
- **Gap.** Official 2026 product. It is a Python/TS orchestration
  library plus optional sandboxes, not a remote Agent HTTP route and
  not a Rust SDK. Swallowtail would have to spawn a language sidecar
  or re-implement the harness.
- **Composer-relevant.** High if admitted (sandbox backend, Manifest,
  snapshot). That is a new topology.
- **Named incompatible reason.** Library, not a server. Do not embed
  a Python/TS SDK as a fake native driver. A Yes on background
  Responses never promotes Agents SDK.
- **Recommendation.** Wait for an official machine-facing boundary
  Swallowtail can own (HTTP Agent API, or a pinned sidecar with
  exact process evidence). Not a first inventory card.

#### OpenAI Responses HTTP (non-background)

- **Official name / CLI or API.** OpenAI Responses API.
  `POST https://api.openai.com/v1/responses`. `previous_response_id`,
  `store`, hosted tools (web search, file search, code interpreter,
  computer use, remote MCP). Official:
  [migrate](https://developers.openai.com/api/docs/guides/migrate-to-responses),
  [why Responses](https://developers.openai.com/blog/responses-api).
- **Vendor.** OpenAI. Do not flatten onto `openai.background` or
  Realtime.
- **Composer flags.** Model, `instructions`, `max_output_tokens`,
  reasoning effort, `store`, `previous_response_id`, hosted tools,
  JSON schema. Background mode is a separate existing route.
- **Adjacent Swallowtail.** Production `openai.background` is the
  **background** Responses branch only. Research 171 gates it for
  addable-descriptor work because of retention/detachment. There is
  no ordinary sync/stateful Responses row.
- **Gap.** Distinct official HTTP surface. Current matrix covers
  background retention and Realtime media, not the default Responses
  loop.
- **Composer-relevant.** Yes. Store/previous-id, hosted tools, and
  reasoning are unique vs background-only.
- **Named incompatible reason.** A Yes on `openai.background` never
  promotes this row. Do not translate UI labels into `reasoning`
  strings. API key is hosted API-key, not hosted OAuth.
- **Recommendation.** Worth a later inventory card as
  `openai.responses` (name TBD). Keep it separate from background and
  Realtime.

#### OpenAI Codex Cloud

- **Official name / CLI or API.** Codex Cloud. `codex cloud`,
  `codex cloud exec`, `codex cloud list --json`. Runs in
  OpenAI-managed containers. Auth is the same ChatGPT or API-key
  profile as local CLI. Official:
  [Codex manual](https://developers.openai.com/codex/codex-manual.md),
  [CLI reference](https://developers.openai.com/codex/cli/reference.md).
- **Vendor.** OpenAI. Do not flatten onto `codex.exec` or
  `codex.app-server`.
- **Composer flags.** Cloud environment, internet access, GitHub repo,
  review vs implement, JSON list/cursor.
- **Adjacent Swallowtail.** Production Codex exec and app-server.
  Codex SDK (`openai-codex` / `@openai/codex-sdk`) wraps local
  app-server or CLI; that is not this surface.
- **Gap.** Hosted container execution is a different topology:
  remote workspace, environment setup, secrets-during-setup, then
  agent phase. No Swallowtail row.
- **Composer-relevant.** Yes. Cloud environment and network posture
  are unique.
- **Named incompatible reason.** Library, not a server. ChatGPT login
  is not hosted OAuth first-proof. A Yes on local exec never promotes
  Cloud.
- **Recommendation.** Worth a later inventory card only after
  contracts can represent remote workspace, ownership, and cleanup
  honestly. Adjacent to existing Codex; do not add as a profile
  switch.

#### Anthropic Claude Agent SDK

- **Official name / CLI or API.** Claude Agent SDK. Python
  `claude-agent-sdk` (`query()`, `ClaudeSDKClient`), TypeScript
  `@anthropic-ai/claude-agent-sdk`. Official:
  [overview](https://code.claude.com/docs/en/agent-sdk/overview).
  Anthropic tells other languages to run the CLI `-p` /
  `--output-format json`. Third-party products must not offer
  claude.ai login; use API keys.
- **Vendor.** Anthropic. Do not flatten onto `claude-agent.acp`,
  `claude-code.headless`, `claude-code.response-only`,
  `anthropic.messages`, or `anthropic.managed-agent`.
- **Composer flags.** `ClaudeAgentOptions`: system prompt, tools,
  MCP, permissions, resume/continue, budget tokens, Bedrock/Vertex
  routing.
- **Adjacent Swallowtail.** All five Anthropic/Claude production
  routes plus Research 013 (SDK is a CLI-bundling library, not a
  Rust embed).
- **Gap.** Official programmable loop. Swallowtail already drives the
  same agent via ACP and two Claude Code stdio routes. The SDK is
  in-process Python/TS around that CLI.
- **Composer-relevant.** Low. Existing Claude Code / ACP controls
  already cover the useful flags.
- **Named incompatible reason.** Do not embed a foreign SDK. Do not
  offer claude.ai login. A Yes on ACP never promotes the SDK.
- **Recommendation.** Adjacent to existing Claude Agent / Claude Code.
  Do not add unless the SDK exposes a wire the CLI routes cannot
  represent.

#### Google Agent Development Kit (ADK) 2.0

- **Official name / CLI or API.** Google Agent Development Kit.
  Python `google-adk` (2.x GA 2026-05-19; observed 2.7.x in August
  2026). Also TypeScript, Go, Java, Kotlin. Official:
  [adk.dev](https://adk.dev/),
  [Cloud ADK](https://docs.cloud.google.com/gemini-enterprise-agent-platform/build/adk).
  Graph workflow runtime. Deploy to Agent Runtime, Cloud Run, or GKE.
- **Vendor.** Google. Do not flatten onto Gemini CLI, Gemini Live, or
  Antigravity.
- **Composer flags.** Workflow graph, model backend, session/memory,
  deploy target. These are app-author controls, not a single harness
  binary.
- **Adjacent Swallowtail.** Production `gemini-cli.acp`,
  `gemini-cli.headless`, `gemini.live`. Antigravity catalogue/headless.
- **Gap.** Official 2026 framework. It is an authoring/runtime SDK,
  not a pinned coding-harness executable Swallowtail can spawn.
- **Composer-relevant.** Only if a later hosted Agent Runtime API is
  selected.
- **Named incompatible reason.** Do not embed ADK as a native driver.
  Cloud IAM is closer to Bedrock than to a public API key.
- **Recommendation.** Wait for a named Agent Runtime invoke API or a
  pinned ADK-serving boundary. Do not add the framework itself.

#### Google Antigravity official ACP server binary

- **Official name / CLI or API.** ACP registry `antigravity-acp`
  `1.0.0`, authors Google LLC. Distribution from
  `dl.google.com/agy-extensions/releases/.../agy-acp-server-...`
  (`agy_acp_server.par` / `.exe`). Website
  [IDE extensions](https://antigravity.google/docs/ide/extensions).
  Unified Google Account or Gemini Enterprise sign-in.
  Native `agy --acp` is **not** confirmed. The CLI issue requesting
  it remains open:
  [antigravity-cli#31](https://github.com/google-antigravity/antigravity-cli/issues/31).
  Community `agy-acp` wrappers exist and are not official.
- **Vendor.** Google. Do not flatten onto `antigravity.catalogue`,
  `antigravity.headless`, or Gemini CLI ACP.
- **Composer flags.** IDE-extension ACP process. Registry linux args
  include `--uid=`. Auth is Google subscription / enterprise, same
  family as existing Antigravity rows.
- **Adjacent Swallowtail.** Production Antigravity catalogue and
  headless. Prepared guide `antigravity-prepared-integration.md`.
  This registry row is **not** in Research 158.
- **Gap.** First-party-looking ACP binary appeared after the 2026-08-19
  watchlist close. Official prose documents IDE extensions, not a
  library-facing `agy --acp`. Identity corpus does not exist.
- **Composer-relevant.** Yes, if the binary is a supported stdio ACP
  agent rather than an IDE-private helper.
- **Named incompatible reason.** Do not wrap community `agy-acp`.
  Google subscription login is not hosted OAuth first-proof. A Yes
  on Antigravity headless never promotes ACP.
- **Recommendation.** Worth a later inventory card only after official
  docs name `agy_acp_server` as a supported integration surface.
  Until then, treat registry+CDN as discovery, not admission.

#### xAI Grok Bot

- **Official name / CLI or API.** Grok Bot. Early beta announced
  2026-08-11. Persistent cloud VM per user, messaging, approvals,
  connectors, routines. Official:
  [overview](https://docs.x.ai/grok-bot/overview),
  [intro](https://x.ai/news/introducing-grok-bot).
  Available to SuperGrok Heavy / selected Cursor plans via desktop
  and iOS. This pass found **no** public Bot HTTP/CLI invoke API.
- **Vendor.** xAI. Do not flatten onto `grok-build.acp` or
  `xai.responses-websocket`.
- **Composer flags.** Bot identity, approvals, connectors, computer
  access. No documented library flags.
- **Adjacent Swallowtail.** Production Grok Build ACP and xAI
  Responses WebSocket.
- **Gap.** Official 2026 product. Consumer/subscription teammate, not
  a machine-facing harness Swallowtail can drive.
- **Composer-relevant.** Not until an official API exists.
- **Named incompatible reason.** Library, not a server. Do not scrape
  the desktop app. Hosted OAuth / subscription login is parked.
- **Recommendation.** Wait for an official Bot API. Do not add.

#### xAI Responses HTTP

- **Official name / CLI or API.** xAI Responses API.
  `POST https://api.x.ai/v1/responses`. Bearer `XAI_API_KEY`.
  `previous_response_id`, `store` (default true, 30-day retention),
  `max_output_tokens`, `include: ["reasoning.encrypted_content"]`.
  Official: [generate text](https://docs.x.ai/developers/model-capabilities/text/generate-text),
  [comparison](https://docs.x.ai/developers/model-capabilities/text/comparison).
  Chat Completions is documented as deprecated on xAI.
- **Vendor.** xAI. Do not flatten onto `xai.responses-websocket` or
  Grok Build.
- **Composer flags.** Model (`grok-4.6` and others), store, previous
  id, reasoning include, tools (function, web search, X search, code
  execution).
- **Adjacent Swallowtail.** Production `xai.responses-websocket` only.
  Auxiliary `swallowtail.xai.models` catalogue. Research 171 gates
  the WebSocket row as a realtime shape.
- **Gap.** Official HTTP Responses is a distinct transport from the
  WebSocket route. Not in the matrix.
- **Composer-relevant.** Yes. Store/previous-id vs connection-scoped
  WebSocket.
- **Named incompatible reason.** A Yes on the WebSocket route never
  promotes HTTP. API key is hosted API-key, not hosted OAuth.
- **Recommendation.** Worth a later inventory card as
  `xai.responses-http` (name TBD). Keep it separate from WebSocket
  and from Grok Build.

#### Grok Build headless

- **Official name / CLI or API.** Grok Build CLI `grok`.
  `grok -p` / `--single`, `--output-format streaming-json|json|plain`,
  `-m`, `--effort`, `--always-approve`, `--sandbox`, `--cwd`,
  `--resume` / `--continue`, `--tools` / `--disallowed-tools`.
  Official: [overview](https://docs.x.ai/build/overview),
  [CLI reference](https://docs.x.ai/build/cli/reference),
  [headless](https://docs.x.ai/build/cli/headless-scripting).
  Auth: browser on first launch, or `XAI_API_KEY`.
- **Vendor.** xAI. Do not flatten onto `grok-build.acp`.
- **Composer flags.** Prompt, output format, model, effort, sandbox,
  permissions, max turns, resume/continue, tool allow/deny.
- **Adjacent Swallowtail.** Production `grok-build.acp` only. Same
  shape as deferred `kiro.headless` relative to `kiro.acp`.
- **Gap.** Official first-party print/stream-json sibling. Not in the
  matrix and not in Research 158.
- **Composer-relevant.** Yes. Unique vs ACP: output-format, sandbox
  profile, effort.
- **Named incompatible reason.** A Yes on `grok-build.acp` never
  promotes headless. Browser login is not hosted OAuth first-proof.
  API-key env is a separate explicit profile.
- **Recommendation.** Worth a later inventory card as
  `grok-build.headless`. Same sibling pattern as deferred Kiro
  headless.

#### Amazon Nova Act

- **Official name / CLI or API.** Amazon Nova Act. AWS service for
  browser/UI workflow agents. Python SDK `nova-act`, IDE extension,
  CLI, console, playground `nova.amazon.com/act`. Deploys onto
  Bedrock AgentCore Runtime / Browser. Official:
  [docs](https://docs.aws.amazon.com/nova-act/),
  [what is](https://docs.aws.amazon.com/nova-act/latest/userguide/what-is-nova-act.html),
  [product](https://aws.amazon.com/nova/act/).
  Region: US East (N. Virginia) in current docs.
- **Vendor.** Amazon. Do not flatten onto `bedrock.runtime`,
  Bedrock Agents Classic, or AgentCore.
- **Composer flags.** Starting page / CDP endpoint, `act("...")`
  natural-language step, human takeover, AgentCore deployment.
- **Adjacent Swallowtail.** Production Bedrock runtime/catalogue.
  No Nova or AgentCore row.
- **Gap.** Official 2025–2026 UI-automation service. Topology is
  hosted browser fleet plus IAM, not a coding harness.
- **Composer-relevant.** Yes, but the controls are browser/UI, not
  repo/workspace.
- **Named incompatible reason.** Delegated cloud identity, not a
  portable API-key field. Library, not a server. A Yes on Bedrock
  Converse never promotes Nova Act.
- **Recommendation.** Worth a later inventory card only if Swallowtail
  wants a UI-automation family. Not a coding-matrix default.

#### Amazon Bedrock Agents (InvokeAgent)

- **Official name / CLI or API.** Agents for Amazon Bedrock Runtime.
  `InvokeAgent` (`bedrock-agent-runtime`). Requires `agentId`,
  `agentAliasId`, `sessionId`. Official:
  [how agents work](https://docs.aws.amazon.com/bedrock/latest/userguide/agents-how.html),
  [InvokeAgent](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent-runtime_InvokeAgent.html).
  AWS notes Agents Classic is no longer open to new customers and
  points to AgentCore.
- **Vendor.** Amazon. Do not flatten onto `bedrock.runtime` Converse.
- **Composer flags.** Agent id/alias, session id, trace, end-session,
  knowledge-base / action-group configuration (operator-owned agent).
- **Adjacent Swallowtail.** Production `bedrock.runtime` and
  `bedrock.catalogue`.
- **Gap.** Distinct official agent runtime. Classic is wind-down for
  new customers. Operator-owned agent version resembles Anthropic
  Managed Agent more than Converse.
- **Composer-relevant.** Yes, if admitted. Agent/alias/session are
  unique.
- **Named incompatible reason.** Cloud IAM / SigV4. A Yes on Runtime
  never promotes Agents. Do not start Classic if AgentCore is the
  current official path.
- **Recommendation.** Prefer AgentCore if any Amazon agent family is
  inventoried. Treat Classic as wait/avoid for new customers.

#### Amazon Bedrock AgentCore

- **Official name / CLI or API.** Amazon Bedrock AgentCore.
  Data plane `InvokeAgentRuntime`. Harness: inline model, prompt,
  tools; isolated microVM with filesystem/shell. Also Runtime,
  Gateway, Memory, Browser, Code Interpreter. Official:
  [what is](https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/what-is-bedrock-agentcore.html),
  [InvokeAgentRuntime](https://docs.aws.amazon.com/bedrock-agentcore/latest/APIReference/API_InvokeAgentRuntime.html),
  [docs home](https://docs.aws.amazon.com/bedrock-agentcore/).
- **Vendor.** Amazon. Do not flatten onto Bedrock Runtime, Agents
  Classic, or Nova Act.
- **Composer flags.** Agent runtime ARN/id, qualifier, session id,
  payload, IAM / inbound OAuth. OAuth invoke cannot use the AWS SDK
  path; HTTPS inbound auth is documented separately.
- **Adjacent Swallowtail.** Bedrock runtime/catalogue only.
- **Gap.** Official 2026 agent platform. New topology: hosted microVM
  harness plus IAM. Inbound OAuth would hit the parked hosted-OAuth
  gate if that is the first proof.
- **Composer-relevant.** Yes.
- **Named incompatible reason.** Library, not a server. Cloud identity
  is not a portable API-key field. Do not pick AgentCore as the
  hosted-OAuth first-proof unless the operator names that route.
- **Recommendation.** Worth a later inventory card as a distinct
  Amazon family. First profile should stay IAM invoke, not inbound
  OAuth.

#### Mistral Agents and Conversations API

- **Official name / CLI or API.** Mistral Agents API plus
  Conversations. `POST /v1/agents`, then start a conversation with
  `agent_id` and `inputs`. Built-in connectors: web search, code
  interpreter, image generation, document library, MCP connectors,
  handoffs. Official:
  [introduction](https://docs.mistral.ai/studio-api/agents/introduction),
  [agents](https://docs.mistral.ai/studio-api/agents/agents-api),
  [beta agents](https://docs.mistral.ai/api/endpoint/beta/agents).
  Auth: Mistral API key.
- **Vendor.** Mistral. Do not flatten onto `mistral-vibe.headless`.
- **Composer flags.** Agent model/instructions/tools, conversation
  persistence, connector selection, handoff targets.
- **Adjacent Swallowtail.** Production `mistral-vibe.headless` only.
  Research 158 deferred Vibe ACP, not this hosted API.
- **Gap.** Official hosted agent/conversation runtime. Distinct from
  the installed Vibe print route.
- **Composer-relevant.** Yes. Agent-id and conversation-id are unique.
- **Named incompatible reason.** A Yes on Vibe headless never promotes
  Agents API. API key is hosted API-key, not hosted OAuth.
- **Recommendation.** Worth a later inventory card as
  `mistral.agents` (name TBD). Keep Vibe ACP/headless separate.

#### Alibaba Model Studio Managed Agents API

- **Official name / CLI or API.** Model Studio Managed Agents API.
  Hosted sessions, sandboxes, tools, SSE event streams.
  `POST /agents`, `/environments`, `/sessions`,
  `POST /sessions/{id}/events`. Official:
  [overview](https://help.aliyun.com/en/model-studio/managed-agents-api-overview).
  DashScope SDK module; Python SDK `>=1.26.2`.
- **Vendor.** Alibaba Cloud Model Studio. Do not flatten onto
  `alibaba.conversations` or Qwen Code.
- **Composer flags.** Agent version, environment, session, event
  stream, archive/delete.
- **Adjacent Swallowtail.** Production `alibaba.conversations`
  (OpenAI-compatible conversations/responses). Research 171 gates
  that row for addable work because of retained conversation
  authority.
- **Gap.** Official provider-hosted harness, closer to Anthropic
  Managed Agent than to conversations-responses.
- **Composer-relevant.** Yes.
- **Named incompatible reason.** A Yes on conversations never
  promotes Managed Agents. Pay-as-you-go API key, not hosted OAuth.
- **Recommendation.** Worth a later inventory card as a distinct
  Alibaba family.

#### Alibaba DashScope agent application API

- **Official name / CLI or API.** DashScope agent/workflow application
  completion. `POST https://dashscope.aliyuncs.com/api/v1/apps/APP_ID/completion`
  (intl `dashscope-intl.aliyuncs.com`). `session_id` or `messages`,
  `stream`, `file_list`, `biz_params`. Bearer `DASHSCOPE_API_KEY`.
  Official:
  [call an agent](https://help.aliyun.com/en/model-studio/call-single-agent-application/),
  [application API](https://help.aliyun.com/en/model-studio/agent-and-workflow-application-api-reference).
- **Vendor.** Alibaba Cloud Model Studio. Do not flatten onto
  Managed Agents or `alibaba.conversations`.
- **Composer flags.** APP_ID (console-created application), session
  vs self-managed messages, RAG pipeline ids, custom plugin params.
- **Adjacent Swallowtail.** `alibaba.conversations` uses the
  Singapore workspace OpenAI-compatible conversations facade, not
  `apps/APP_ID/completion`.
- **Gap.** Official application-shaped agent API. Operator-owned
  APP_ID is a different authority than a model id.
- **Composer-relevant.** Yes. APP_ID and session_id are unique.
- **Named incompatible reason.** Do not flatten APP_ID onto the
  conversations facade. A Yes on conversations never promotes this.
- **Recommendation.** Worth a later inventory card, separate from
  Managed Agents and conversations.

#### DeepSeek Responses API

- **Official name / CLI or API.** DeepSeek Responses API.
  `https://api.deepseek.com` with OpenAI Responses format.
  Official: [Responses guide](https://api-docs.deepseek.com/guides/responses_api/).
  Models documented as `deepseek-v4-flash`, `deepseek-v4-pro`,
  `deepseek-v4-flash-vision-exp`. Tools: function and web_search;
  other tool types ignored. `store` / `previous_response_id` are not
  full OpenAI twins (`store: false` in compatibility notes).
- **Vendor.** DeepSeek. Do not flatten onto `deepseek.continuation`
  (Chat Completions) or DeepSeek Harness.
- **Composer flags.** Model, `instructions`, `input`,
  `max_output_tokens`, `tools` / `tool_choice`, stream.
- **Adjacent Swallowtail.** Production `deepseek.continuation`,
  `deepseek-harness.jsonrpc`, `deepseek-harness.local-server`.
  Research 018 covered Chat Completions and Anthropic-format
  compatibility, not this Responses facade.
- **Gap.** Official 2026 Responses-compatible endpoint. Distinct
  codec from the existing Chat Completions continuation row.
- **Composer-relevant.** Yes. Responses item types vs chat messages.
- **Named incompatible reason.** A Yes on continuation never promotes
  Responses. Do not silently ignore unsupported fields the way the
  provider docs allow clients to. Swallowtail must fail closed on
  unmapped controls.
- **Recommendation.** Worth a later inventory card as
  `deepseek.responses`. Keep Anthropic-format
  `https://api.deepseek.com/anthropic` as a separate later question,
  not this row.

#### Gemini generateContent hosted direct

- **Official name / CLI or API.** Gemini Developer API
  `models.generateContent` / `streamGenerateContent` via the unified
  Google Gen AI SDK or REST. API-key project auth. Official:
  [libraries](https://ai.google.dev/gemini-api/docs/libraries),
  [generate content](https://docs.cloud.google.com/gemini-enterprise-agent-platform/reference/models/inference).
- **Vendor.** Google. Do not flatten onto `gemini-cli.*`,
  `gemini.live`, or Antigravity.
- **Composer flags.** Model, `thinkingConfig` / `thinkingLevel`,
  `maxOutputTokens`, tools, Google Search, multimodal contents.
- **Adjacent Swallowtail.** Production Gemini CLI ACP/headless and
  Gemini Live. Auxiliary `swallowtail.gemini.models` catalogue.
  There is no hosted generateContent inference row.
- **Gap.** Official hosted direct API. Matrix covers installed CLI
  and Live WebSocket only.
- **Composer-relevant.** Yes. Thinking level and search are unique vs
  CLI.
- **Named incompatible reason.** A Yes on Gemini CLI never promotes
  hosted generateContent. API key is hosted API-key, not hosted
  OAuth. Gemini CLI range requalification stays a separate deferred
  currentness item.
- **Recommendation.** Worth a later inventory card as
  `gemini.generate-content` (name TBD).

#### Gemini Enterprise Agent Platform / Vertex Agent Engine

- **Official name / CLI or API.** Gemini Enterprise Agent Platform
  (formerly Vertex AI agent surface). Same Gen AI SDK with
  `vertexai=True`, project, location, IAM. Agent Engine / Agent
  Runtime for deployed ADK agents. Official:
  [migrate to cloud](https://ai.google.dev/gemini-api/docs/migrate-to-cloud),
  [product](https://cloud.google.com/products/gemini-enterprise-agent-platform).
- **Vendor.** Google. Do not flatten onto Gemini Developer API,
  Gemini CLI, ADK-the-framework, or Antigravity.
- **Composer flags.** Project, location, IAM, Agent Engine resource,
  session/memory bank.
- **Adjacent Swallowtail.** Gemini CLI, Live, models catalogue.
  Bedrock is the existing cloud-IAM analog.
- **Gap.** Official enterprise invoke/deploy path. Different auth and
  residency than the Developer API key.
- **Composer-relevant.** Yes. Project/location/IAM are unique.
- **Named incompatible reason.** Delegated cloud identity, not a
  portable public API-key field. A Yes on generateContent never
  promotes Vertex.
- **Recommendation.** Worth a later inventory card only after a
  Bedrock-like cloud-identity proof is wanted for Google. Do not
  bundle it with generateContent.

### Checked and dropped

#### Windsurf Cascade

Official Cascade is an IDE-bound agent in Devin Desktop / Windsurf.
Docs: [Cascade](https://docs.windsurf.com/windsurf/cascade/cascade).
This pass did not find an official headless CLI or Agent API for
Cascade itself. Cognition's machine-facing surface is Devin CLI ACP,
already inventoried. Do not invent a Windsurf route. Do not flatten
Cascade onto Devin.

#### OpenAI Codex SDK

Official Python/TS SDKs wrap local Codex app-server or CLI.
[Codex SDK](https://developers.openai.com/codex/sdk). Adjacent to
existing `codex.app-server`. Do not add a family that only re-embeds
the current driver.

#### OpenAI Chat Completions

Official and still supported. Responses is the current official
primitive for new agent work. Swallowtail never selected Chat
Completions. Not a 2026 new surface. Do not add from this triage.

## Recommendations, not decisions

These are recommendations only. They do not invent a lane, do not
answer for the operator, and do not change the matrix.

**Do not add from this PR.** No adapter, contract, matrix, composer,
or flag work follows from this note.

**Already inventoried (24).** Cite Research 143/153/158, Research
170/171, and the backlog. Official status has not clearly changed
except stronger first-party docs for Devin, Factory Droid, Auggie,
and Junie, plus CodeBuddy version drift. Those remain watchlist /
named-roadmap-after-review, not new discoveries. OpenHands, Aider,
and Kiro headless stay deferred. Community wrappers stay rejected.
`kimi-cli` is winding down into already-adapted Kimi Code.

**Genuinely new, highest-signal later inventory (not a decision).**

1. Amp official `amp --execute --stream-json` — first-party installed
   harness, closest to existing print-route evidence.
2. Grok Build headless — official sibling of production
   `grok-build.acp`.
3. OpenAI Responses HTTP and xAI Responses HTTP — hosted API-key
   families the matrix does not cover; keep separate from background
   and WebSocket.
4. DeepSeek Responses — distinct codec from `deepseek.continuation`.
5. Mistral Agents API, Alibaba Managed Agents, Alibaba DashScope
   `apps/APP_ID` — hosted agent runtimes, unflattened from existing
   Vibe / conversations rows.
6. Gemini generateContent — hosted direct Google inference, unflattened
   from CLI and Live.
7. Factory Droid / Auggie / Devin / Junie — already inventoried;
   official docs are now strong enough that a later identity card is
   the remaining gap, not discovery.

**Wait / do not start.**

- OpenAI Agents SDK, Claude Agent SDK, Google ADK: official, but
  language-sidecar or framework embeds. Research 013 still applies.
- Codex Cloud, Grok Bot, Nova Act, AgentCore, Vertex Agent Engine:
  official, heavier topology (remote computer, IAM, browser fleet).
- Antigravity `agy_acp_server`: first-party-looking CDN/registry row;
  wait for official integration docs. Do not wrap community ACP.
- Bedrock Agents Classic: official but closed to new customers;
  prefer AgentCore if Amazon agent work is later selected.
- Windsurf Cascade, Codex SDK, Chat Completions: checked, do not add.

**Composer.** Any later family that lands will need unique per-route
controls. Do not reuse another family's labels. Generation-controls
stays typed.

**OAuth.** Nothing here is the hosted URL-open plus loopback
first-proof. Installed Amp / Devin / Factory / Junie / Grok / Google
logins stay delegated or inherited. Parked gate unchanged.

**Next move, if Tom wants one.** Pick at most one later inventory
card from the highest-signal list. Do not batch-implement. Do not
edit the matrix from this document.
