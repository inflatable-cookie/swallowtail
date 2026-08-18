# 143 New Harness Route Expansion Selection

Status: promoted
Owner: Tom
Date: 2026-08-18

## Question

Which notable current coding-agent and harness surfaces should enter the next
Swallowtail route-expansion runway, and which should remain secondary or under
observation?

## Method

This is a bounded currentness and route-shape pass, not an implementation or
compatibility qualification. It combines:

- the current Swallowtail route matrix and workspace package inventory
- the latest Agent Client Protocol registry
- official project repositories and documentation for candidate harnesses
- the existing Swallowtail research posture around ACP, installed harnesses,
  foreign SDKs, and remote agent servers

No executable was installed, no provider account was used, no login or model
request was made, and no candidate route is qualified by this record.

## Current Swallowtail Shape

The current provider-route matrix declares 39 production routes. The workspace
contains 24 `swallowtail-adapter-*` crates. The README package map lists 22 of
them and omits the already-realized `swallowtail-adapter-deepseek-harness` and
`swallowtail-adapter-zcode`; that documentation drift should be repaired as
part of the first route-expansion acceptance work.

The current installed-harness set already covers Codex, Claude Agent/Code,
Gemini CLI, Grok Build, Kimi Code, OpenCode, Pi, Qwen Code, Antigravity, Cursor,
Oh My Pi, Muse Code, Command Code, DeepSeek Harness, and ZCode. The next
selection therefore favours distinct current harness families and machine-facing
surfaces rather than another direct model API or a cosmetic provider alias.

## Selection Rules

A candidate may enter a roadmap only when its first route has:

1. an explicit executable, server, or protocol boundary;
2. a maintained official source or provider-supported integration surface;
3. a route shape that is materially distinguishable from an existing route;
4. enough observable evidence to build a deterministic corpus without relying
   on a provider prompt or hidden credential state;
5. an authority and cleanup posture that can be represented without adding
   consumer policy or a generic router.

ACP registry membership is discovery evidence. It is not by itself a
compatibility claim, version range, lifecycle guarantee, or implementation
approval.

## Primary Wave

The primary wave is the next route-expansion runway. Each candidate gets its
own evidence gate and may stop at disposition without creating a package.

| Rank | Candidate | Initial route candidates | First useful pressure |
| --- | --- | --- | --- |
| 1 | Cline | `cline.acp`, `cline.headless` | ACP plus explicit JSON headless execution from a large, model-agnostic coding harness |
| 2 | Goose | `goose.acp` | local extensible agent with an explicit ACP client/server boundary and existing Swallowtail ACP infrastructure |
| 3 | GitHub Copilot CLI | `copilot-cli.acp` | first-party GitHub terminal harness with an official ACP server; preview maturity must stay visible |
| 4 | Mistral Vibe | `mistral-vibe.headless` | official programmatic prompt mode with JSON and streaming output; ACP remains a separate candidate branch |
| 5 | Qoder CLI | `qoder.headless` | official terminal-native headless mode with explicit automation and worktree pressure |
| 6 | Pi ACP | `pi.acp` | a distinct ACP transport for the already-supported Pi family; do not flatten it onto `pi.rpc` |

The primary wave does not assume that every candidate needs both ACP and
headless routes. Route identity, version axis, session posture, and package
boundary remain candidate-specific.

## Secondary Wave

The secondary wave is valuable but follows the primary wave because it adds
heavier topology, weaker or less mature machine-facing evidence, or lower
shared information gain.

| Rank | Candidate | Initial route candidate | Reason for deferral |
| --- | --- | --- | --- |
| 1 | OpenHands Agent Server | `openhands.agent-server` | remote HTTP/WebSocket agent execution adds workspace, persistence, ownership, and cleanup policy |
| 2 | Kiro CLI | `kiro.headless` | documented headless mode is useful, but ACP alignment and stable route evidence need a separate check |
| 3 | Aider | `aider.headless` | mature scripted CLI, but its first automation surface is text/Git-oriented rather than a structured event protocol |
| 4 | Deep Agents | `deepagents.acp` | current ACP presence and LangChain implementation are promising, but the executable/package boundary needs qualification |

Each secondary candidate has a stop-at-disposition path. Secondary status does
not mean planned implementation regardless of the primary-wave outcome.

## Watchlist And Negative Disposition Candidates

Crush, Continue, MiMo Code, Kilo Code, and Roo Code remain useful watchlist
inputs. They should not receive automatic adapters merely because they are
popular or appear in a current catalogue. The next disposition pass should ask
whether each exposes a maintained, machine-facing route that is not already
covered by OpenCode, Pi, or an existing ACP/JSONL shape.

The ACP registry also names Amp, Auggie, CodeBuddy, Cortex Code, Devin, Factory
Droid, Junie, GLM Agent, and other agents. They remain discovery leads until
transport, authority, installation, and lifecycle evidence are strong enough for
an explicit Swallowtail route.

## Decision

Promote the primary and secondary waves into `g03.086` and `g03.087`.

- Card 260 owns the primary-wave source and contract-fit gate.
- Cards 261 onward keep each primary candidate on its own evidence and
  implementation lane.
- Card 286 owns the secondary-wave disposition gate.
- Secondary candidates must not be pulled into the primary implementation lane
  by popularity, registry presence, or an orchestrator's convenience.
- No new provider-neutral contract is assumed. If a candidate introduces a new
  authority, lifecycle, topology, or operation shape, stop and promote that
  question into a contract or spec before driver work.

## Non-Goals

- bulk adapter generation from the ACP registry
- provider, model, route, endpoint, credential, or fallback selection policy
- installation, update, login, live inference, or publication
- flattening all candidates onto ACP, OpenCode, Codex, or a generic CLI driver
- qualifying version ranges from package metadata alone
- changing existing route claims or currentness ceilings

## Primary Sources

- [ACP latest registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
- [Agent Client Protocol](https://github.com/agentclientprotocol/agent-client-protocol)
- [Cline](https://github.com/cline/cline)
- [Cline CLI reference](https://docs.cline.bot/cli/cli-reference)
- [Goose](https://github.com/block/goose)
- [Goose ACP clients](https://goose-docs.ai/docs/guides/acp-clients/)
- [Goose ACP providers](https://goose-docs.ai/docs/guides/acp-providers/)
- [GitHub Copilot CLI](https://github.com/github/copilot-cli)
- [Copilot CLI ACP server](https://docs.github.com/en/copilot/reference/copilot-cli-reference/acp-server)
- [Mistral Vibe](https://github.com/mistralai/mistral-vibe)
- [Mistral Vibe CLI](https://docs.mistral.ai/vibe/code/cli/work-with-cli)
- [MiMo Code](https://github.com/XiaomiMiMo/MiMo-Code)
- [Qoder CLI overview](https://docs.qoder.com/cli/overview)
- [Qoder headless scripts](https://docs.qoder.com/cli/run-in-scripts)
- [Kiro headless mode](https://kiro.dev/docs/cli/headless/)
- [Aider scripting](https://aider.chat/docs/scripting.html)
- [OpenHands Agent Server](https://docs.openhands.dev/sdk/arch/agent-server)
- [OpenHands ACP agents](https://docs.openhands.dev/openhands/usage/agent-canvas/acp-agents)
- [Deep Agents](https://github.com/langchain-ai/deepagents)
- [Crush](https://github.com/charmbracelet/crush)
- [Continue](https://github.com/continuedev/continue)
- [Kilo Code](https://github.com/Kilo-Org/kilocode)
