# 091 g03 Evidence-Gated Maintenance Checkpoint

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Does current consumer or upstream evidence justify another g03 implementation
lane after prepared-facade usability closeout?

## Method

The checkpoint compared safe local `--version` observations, official package
registry and upstream release metadata, current Swallowtail qualified bounds,
the three deferred gates, current Nucleus and Soundcheck worktree records, the
workspace package graph, Contract 036, and the realized architecture.

It ran no provider prompt, authentication flow, model catalogue, session
operation, workspace mutation, installation, update, publication, or consumer
edit.

## Compatibility Result

| Surface | Local observation | Current external point | Swallowtail boundary | Result |
| --- | --- | --- | --- | --- |
| Codex CLI | `0.146.0` | `0.146.0` | through `0.146.0` | unchanged |
| Claude Code | `2.1.220` | `2.1.220` | exact `2.1.220` | unchanged |
| Claude Agent ACP | `0.63.0` | `0.64.0` | through `0.64.0` | unchanged |
| Gemini CLI | `0.53.0` | `0.53.1` | ACP `0.51.0`; headless through `0.52.0` | deferred, unchanged |
| Kimi Code | `0.31.0` | `0.31.1` | through `0.31.1` | unchanged |
| Grok Build | `0.2.117` | `0.2.117` | through `0.2.117` | unchanged |
| Pi RPC | `0.83.0` | `0.83.0` | through `0.83.0` | unchanged |
| Qwen Code | `0.21.2` | `0.21.2` | through `0.21.2` | unchanged |
| OpenCode | `1.18.10` | `1.18.10` | through `1.18.10` | unchanged |
| Antigravity CLI | `1.1.9` | tag `1.1.9` | exact `1.1.9` | unchanged |
| Cursor Agent | `2026.07.01-41b2de7` | registry `2026.07.23` | both exact builds | unchanged |
| stable ACP | local wrapper evidence only | schema `v1.20.0` | schema `v1.20.0` | unchanged |

The official registries expose newer preview, nightly, alpha, or development
points for several tools. Those channels do not change stable compatibility
truth and do not justify qualification work.

## Consumer Result

Nucleus has ongoing uncommitted structured-chat presentation work, but its
latest native acceptance record closes the prior Codex request-reference and
child-lifecycle defects. The bounded Plan-mode child run and separate
Normal-mode task-list run both completed with zero failed, active, or
unexpected terminal turns. It records no new Swallowtail defect.

Soundcheck has ongoing product work outside its Codex integration. Its latest
roadmap still records Swallowtail adoption as complete and reports no new
portable integration failure.

## Authority Drift

The compatibility pass found no implementation candidate, but the repository
authority was stale:

- Cargo metadata and the accepted public API baseline contain 26 workspace
  libraries
- Contract 036 still declared 24 and omitted the accepted Antigravity and
  Cursor adapter packages
- realized architecture still said 24 crates, a 23-crate prepared dependency
  graph, and 30 production routes instead of 26 crates and 32 routes
- package metadata, internal-dependency, and public-API checks still encoded or
  reported the older package set

Contract 036, its index summary, the architecture, and the repository front
door now match realized package and route truth. The package checks now cover
the same 26-package graph and accepted probe-feature set. Publication remains
outside the active runway and behind a future operator decision.

## Decision

Do not compile g03.018. Keep g03 active but evidence-gated. Reassess only when
one of these facts changes:

1. Nucleus, Soundcheck, or another accepted consumer reproduces a portable
   Swallowtail defect
2. a non-deferred upstream stable point moves beyond a qualified boundary and
   selected-route evidence shows material drift or useful extension
3. the operator explicitly promotes a deferred gate or selects a new product
   policy lane

Gemini requalification, Pi session continuity, and provider-session binding
persistence remain deferred. Registry publication remains removed rather than
paused. No implementation card is ready.

## Contract Result

Contract 036 receives a currentness correction only. No package, publication
stage, compatibility promise, runtime role, provider capability, access route,
or authority changes. No new contract or roadmap is required.

## Sources

- [Codex npm metadata](https://registry.npmjs.org/@openai%2Fcodex/latest)
- [Claude Code npm metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest)
- [Gemini CLI npm metadata](https://registry.npmjs.org/@google%2Fgemini-cli/latest)
- [Kimi Code npm metadata](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest)
- [Grok npm metadata](https://registry.npmjs.org/@xai-official%2Fgrok/latest)
- [Pi npm metadata](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest)
- [Qwen Code npm metadata](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest)
- [OpenCode npm metadata](https://registry.npmjs.org/opencode-ai/latest)
- [Claude Agent ACP releases](https://github.com/agentclientprotocol/claude-agent-acp/releases)
- [ACP releases](https://github.com/agentclientprotocol/agent-client-protocol/releases)
- [Antigravity CLI tags](https://github.com/google-antigravity/antigravity-cli/tags)
- [ACP agent registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
- local safe version observations and consumer Northstar records
