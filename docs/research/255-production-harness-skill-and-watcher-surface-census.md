# 255 Production Harness Skill And Watcher Surface Census

Status: promoted
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Question source: g05.001 production harness skill and watcher surface census
Assigned card: [g05.001](../roadmaps/g05/batch-cards/001-production-harness-skill-and-watcher-surface-census.md)

## Question

Across every production harness route, what can be established without a prompt,
credential, paid operation, installation, login, account inspection, broad host
or project scan, skill injection, or process mutation about:

- skill provenance and prompt-free listing;
- selected model and session visibility;
- native background identity, status, wait, output, stop, and terminal truth;
- process ownership, process-tree join, cancellation, deadline, and turn
  completion?

## Boundary and method

The route boundary is the route-row enumeration in
[provider-route-matrix.md](../guides/provider-route-matrix.md) at base
69c8180553aace72635a2dec938d432d248c1e5e. The guide prose says 47 production
routes, but the installed, attached, hosted, realtime, embedded, and local
runtime sections contain 48 route rows. This census records the row
enumeration, not the stale prose total.

The census includes 35 harness rows:

- 32 installed harnesses;
- 2 attached harnesses;
- 1 provider-hosted harness, anthropic.managed-agent.

It excludes 13 direct API, realtime, embedded SDK, and local-runtime rows.
The excluded rows do not own the questioned harness skill or watcher surface.
OpenAI background responses remain direct provider state, not a production
harness process route for this card.

Evidence is limited to official documentation, exact source or distribution
records, existing repository fixtures, and prompt-free help or list surfaces
already captured by the repository. No live provider process, login, prompt,
credential, paid call, installation, update, account inspection, broad
filesystem scan, skill injection, or process mutation was used. Version
records below preserve the current repository qualification; they do not
raise any compatibility bound.

## Result

No included route has a prompt-free, selected-route proof of both an active
skill roster and selected model/session visibility. No included route has a
consumer-controllable native watcher with all of native identity, start,
status, wait, output, stop, and terminal truth.

The useful dispositions are therefore:

- skill discovery: 0 deliver-now rows;
- native watcher control: 0 deliver-now rows;
- provider or adapter activity: observed on several routes, but not a host
  watcher authority;
- host lifecycle: available for owned child processes and client tasks, but
  not evidence of a provider-native background task.

Claude/T3 Code activity research remains a lead for observable activity and
ownership boundaries. It is not portable harness behavior and does not select
an implementation. The evidence supports no production, architecture, or
contract change in this card.

## Truth layers

| Layer | What it can establish | What it cannot establish |
| --- | --- | --- |
| Distribution membership | A bundled binary, source feature, help entry, or package artifact exists | That the selected route loads it, exposes it to the model, or exposes its current names |
| Configuration provenance | A route may read host, project, plugin, extension, or provider-managed configuration | The actual configured contents without an out-of-scope scan |
| Selected argv or wire | The exact repository-selected command, adapter profile, method set, and fixture-visible fields | Hidden provider defaults or a live account's resolved state |
| Model/session visibility | A value explicitly returned by the selected prompt-free protocol | A value inferred from a catalogue, distribution member, product name, or prose advertisement |
| Native activity | Provider events, IDs, statuses, tool calls, child activity, or persisted events observed in a fixture | Host authority to wait, stop, or control the observed object |
| Host lifecycle | Ownership, cancellation, deadline, cleanup, and process-tree join for host-started work | Provider-native background identity or provider task completion |
| Consumer control | An explicit selected method or host-owned handle with defined authority | Control inferred from command activity, a native ID, or a provider's internal task name |

Distribution membership is not model visibility. Provider-observed command
activity or a native task ID is not a host-controllable watcher.

## Route boundary and exact version register

The following is the complete included set. Exact ranges, gaps, exclusions,
opaque identities, and unverified-newer posture are carried from the current
route claims and their existing qualification research.

| # | Route | Topology | Exact qualified identity or claim | Evidence bundle |
| ---: | --- | --- | --- | --- |
| 1 | antigravity.catalogue | installed catalogue | antigravity-cli.release 1.1.9..=1.1.17; later visible as UnverifiedNewer | AG |
| 2 | antigravity.headless | installed child | antigravity-cli.release 1.1.9..=1.1.17; later visible as UnverifiedNewer | AG |
| 3 | codex.exec | installed child | codex.cli: 0.80.0..=0.81.0, 0.84.0..=0.98.0, 0.99.0..=0.121.0, 0.122.0..=0.149.1; exclusions 0.108.0 and 0.109.0 | CX |
| 4 | codex.app-server | installed child | codex.cli: legacy 0.80..=0.99, explicit 0.100..=0.107, base 0.110..=0.130, workspace roots 0.131..=0.149.1; gaps 0.82..=0.83 and 0.108..=0.109 | CX |
| 5 | claude-agent.acp | installed child | claude-agent.acp-adapter 0.53.0..=0.70.0; exclusion 0.58.0 | CL |
| 6 | claude-code.headless | installed child | claude-code.headless-stream-json 2.1.220..=2.1.241; later visible as UnverifiedNewer | CL |
| 7 | claude-code.response-only | installed child | claude-code.headless-stream-json 2.1.227..=2.1.241; deny list retained | CL |
| 8 | cursor-agent.catalogue | installed catalogue | qualified build specimens 2026.07.01-41b2de7, 2026.07.23-e383d2b, 2026.08.04-aaa8809, 2026.08.11-e8db854 | CU |
| 9 | cursor-agent.acp | installed child | same exact qualified build specimens; no inferred calendar gap | CU |
| 10 | cursor-agent.headless | installed child | same exact qualified build specimens; later dates are unverified | CU |
| 11 | gemini-cli.acp | installed child | gemini-cli 0.51.0..=0.56.0; later visible as UnverifiedNewer | GM |
| 12 | gemini-cli.headless | installed child | gemini-cli 0.51.0..=0.56.0; later visible as UnverifiedNewer | GM |
| 13 | grok-build.acp | installed child | deprecated 0.2.114..=0.2.117; maintained 1.0.4..=1.0.5; incompatible mid-gaps 0.2.118..=0.2.121 and 1.0.0..=1.0.3 | GB |
| 14 | kimi-code.acp | installed child | 0.28.1 and 0.29.0..=0.38.0; later visible as UnverifiedNewer | KM |
| 15 | kimi-code.headless | installed child | 0.29.0..=0.37.2 protocol v1 plus 0.38.0 protocol v2; later synthetic versions unverified | KM |
| 16 | muse-code.headless | installed child | opaque muse-code 0.2.1-R1215.1; no UnverifiedNewer claim | MU |
| 17 | command-code.headless | installed child | command-code 1.15.1; no UnverifiedNewer claim | CC |
| 18 | cline.acp | installed child | cline 3.0.55; no UnverifiedNewer claim | CN |
| 19 | cline.headless | installed child | cline 3.0.55; no UnverifiedNewer claim | CN |
| 20 | goose.acp | installed child | goose 1.46.0; commit 98c11ce2ee7b9b302978aa64b1eab7d0895607c7; no UnverifiedNewer claim | GO |
| 21 | kiro.acp | installed child | kiro-cli 2.18.1; no UnverifiedNewer claim | KI |
| 22 | deepagents.acp | installed child | deepagents-acp 0.1.25; no UnverifiedNewer claim | DA |
| 23 | copilot-cli.acp | installed child | copilot-cli 1.0.80; no UnverifiedNewer claim | CP |
| 24 | mistral-vibe.headless | installed child | mistral-vibe v2.24.2; selected agent profile is plan | MV |
| 25 | qoder.headless | installed child | qoder 1.1.25; no UnverifiedNewer claim | QO |
| 26 | deepseek-harness.jsonrpc | installed child | deepseek-harness 0.1.0rc6; no UnverifiedNewer claim | DS |
| 27 | zcode.app-server | installed child | zcode 0.16.3; no UnverifiedNewer claim | ZC |
| 28 | deepseek-harness.local-server | installed child | deepseek-harness 0.1.0-rc.6; no UnverifiedNewer claim | DS |
| 29 | oh-my-pi.rpc | installed child | oh-my-pi package 17.2.9..=17.4.0; 18.x not qualified | OP |
| 30 | pi.rpc | installed child | pi package qualified through 0.84.3; retained segment gaps and exclusions; later visible as UnverifiedNewer | PI |
| 31 | pi.sdk-sidecar | installed child | pi SDK sidecar 0.84.2 plus Node 22.23.2; wire swallowtail-pi-sdk-jsonl-v1; source-tag axes | PI |
| 32 | qwen.headless | installed child | deprecated 0.19.11..=0.20.1 and 0.21.0..=0.21.14; maintained 0.21.15 and same-revision 0.22.0..=0.22.1; 0.21.16 incompatible | QW |
| 33 | kimi-code.local-server | attached or owned local server | milestones 0.28.1, 0.29.0, 0.29.1..=0.30.0, 0.31.0, 0.31.1, 0.32.0..=0.34.0, 0.35.0..=0.38.0; later visible as UnverifiedNewer | KM |
| 34 | opencode.http | attached server | opencode.server published exact segments 1.14.48..=1.18.20; not one inferred continuous range; later visible as UnverifiedNewer | OC |
| 35 | anthropic.managed-agent | provider-hosted harness | opaque anthropic.managed-agents-facade | MA |

Excluded route rows are anthropic.messages, kimi-platform.chat,
deepseek.continuation, alibaba.conversations, openai.background,
anthropic.realtime, openai.realtime, gemini.live, aws.bedrock.converse,
aws.bedrock.converse-stream, llama.cpp.server, ollama.chat, and
llama.cpp.embedded. They are direct provider state, realtime transport,
embedded SDK, or local model runtime rows. They do not add a production
harness route to this census.

## Frozen official cross-route sources

These sources were fetched on 2026-08-28. The digest records the exact
source body used for this evidence pass.

| Source | Use | SHA-256 |
| --- | --- | --- |
| [ACP v1 overview](https://agentclientprotocol.com/protocol/v1/overview) | Standard initialize, session, prompt, update, cancel, and extensibility surface; no standard skill registry or watcher registry in the overview | 9cd19934bacae0319fea43537054c087dee108362cc77eb5fec798360312bf71 |
| [OpenCode server API](https://dev.opencode.ai/docs/server/) | External server health, events, session, prompt_async, status, abort, agent, command, and tool surfaces; no skill endpoint in the server table | e9be608c288468ccaefb49f7f831265b721c1596a2bb2f85af644573b5bd0cd3 |
| [Claude Code skills](https://code.claude.com/docs/en/skills) | Official bundled skill and skill-expansion behavior | f4d39a7a3cb3f8d7181800fbb0307e771b237b581ca850018c1e695b69f4c834 |
| [Claude Code headless](https://code.claude.com/docs/en/headless) | Headless selection, auto-discovery, bare mode, background restriction, and terminal behavior | 1765d7d91a794af5a696a8a278442d5a091406bab18f61e54c2126f30db0173a |
| [Managed agent sessions](https://platform.claude.com/docs/en/managed-agents/sessions) | Provider-hosted managed session and agent identity | 1aaaf37aa3858b37cd567a3be9d0fa8b8129f6953593f3cdde9ddc156a33b916 |
| [Managed agent session creation](https://platform.claude.com/docs/en/api/beta/sessions/create) | Provider-managed session fields and skills beta; selected repository route returns an empty skills set | 1a3781cf426793cd84beaacee40602cc63c3e1048bb3eff841603c35fa6ad4d8 |

The official sources document upstream capabilities. They do not override the
selected route, exact fixture, or current compatibility claim.

## Frozen local corpus

The bundle codes keep the matrix readable while pointing to exact repository
evidence. The cited prior research records contain the source URLs, fixture
paths, hashes, and route-specific qualification notes.

| Bundle | Exact fixture or source roots | Prior evidence |
| --- | --- | --- |
| AG | crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.9 and antigravity-cli-1.1.17 | [205](205-antigravity-headless-agent-profile-evidence.md), 078, 079, 177 |
| CX | crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.149.1 | [201](201-codex-0-149-1-identity.md), 064, 213, 229, 234, 242, 246 |
| CL | crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.70.0 and claude-code-2.1.241 | [202](202-claude-code-2-1-241-identity.md), 161, 212, 226, 249 |
| CU | crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-2026.08.04-2026.08.11 and protocol ACP fixtures | 135, 183, 223, 224, 243 |
| GM | crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.56.0 and protocol ACP fixtures | 182, 230, 235, 239, 244 |
| GB | crates/swallowtail-protocol-acp/tests/fixtures/acp-v1-grok-build-0.2.114 | 163, 204, 219 |
| KM | Kimi adapter fixtures for 0.38.0, headless v2, and local server 0.31.0; provider-evidence fixture corpus | 040, [066](066-non-acp-harness-activity-inventory-and-corpus.md), 068, 069, 179, 207, 208, 210, 211 |
| MU | crates/swallowtail-adapter-muse/tests/fixtures/muse-code-0.2.1-R1215.1 | [112](112-muse-code-installed-route-qualification.md), 131 |
| CC | crates/swallowtail-adapter-command-code/tests/fixtures/command-code-1.15.1 | 116, 117, 118 |
| CN | Cline ACP and headless fixtures for 3.0.55 | 146, 147, 220, 221, 240, 248 |
| GO | Goose 1.46.0 fixtures and g04-089b ACP-builtins fixtures | [250](250-goose-acp-builtin-evidence.md), 148, 253 |
| KI | Kiro ACP 2.18.1 fixtures | [251](251-kiro-acp-effort-evidence.md), [254](254-kiro-acp-agent-profile-evidence.md), 156 |
| DA | DeepAgents ACP 0.1.25 fixtures | [206](206-deepagents-acp-model-selection-evidence.md), 157 |
| CP | Copilot CLI ACP 1.0.80 fixtures | [218](218-copilot-cli-acp-built-in-tool-allowlist-evidence.md), 149, 188 |
| MV | Mistral Vibe 2.24.2 route and agent-profile fixtures | [252](252-mistral-vibe-headless-agent-profile-evidence.md), 150, 199 |
| QO | Qoder headless 1.1.25 fixtures | 151, 200 |
| DS | DeepSeek harness runtime-bin and web fixtures for 0.1.0rc6 | 124, [125](125-deepseek-harness-web-api-route-qualification.md), 197 |
| ZC | Zcode runtime 0.16.3 fixtures | 126 |
| OP | oh-my-pi 17.4.0 fixtures | 109, 178, [217](217-oh-my-pi-18-identity.md) |
| PI | Pi RPC 0.84.3 and SDK sidecar fixtures | [215](215-pi-rpc-0-84-3-identity.md), [181](181-pi-sdk-sidecar-route-qualification.md), 228 |
| QW | Qwen 0.21.15 and 0.22.1 fixtures | [222](222-qwen-headless-plan-mode-evidence.md), 173, 198, 216 |
| OC | OpenCode 1.18.20 and published segment fixtures | 039, 066, 136, 176, 214 |
| MA | Managed Agents 2026-04-01 agent, session, and session-create fixtures | 016, 066, 102, 103 |

## Skill matrix

Provenance codes are deliberately separate: B bundled or distribution,
H host-configured, P project-local, X plugin or extension, V
provider-managed, and ? unknown or not safely inspectable. A code means the
source class may exist; it does not mean the selected route loaded it.

| Route | Provenance evidence | Prompt-free list | Selected model/session visibility | Mutation, auth, and freshness | Disposition |
| --- | --- | --- | --- | --- | --- |
| antigravity.catalogue | — | N/A | Catalogue only | Provider catalogue; live account state not inspected | Not a skill route |
| antigravity.headless | B/H/P/? | None | No active roster | Agent or skill expansion is documented; resolving host state needs a scan or provider auth | Stop at documentation/help |
| codex.exec | ? | None | No active roster | Selected exec ignores user config and rules; no provider account inspection | No skill evidence |
| codex.app-server | ? | None | Model/thread protocol fields only; no skill roster | Configured profile is not a skill list; no host scan | No skill evidence |
| claude-agent.acp | ? | None | Session identity only; no skill field | ACP baseline has no standard skill registry; provider auth not used | No portable skill evidence |
| claude-code.headless | B/H/P/X | None | No active roster | Selected settings permit ambient discovery; current headless docs identify bare mode as the suppression gate, but selected route does not use it | Unsafe to resolve without scan or prompt |
| claude-code.response-only | B/H/P/X | Suppressed | No active roster | Selected tools, MCP, slash expansion, and persistence are suppressed; no account inspection | Distribution membership is not visibility |
| cursor-agent.catalogue | — | N/A | Catalogue only | Provider catalogue; freshness not promoted | Not a skill route |
| cursor-agent.acp | ? | None | Session identity only; no skill field | ACP extension space is not a portable registry; no auth or host scan | No skill evidence |
| cursor-agent.headless | H/P/? | None | No active roster | Selected headless stream has no prompt-free roster | No skill evidence |
| gemini-cli.acp | H/P/X/? | None | Session identity only; no skill field | ACP has no standard skill registry; ambient configuration not inspected | No skill evidence |
| gemini-cli.headless | H/P/X/? | None | No active roster | Selected route has no mapped skill surface; extension and provider state not scanned | No skill evidence |
| grok-build.acp | H/P/X/? | None | Session identity only; no skill field | Subagent parser evidence is unselected; provider config and auth not inspected | Activity is not skill visibility |
| kimi-code.acp | H/P/X/? | None | Session identity only; no skill field | Import does not inherit newer versions; host/provider state not scanned | No skill evidence |
| kimi-code.headless | H/P/X/? | None | No active roster | Selected v1/v2 identity is fixed by route claim; native wait tool is unselected | No skill evidence |
| muse-code.headless | B/H/P | Help-only; selected route suppresses foreign context | No active roster | Public skills command would inspect or mutate; selected route does not trust workspace skills and excludes foreign personal context | Help is not selected visibility |
| command-code.headless | H/? | Suppressed | No active roster | Selected route uses the explicit no-skills posture; no auth or scan | Strong negative evidence |
| cline.acp | ? | None | Session identity only; no skill field | ACP baseline only; no provider auth | No skill evidence |
| cline.headless | ? | None | No active roster | Plan/headless fixture has no roster | No skill evidence |
| goose.acp | B/H/X/? | Observed-only registry source | No active roster | Builtin registry and host extensions exist; selected builtin flag is unmapped; no live provider auth | Source membership is not selected visibility |
| kiro.acp | B/H/P | Help/docs-only | Session identity only; no skill field | Optional agent and effort surfaces are separate; package or account state not inspected | No skill evidence |
| deepagents.acp | B/H/P/? | Observed-only CLI option | Session identity only; no skill field | CLI skill option is unmapped in selected ACP; no prompt or provider auth | Unmapped option is not selected behavior |
| copilot-cli.acp | B/H/X/? | Observed-only tool/prose | Session identity only; no skill field | Builtin skill tool is documented, but selected available-tools surface is unmapped; no account inspection | Documentation is not active visibility |
| mistral-vibe.headless | B/H/P | None | Fixed plan profile; no applied skill roster | Selected plan profile is exact; custom and subagent profiles are not selected | Profile membership is not applied visibility |
| qoder.headless | B/H/P/X? | Empty response field | Model field plus empty skills/plugins in init fixture | Provider selection is fixture-visible; live account state not inspected | Empty selected evidence |
| deepseek-harness.jsonrpc | ? | None | Session identity only; no skill field | Skill method is denied in selected allowlist; no auth or prompt | Strong negative evidence |
| zcode.app-server | ? | None | Session identity only; no skill field | Selected method set has no skill surface; no live session inspection | No skill evidence |
| deepseek-harness.local-server | ? | Suppressed | Session identity only; no skill field | Skill invocation is denied by selected method policy; no broad scan | Strong negative evidence |
| oh-my-pi.rpc | B/H/P/X? | Suppressed | No active roster | Selected route disables skills and ambient extension/context surfaces; no auth | Strong negative evidence |
| pi.rpc | B/H/P/X? | Suppressed | No active roster | Selected route disables skills and ambient extension/context surfaces; exact route is pinned | Strong negative evidence |
| pi.sdk-sidecar | B/H/P/X? | Suppressed | In-memory sidecar identity only; no skill roster | Validation rejects ambient skills, extensions, prompt templates, context files, themes, retries, and fallback | Strong negative evidence |
| qwen.headless | B/H/P/X? | Suppressed | No active roster | Selected safe-mode tool set excludes skill and task surfaces; no auth or scan | Strong negative evidence |
| kimi-code.local-server | H/P/X/V? | None | Session and model fields only; no skill roster | Rich activity and task events exist; attached-server auth/freshness not inspected | Activity is not skill visibility |
| opencode.http | B/H/P/X? | None | Session, status, agent, command, and tool endpoints; no skill endpoint | Attached server owns auth and freshness; server is not started or stopped by Swallowtail | No skill endpoint, no safe registry |
| anthropic.managed-agent | V/B? | Empty selected response field | Provider session identity plus empty skills in fixtures | Managed agent docs expose provider fields, but selected route skills are empty; account state not inspected | Empty selected evidence |

The matrix has no deliver-now row. A prompt-free help entry, compiled registry,
official prose, or distribution member is not an active selected roster. A
provider login, prompt, host/project/plugin scan, or inspection of private
configuration would be required to close the unresolved rows and is outside
this card.

## Watcher matrix

Native identity and activity are recorded as observations. A native
start/status/wait/output/stop surface is a deliverable only when the selected
route also gives the consumer authority over the same object. Active-turn
cancel is recorded separately from background-task stop.

| Route | Native identity | Start | Status, wait, and output | Stop | Selected terminal | Watcher disposition |
| --- | --- | --- | --- | --- | --- | --- |
| antigravity.catalogue | None | N/A | Catalogue result | N/A | N/A | Not applicable |
| antigravity.headless | Provider activity only | Provider stream | Stream output; no bound wait/status | No consumer stop | Process result | Activity only |
| codex.exec | Provider child/collab activity IDs | Provider run | Activity notifications; no bound watcher handle | No consumer child stop | Exec terminal | Activity only |
| codex.app-server | Provider thread and child activity | Provider turn/collab | Turn and activity status; no bound watcher wait/output object | No consumer child stop | Turn terminal | Activity only |
| claude-agent.acp | ACP session ID | Session/new | Session/update stream | session/cancel for active turn | Prompt terminal | Session control, not watcher |
| claude-code.headless | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| claude-code.response-only | None | Host process | Result output only | Host process cancellation | Result/exit | Host lifecycle only |
| cursor-agent.catalogue | None | N/A | Catalogue result | N/A | N/A | Not applicable |
| cursor-agent.acp | ACP session ID | Session/new | Session/update stream | session/cancel for active turn | Prompt terminal | Session control, not watcher |
| cursor-agent.headless | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| gemini-cli.acp | ACP session ID | Session/new | Session/update stream | session/cancel for active turn | Prompt terminal | Session control, not watcher |
| gemini-cli.headless | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| grok-build.acp | Provider activity only | Provider run | Activity evidence; no bound wait/status/stop | No consumer child stop | Prompt terminal | Activity only |
| kimi-code.acp | ACP session/activity ID | Session/new | Session/update stream | session/cancel for active turn | Prompt terminal | Session control, not watcher |
| kimi-code.headless | Provider stream identity | Provider run | Stream output; no task watcher handle | No consumer task stop | Stream terminal | Activity only |
| muse-code.headless | Provider task stream identity | Provider run | Task lifecycle output/status observed | No bound native task stop | Run terminal | Activity only |
| command-code.headless | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| cline.acp | ACP session ID | Session/new | Session/update stream | session/cancel for active turn | Prompt terminal | Session control, not watcher |
| cline.headless | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| goose.acp | ACP session ID | Session/new | Session/update stream | session/cancel for active turn | Prompt terminal | Session control, not watcher |
| kiro.acp | ACP session ID | Session/new | Session/update stream | session/cancel for active turn | Prompt terminal | Session control, not watcher |
| deepagents.acp | ACP session ID | Session/new | Session/update stream | session/cancel for active turn | Prompt terminal | Session control, not watcher |
| copilot-cli.acp | ACP session ID | Session/new | Session/update stream | session/cancel for active turn | Prompt terminal | Session control, not watcher |
| mistral-vibe.headless | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| qoder.headless | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| deepseek-harness.jsonrpc | Session/subagent notification IDs | Provider run | session.status and subagent notifications observed | Native cancel unavailable; host force-stop | turn/end | Activity only |
| zcode.app-server | Session ID | Session/create | Subscribe events; no watcher wait | Native cancel unavailable; host force-stop | turn.completed or turn.failed | Activity only |
| deepseek-harness.local-server | Provider session ID | Session/create and prompt | Host/status frames and session events | Session cancel for active turn; no generic task stop | Session terminal event | Session control, not watcher |
| oh-my-pi.rpc | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| pi.rpc | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| pi.sdk-sidecar | Sidecar operation identity | Host task | JSONL operation output | Host task cancellation | Sidecar terminal frame | Host lifecycle only |
| qwen.headless | None | Host process | Stream output only | Host process cancellation | Result/exit | Host lifecycle only |
| kimi-code.local-server | Native subagent/task identities | Server turn | Native events include start, status, output, and completion | WebSocket abort is active-turn control; no generic task stop | Turn/task events | Provider activity only |
| opencode.http | Provider session/run ID | prompt_async | Session status and SSE events | session abort for active turn | Session idle or terminal event | Provider session control, not watcher |
| anthropic.managed-agent | Provider session/run ID | Remote managed session | Persisted status and events | user.interrupt for remote run | Persisted terminal state | Remote session control, not local watcher |

Positive native activity evidence does not satisfy the watcher contract. In
particular, Kimi local server task events, Codex child activity, DeepSeek
subagent notifications, Muse task lifecycle, OpenCode session events, and
managed-agent persisted events are correlation or provider-session evidence.
They do not supply a consumer-owned background handle with complete
start/status/wait/output/stop authority.

## Host lifecycle matrix

The host lifecycle is a separate truth layer. Owned process trees are joined
by the adapter or host cleanup path; provider descendants and attached
servers are not silently promoted to host ownership.

| Route | Process ownership | Process-tree join | Cancellation and deadline | Turn-completion truth |
| --- | --- | --- | --- | --- |
| antigravity.catalogue | No child | N/A | Catalogue operation deadline | Catalogue result |
| antigravity.headless | Host-owned child | Root child and IO/task cleanup | Host cancellation and route deadline | Selected stream/result terminal plus cleanup |
| codex.exec | Host-owned child | Root child and adapter IO/task cleanup; provider child activity is not OS ownership | Host cancellation and route deadline | Exec terminal plus joined cleanup |
| codex.app-server | Host-owned app-server child | Root child, connection, and adapter task cleanup; provider descendants remain provider-owned | Host cancellation and turn deadline posture; session-open deadline unsupported | Turn terminal plus local cleanup |
| claude-agent.acp | Host-owned child | Root child, ACP connection, and task cleanup | Active-turn cancellation; session-open deadline posture | Prompt terminal plus joined cleanup |
| claude-code.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| claude-code.response-only | Host-owned child | Root child and response task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| cursor-agent.catalogue | No child | N/A | Catalogue operation deadline | Catalogue result |
| cursor-agent.acp | Host-owned child | Root child, ACP connection, and task cleanup | Active-turn cancellation; session-open deadline posture | Prompt terminal plus joined cleanup |
| cursor-agent.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| gemini-cli.acp | Host-owned child | Root child, ACP connection, and task cleanup | Active-turn cancellation; session-open deadline posture | Prompt terminal plus joined cleanup |
| gemini-cli.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| grok-build.acp | Host-owned child | Root child, ACP connection, and task cleanup | Host cancellation and route deadline | Prompt terminal plus joined cleanup |
| kimi-code.acp | Host-owned child | Root child, ACP connection, and task cleanup | Active-turn cancellation; session-open deadline posture | Prompt terminal plus joined cleanup |
| kimi-code.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Stream terminal plus joined cleanup |
| muse-code.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Run terminal plus joined cleanup |
| command-code.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| cline.acp | Host-owned child | Root child, ACP connection, and task cleanup | Active-turn cancellation; session-open deadline posture | Prompt terminal plus joined cleanup |
| cline.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| goose.acp | Host-owned child | Root child, ACP connection, and task cleanup | Active-turn cancellation; session-open deadline posture | Prompt terminal plus joined cleanup |
| kiro.acp | Host-owned child | Root child, ACP connection, and task cleanup | Active-turn cancellation; session-open deadline posture | Prompt terminal plus joined cleanup |
| deepagents.acp | Host-owned child | Root child, ACP connection, and task cleanup | Active-turn cancellation; session-open deadline posture | Prompt terminal plus joined cleanup |
| copilot-cli.acp | Host-owned child | Root child, ACP connection, and task cleanup | Active-turn cancellation; session-open deadline posture | Prompt terminal plus joined cleanup |
| mistral-vibe.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| qoder.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| deepseek-harness.jsonrpc | Host-owned child | Root child, JSON-RPC task, and IO cleanup | Host cancellation and deadline; force-stop when native cancel is unavailable | turn/end plus joined cleanup |
| zcode.app-server | Host-owned child | Root child, connection, and subscription task cleanup | Host cancellation and deadline; force-stop when native cancel is unavailable | turn.completed or turn.failed plus joined cleanup |
| deepseek-harness.local-server | Owned server or attached client according to configured mode | Owned mode joins root server and client tasks; attached mode joins client tasks only | Active-turn session cancel where selected; host deadline and cleanup | Session terminal event plus local cleanup |
| oh-my-pi.rpc | Host-owned child | Root child and RPC task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| pi.rpc | Host-owned child | Root child and RPC task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| pi.sdk-sidecar | Host-owned sidecar task | Sidecar task and JSONL cleanup | Host cancellation and route deadline | Sidecar terminal frame plus cleanup |
| qwen.headless | Host-owned child | Root child and stream task cleanup | Host cancellation and route deadline | Result/exit plus joined cleanup |
| kimi-code.local-server | Attached server is external; owned mode starts a local root | Owned mode joins root server and client tasks; attached mode never joins external descendants | Active-turn abort plus host deadline; no generic task stop | Selected turn terminal plus local cleanup |
| opencode.http | External attached server | HTTP/SSE client tasks are joined; external server is not | Session abort plus host deadline | Provider session terminal/idle plus client cleanup; detachment is not provider success |
| anthropic.managed-agent | No local provider process | HTTP/SSE/task cleanup only | Remote interrupt plus host deadline | Persisted provider terminal state plus client cleanup |

An ordinary operation is successful only when the selected route terminal is
valid and local cleanup has completed. Provider activity, watcher-like events,
or a detached state do not replace turn completion truth. Cancellation,
deadline, and cleanup are distinct from successful completion.

## Evidence gaps and unsafe gates

- No selected route exposes a complete prompt-free skill roster together with
  selected model/session visibility.
- Many routes may have host, project, plugin, extension, or provider-managed
  skill provenance, but resolving contents would require a broad scan or
  provider/account state. That is outside the handoff.
- Official ACP has extensibility but no portable skill or watcher registry.
  Provider-specific extension methods cannot be promoted across ACP routes.
- Claude Code documents bundled skills and a bare-mode suppression gate, but
  the selected headless route does not use bare mode and does not return an
  active roster. This is a route-specific scan/prompt gate, not a portable
  claim.
- OpenCode exposes session, status, event, prompt, and abort surfaces, but
  the current server API has no skill endpoint and the attached server is not
  host-owned.
- Managed-agent documentation exposes provider-managed fields, while the
  selected repository fixtures record empty skills. No broader account or
  model roster is claimed.
- Native child, task, command, or subagent activity is not consumer control.
  The only safe host control proven here is the host-owned process or client
  task lifecycle.
- No private skill names, local secret paths, credentials, provider payloads,
  or live output were copied into this record.

## Decision

This is an evidence-only promotion. It selects no model, skill source,
watcher API, provider behavior, or product policy. It records zero
deliver-now skill rows and zero deliver-now watcher rows. It makes no
production, architecture, or contract change.

Card 002 owns any later operator decision about an evidence-shaped operator
surface. Card 003 owns any later route-specific follow-up. Both remain
planned.

## Authority references

- [Provider route matrix](../guides/provider-route-matrix.md)
- [Provider-solution activity matrix](../guides/provider-solution-activity-matrix.md)
- [Product guardrails](../architecture/product-guardrails.md)
- [Contract 013 Interactive Session Access Policy](../contracts/013-interactive-session-access-policy.md)
- [Contract 017 Provider-Owned Session Load, Replay, And Host Containment](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
- [Contract 023 Harness Operation Isolation And Native Boundary](../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification And Compatibility](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 044 Observable Agent Activity And Disclosure](../contracts/044-observable-agent-activity-and-disclosure.md)
- [Contract 047 Configured Provider Instance Catalogue](../contracts/047-configured-provider-instance-catalogue.md)
- [Research 063: Claude/T3 Code activity lead](063-observable-agent-activity-and-t3-code-reference.md)
- [Research 066: non-ACP activity corpus](066-non-acp-harness-activity-inventory-and-corpus.md)
- [Research 072: subagent topology evidence](072-subagent-topology-observation-and-control-evidence.md)

## Promotion record

Research 255 is promoted by g05.001. Card 001 and PR 112 closed the census;
cards 002-003 recorded operator decisions and promoted Contracts 058-059.
Qoder and Claude Code remain conditional evidence candidates under Research
256 and 257. No zero-row census result was converted into a production claim.
