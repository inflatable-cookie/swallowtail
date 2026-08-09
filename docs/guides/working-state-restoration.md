# Working-State Restoration

Use this facade when a consumer persisted an operation as active, lost its
runtime handle, and wants the strongest qualified route-local path back to a
usable state. New to the shared vocabulary? Read
[Key Concepts](key-concepts.md) first.

Do not choose between reconciliation and continuation recovery in the UI or
persistence layer. Restore the exact provider facade and route-specific input,
then call that adapter's `prepare_working_state_restoration`. Every supported
route returns `PreparedWorkingStateRestoration`:

```rust
let method = restoration.method();
let outcome = restoration.restore(host_services).await?;

match outcome {
    WorkingStateRestorationOutcome::SessionReconciled(observation) => {
        // Apply exact-turn or session-scoped provider truth.
    }
    WorkingStateRestorationOutcome::RunReconciled(observation) => {
        // Apply exact provider-run truth.
    }
    WorkingStateRestorationOutcome::SessionRecovered(recovered) => {
        // Merge bounded replay, keep the interrupted turn unresolved,
        // then continue through recovered.into_parts().1.
    }
    WorkingStateRestorationOutcome::SessionReattached(reattached) => {
        // Continue through the exact provider session. No replay was accepted
        // as transcript or terminal truth.
    }
    WorkingStateRestorationOutcome::SessionReplaced(replaced) => {
        // Continue in a fresh usable session. Provider context from the lost
        // session is gone and the interrupted turn remains unresolved.
    }
    WorkingStateRestorationOutcome::RealtimeSessionReplaced(replaced) => {
        // Continue through a new media connection. Audio, transcript,
        // response, buffer, rollover, and terminal state were not recovered.
    }
}
```

The facade is consumed by `restore`. It cannot dispatch twice.

## Reconcile Then Attach A Settled Session

Codex app-server, OpenCode HTTP, and Kimi local server also expose a stronger
two-phase path when the consumer needs a live session after observation. First
prepare the ordinary session and reconciliation independently. Then consume
the prepared reconciliation through `prepare_settled_session_restoration`,
supplying the prepared session and a new attachment request id.

The returned `PreparedSettledSessionRestoration` always reconciles first. It
attaches only for `Completed`, `Failed`, `Cancelled`, or
`InactiveUnresolved`. `Active`, `WaitingForProviderInput`, and `Unknown`
return `SettledSessionRestorationOutcome::Observed` without issuing an
attachment request.

Codex and OpenCode return `SettledSessionAttachment::Loaded`, preserving their
bounded ordered replay. Kimi local server returns
`SettledSessionAttachment::Resumed`; it carries no replay. An attachment-phase
failure retains the complete successful reconciliation in
`SettledSessionRestorationFailure`.

This path does not replace `prepare_working_state_restoration`. Use the common
facade for observation-only recovery across routes. Use the consuming
two-phase path only after the application has independently selected and
prepared one of these exact attachment-capable routes.

## Preparation

Preparation remains adapter-local because models, checkpoints, provider turn
references, access, and working resources are exact route evidence.

- Codex, OpenCode, and Kimi local server accept their existing session
  reconciliation input through `prepare_working_state_restoration`.
- OpenAI background and Anthropic Managed Agents accept their existing run
  reconciliation input through the same method name.
- A prepared Claude Agent ACP or Kimi ACP session accepts request id, exact
  resume binding, and interrupted consumer turn id through
  `prepare_working_state_restoration`.
- A prepared Alibaba retained conversation accepts the same identities through
  `prepare_working_state_restoration`; its binding is resource-free and its
  loaded session preserves provider state.
- A prepared Cursor or Grok ACP session accepts the same exact attachment
  dimensions and returns a live attachment without claiming complete replay.
- Prepared Antigravity continuation, Gemini ACP, Pi RPC, and Qwen continuation
  sessions accept the interrupted consumer turn id and return a fresh session.
- Prepared Anthropic and DeepSeek direct-continuation sessions, Ollama attached
  sessions, xAI WebSocket sessions, and ordinary Alibaba conversations accept
  the interrupted turn id and return a fresh session with context loss.
- Prepared OpenAI Realtime and Gemini Live sessions accept the interrupted turn
  id and return a fresh realtime media session with connection-state loss.

Consumers may store the resulting prepared operation behind one application
boundary. They must still persist the exact route identity and binding or
checkpoint needed to prepare it after restart.

## Strength

`WorkingStateRestorationMethod` is visible before execution:

| Method | Effect | Lost-operation truth |
| --- | --- | --- |
| `ProviderSessionReconciliation` | bounded read-only observation | exact turn or session-scoped |
| `ProviderRunReconciliation` | bounded read-only observation | exact provider run |
| `ProviderSessionContinuationRecovery` | stateful load, replay, and live attachment | unresolved |
| `ProviderSessionAttachmentRecovery` | exact live attachment; bounded pre-response replay discarded | unresolved |
| `FreshSessionReplacement` | new usable session with no prompt replay | unresolved; provider context lost |
| `FreshRealtimeSessionReplacement` | new usable media session with no media replay | unresolved; connection context lost |

The continuation and attachment variants intentionally expose no terminal
state accessor. Transcript shape or a terminal-looking provider message is not
terminal evidence. Replacement is not recovery of provider state.

## Prepared Reusable Routes

| Route | Prepared action after restart |
| --- | --- |
| Codex app-server, OpenCode HTTP | session reconciliation; optional settled reconcile then bounded load |
| Kimi local server | exact-turn reconciliation; optional settled reconcile then replay-free resume |
| Claude Agent ACP, Kimi ACP | complete continuation recovery |
| Cursor ACP, Grok ACP | exact attachment; replay discarded |
| Antigravity continuation, Command Code interactive, Gemini ACP, Oh My Pi RPC, Pi RPC, Qwen continuation | fresh replacement; context lost |
| Anthropic Messages, DeepSeek continuation, Ollama attached, xAI Responses WebSocket | fresh replacement; context lost |
| OpenAI Realtime, Gemini Live | fresh realtime replacement; connection context lost |

This table covers prepared interactive harness routes. Catalogue-only and
one-prompt headless routes do not automatically retry a prompt after restart.

## Hosted Retained Conversation

The separate `alibaba.conversations` retained profile also maps to
`ProviderSessionContinuationRecovery`. It retrieves exact conversation
metadata and complete bounded ordered items before returning one live loaded
session. The interrupted consumer turn remains unresolved. Ordinary close
preserves the conversation; deletion requires the separately prepared
management operation.

This hosted-direct mapping does not change the connected-harness route count.
The ordinary Alibaba delete-on-close profile maps to fresh replacement. Its
new remote conversation is still deleted on ordinary close. It does not
inherit retained replay, preservation, or management authority.

## Failure

Method selection is fixed during preparation. A failed reconciliation returns
that failure. It never falls back to ACP load, another credential, another
route, prompt replay, callback response, cancellation, or cleanup.

Routes without a prepared interactive restoration mapping fail during
route-specific preparation. Do not infer stronger support from provider
family, durable retention, session import, or ordinary load support.
