# 017 Qwen Headless And Post-Managed-Harness Coverage Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-21

## Question

Which current route should follow the Claude Managed Agents proof without
inventing repository policy, depending on an unfinished remote protocol, or
making a heavy sandbox a prerequisite for harness communication?

## Method

Official provider, maintained-project, protocol, and release evidence was
checked on 2026-07-21 against the thirteen realized production routes and ten
provider-neutral conformance profiles. No provider account, credential,
repository integration, remote resource, installed Qwen binary, or paid
inference was used.

## Current Evidence

### Remote ACP remains premature

The ACP Streamable HTTP and WebSocket RFD is Active, not Completed. It defines
HTTP/2 SSE and WebSocket profiles, connection and session headers, cookies,
and a common JSON-RPC lifecycle. Its v1 durability section still delegates
reconnect, retry, liveness, and affinity implementation to the SDK or host and
does not replay in-flight messages. The reference implementation is in
progress; Rust and TypeScript SDK support, protocol-version headers,
resumability, and security hardening remain later phases.

Evidence:

- [ACP Streamable HTTP and WebSocket RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport)

### Cursor remains product-policy bound

Cursor's supported Background Agents API is a beta account-scoped bearer route
for agents that work directly on GitHub repositories. Its useful behavior
includes repository access, remote code mutation, follow-up prompts, branch or
pull-request workflow, and durable agent resources. Selecting it would decide
repository, GitHub, remote-workspace, mutation, artifact, and cleanup authority
for consumers. Existing Swallowtail authority does not make that choice.

Evidence:

- [Cursor Background Agents API](https://docs.cursor.com/background-agent/api/overview)
- [Cursor CLI GitHub Actions guidance](https://docs.cursor.com/en/cli/github-actions)

### Qwen Code headless is stable and bounded

Qwen Code `v0.19.11`, source commit `f22cf50`, is a stable maintained release
from 2026-07-16. Its headless route accepts stdin, emits buffered JSON or
stream JSON, supports partial messages and structured output, and has distinct
exit codes for turn-budget, wall/tool-budget, and signal interruption.

The route exposes three useful safety facts:

- `--safe-mode` disables context files, hooks, extensions, skills, MCP
  servers, custom subagents, memory, settings-derived permission rules, and
  settings-derived sandbox selection
- `--max-wall-time`, `--max-tool-calls`, and `--max-session-turns` are explicit
  harness-native bounds; subagent and structured-output exceptions are
  documented and must not be hidden
- sandboxing is disabled by default and `--yolo` does not enable it; the
  default sandbox route uses a container image and remains optional

Release `v0.19.11` also changed the default approval mode to `auto`. A
Swallowtail invocation must therefore select approval mode and tool posture
explicitly rather than inherit the new default.

Qwen OAuth access was discontinued on 2026-04-15. Current supported access
includes distinct Alibaba ModelStudio Coding Plan, Token Plan, standard API
key, third-party provider, and custom-provider routes. The first proof must use
one exact configured access profile and model route. It may delegate credential
handling to the installed harness but cannot report a generic Qwen entitlement.

Headless sessions are persisted as project-scoped local JSONL. The first proof
does not resume or delete that state. It therefore requires explicit durable
harness-retention acceptance and cannot claim an ephemeral run solely because
the process exits.

Evidence:

- [Qwen Code v0.19.11 release](https://github.com/QwenLM/qwen-code/releases/tag/v0.19.11)
- [Qwen Code headless mode](https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/)
- [Qwen Code authentication](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/auth/)
- [Qwen Code configuration](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/settings/)

### Qwen daemon is not the first proof

`qwen serve` now exposes valuable multi-client, workspace, permission,
capability, and SSE-replay behavior, but its public user surface still labels
the route experimental and its native wall/tool budgets do not apply to daemon
ACP sessions. The in-progress official remote-ACP transport also retains
permission and replay gaps. It remains a later experimental route, separate
from stable headless execution.

Evidence:

- [Qwen Code daemon mode](https://qwenlm.github.io/qwen-code-docs/en/users/qwen-serve/)
- [Qwen ACP-over-HTTP replay design](https://qwenlm.github.io/qwen-code-docs/en/design/daemon-acp-http/sse-resumable-stream/)

## Coverage Comparison

| Candidate | New value | Blocking pressure | Result |
| --- | --- | --- | --- |
| Qwen Code headless | second stable one-shot harness, exact native budgets, config suppression, explicit optional sandbox, broad current access routes | structured-run isolation posture is not preflight-bound today | select |
| Cursor Background Agents | repository-backed remote agent and branch/PR lifecycle | consumer repository and mutation policy | operator gate |
| remote ACP | standard remote protocol topology | Active RFD, incomplete reference/SDK/reconnect authority | protocol gate |
| Qwen daemon | multi-client attached harness and replay | experimental route; daemon budgets incomplete | later experimental proof |
| DeepSeek, Z.AI, Alibaba direct APIs | provider breadth | repeat hosted JSON/SSE before compatible-codec reuse is planned | later breadth tranche |
| Ollama, vLLM, SGLang | serving breadth | repeat attached or background lifecycle | later deployment tranche |

## Decision

Select Qwen Code `v0.19.11` headless as the next bounded proof. This is the
first coverage-expansion route after the major runtime shapes, so practical
provider breadth now outweighs inventing another lifecycle abstraction.

The first subset is one read-only, attached, one-shot structured run:

- exact Qwen executable, configured instance, access profile, provider/model
  route, working resource, execution host, and source version
- stdin prompt; bounded stream-JSON output and no prompt in argv
- explicit `--safe-mode`, approval posture, tool exclusions, wall-time,
  top-level tool-call, and session-turn bounds
- explicit `AmbientHost` isolation; no sandbox or container claim
- durable local harness retention explicitly allowed; no resume or deletion
  claim
- no `--yolo`, provider fallback, persistent retry, background agents,
  subagents, MCP, extensions, skills, memory, web/search, attachments,
  scheduled work, or multi-directory access
- host deadline and cancellation remain authoritative; Qwen-native budgets are
  defense-in-depth provider behavior and retain their distinct terminal truth

The route stays harness interaction even when tool use is tightly restricted.
It does not become direct inference.

## Required Promotion

Contract 023 must make harness isolation operation-shape neutral:

- `AmbientHost`, `ProviderEnforced`, and `HostEnforced` apply to structured
  runs as well as interactive sessions
- exact posture participates in requirements, pure preflight, request policy,
  and conformance
- provider approval modes, tool denials, native budgets, and config suppression
  do not establish process isolation
- provider-native sandbox support is opt-in and has no ambient fallback
- host deadlines and cancellation remain independent of harness-native bounds
- local harness retention remains explicit and does not imply resume or delete
  authority

No provisional spec is needed. Existing structured-CLI, process, access,
retention, deadline, event, and cleanup contracts cover the remainder.

## Sequence

1. Promote Contract 023 and realize operation-shape-neutral isolation records.
2. Freeze the exact `v0.19.11` argv, stream-JSON, budget, retention, exit, and
   redaction corpus without a binary or credential.
3. Implement the separately registered Qwen headless driver.
4. Run the one-shot structured-CLI profile under local and remote-authoritative
   host identities; keep the installed/authenticated probe separate.
5. Return to provider breadth after the proof. Recheck direct Kimi Platform,
   DeepSeek, Z.AI, and Alibaba/Qwen compatibility seams before choosing shared
   codec work.

## Promotion

- operation-shape-neutral harness isolation: Contract 023
- implementation sequence: g01 roadmap 026 and cards 080-082
- deferred policy gate: Cursor repository-backed agents
- deferred protocol gate: remote ACP
