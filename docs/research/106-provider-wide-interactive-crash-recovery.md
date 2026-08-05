# 106 Provider-Wide Interactive Crash Recovery

Status: promoted
Owner: Tom
Date: 2026-08-05

## Question

Can every connected harness interactive route offer one truthful action after
a consumer crash without retrying provider work or inventing continuity?

## Finding

Yes, through a five-strength ladder:

1. exact provider-session reconciliation
2. exact provider-run reconciliation
3. complete load/replay continuation recovery
4. exact provider-session attachment with non-authoritative replay discarded
5. a fresh replacement session with provider context explicitly lost

The last two are distinct. Attachment keeps the exact provider session but
cannot restore transcript truth. Replacement restores only route usability.
Neither settles the interrupted turn.

## Interactive Harness Classification

| Route | Strongest restart action | Provider context |
| --- | --- | --- |
| `codex.app-server` | session reconciliation | observed exactly or session-scoped; later settled attachment |
| `opencode.http` | session reconciliation | observed session-scoped; later settled attachment |
| `kimi-code.local-server` | exact-turn reconciliation | observed exactly; later replay-free resume |
| `claude-agent.acp` | continuation recovery | complete bounded load replay |
| `kimi-code.acp` | continuation recovery | complete bounded load replay |
| `cursor-agent.acp` | attachment recovery | exact session; replay indeterminate and discarded |
| `grok-build.acp` | attachment recovery | exact session; replay indeterminate and discarded |
| `antigravity.headless` continuation | fresh replacement | lost |
| `gemini-cli.acp` | fresh replacement | lost pending separate attachment qualification |
| `pi.rpc` | fresh replacement | lost pending stored-cwd qualification |
| `qwen.headless` continuation | fresh replacement | lost |

Catalogue-only and one-prompt headless routes expose no connected interactive
handle. Automatically rerunning their last prompt could duplicate tools,
writes, usage, or billing, so they remain outside the action facade.

## Cursor And Grok

Cursor source proves that exact session initialization precedes retained
history replay, while history-read and per-turn replay failures are suppressed.
Grok artifacts prove a retained session load path and replay drain but not
complete replay. Those defects block `LoadedSession`; they do not block an
explicit bounded discard phase which returns only the exact live attachment.

Both routes must issue a durable attachment binding from ordinary session
creation. Model-less prepared routes bind the exact absence of selectable
model identity rather than inventing a synthetic model.

## Decisions

- Contracts 017 and 050 own the expanded ladder.
- Method selection remains static before provider work.
- Failure never falls through to a weaker action.
- Discarded provider replay is never a transcript or terminal-state source.
- Replacement never replays a prompt or consumer history.
- Every interactive harness route gets one prepared action; stronger routes
  retain their stronger meaning.
- One-shot routes get no automatic retry.

## Sources

- Research 099, 104, and 105
- Contracts 017, 042, 048, and 050
- exact Cursor `2026.07.01-41b2de7` and `2026.07.23-e383d2b` source bundles
- exact Grok Build `0.2.114..=0.2.117` artifacts
- current prepared interactive route matrix
