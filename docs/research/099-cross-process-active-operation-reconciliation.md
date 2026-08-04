# 099 Cross-Process Active Operation Reconciliation

Status: promoted
Owner: Tom
Updated: 2026-08-04

## Trigger

T3 Code leaves locally active turns marked as working after application restart.
Closing the application can also cancel work even when the provider can retain
it. Swallowtail already imports provider sessions and persists exact resume
bindings, but neither operation answers what happened to a consumer turn that
lost its runtime handle.

## Finding

This is adjacent to provider-session import, not an import extension.

- import selects and attaches a provider session
- reconciliation observes an already-bound session after attachment loss
- reattachment resumes one live event stream
- cancellation changes provider state

Combining them would turn a read-only restart check into implicit attachment or
control authority.

OpenCode exposes the first complete observation surface: exact session lookup,
session status, and retained paged messages. It does not expose a prompt or turn
identifier from `prompt_async`, so its attribution is session-scoped. An idle
session therefore proves only `InactiveUnresolved`, never completion.

Codex app-server `thread/read` carries exact turn records. The promoted mapping
now admits exact turn status when present and fails closed when the requested
turn is absent. Kimi local server carries exact turn ids plus resumable event
cursors and remains the next stronger candidate. OpenAI background and
Anthropic managed-agent work also can
outlive an attachment; their current provider operation/resource references
are operation-private and have no durable recovery record.

ACP child-process routes can often reload retained history. The original child
and live turn do not survive as an observable attachment, so history recovery
must not be advertised as active-turn reattachment or exact terminal proof.

## Route Classification

| Class | Routes | Promotion gate |
| --- | --- | --- |
| supported exact-turn observation | `codex.app-server` | realized in g03.027; missing status remains exact-attribution `Unknown` |
| supported session-scoped observation | `opencode.http` | realized in g03.027 |
| supported exact-turn observation | `kimi-code.local-server` | realized in g03.029 with a persisted operation checkpoint and finite cursor replay |
| retained-operation candidate | `openai.background`; `anthropic.managed-agent` | persist a strict route-bound provider operation/resource recovery record before dispatch can be lost |
| history-snapshot candidate | `claude-agent.acp`; `kimi-code.acp` | qualify load-after-process-loss as read-only reconciliation and prove no prompt, callback, or control side effect |
| durable-transcript candidate | `gemini-cli.headless` | bind one consumer turn to exact transcript terminal evidence without prompt replay |
| blocked pending durable provider identity | `cursor-agent.acp`; `gemini-cli.acp`; `grok-build.acp`; `pi.rpc`; `qwen.headless`; `antigravity.headless`; `alibaba.conversations` | expose and persist an exact route-qualified operation/session reference plus bounded status or history lookup |
| not applicable to current lifecycle | catalogue-only routes; stateless direct runs; connection-only continuation and realtime routes; attached/owned local model inference | provider work cannot outlive the selected operation attachment or no retained operation identity exists |

The classification is deliberately separate from the main solution feature
CSV. It records routes that can move, their current evidence strength, and the
next gate without adding permanent `No` columns to the conversion scoreboard.

## Promoted Decisions

- Contract 048 owns the read-only boundary.
- The runtime carries distinct attribution and state; Codex is the first exact
  turn mapping and OpenCode is the first session-scoped mapping.
- Session-scoped evidence cannot claim a terminal result.
- Reconciliation returns a bounded replacement snapshot with explicit
  completeness, not an append-only transcript delta.
- Observation never grants cancellation, callback, provider-request, prompt,
  resume, import, or child-control authority.
- Controlled shutdown preservation remains separate work. This tranche does
  not change ordinary handle close semantics.

## Sources

- T3 Code restart symptoms: `pingdotgg/t3code` issues 2173 and 2886
- OpenCode server API: `/session/status`, `/session/:id`, and
  `/session/:id/message`
- Swallowtail Contracts 017, 021, 022, 042, and 046
- qualified Codex thread catalogue and Kimi local-server corpora in this repo
