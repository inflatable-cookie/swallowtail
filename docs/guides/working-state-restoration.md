# Working-State Restoration

Use this facade when a consumer persisted an operation as active, lost its
runtime handle, and wants the strongest qualified route-local path back to a
usable state.

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
}
```

The facade is consumed by `restore`. It cannot dispatch twice.

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
- A prepared Cursor or Grok ACP session accepts the same exact attachment
  dimensions and returns a live attachment without claiming complete replay.
- Prepared Antigravity continuation, Gemini ACP, Pi RPC, and Qwen continuation
  sessions accept the interrupted consumer turn id and return a fresh session.

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

The continuation and attachment variants intentionally expose no terminal
state accessor. Transcript shape or a terminal-looking provider message is not
terminal evidence. Replacement is not recovery of provider state.

## Connected Harness Routes

| Route | Prepared action after restart |
| --- | --- |
| Codex app-server, OpenCode HTTP, Kimi local server | session reconciliation |
| Claude Agent ACP, Kimi ACP | complete continuation recovery |
| Cursor ACP, Grok ACP | exact attachment; replay discarded |
| Antigravity continuation, Gemini ACP, Pi RPC, Qwen continuation | fresh replacement; context lost |

This table covers prepared interactive harness routes. Catalogue-only and
one-prompt headless routes do not automatically retry a prompt after restart.

## Failure

Method selection is fixed during preparation. A failed reconciliation returns
that failure. It never falls back to ACP load, another credential, another
route, prompt replay, callback response, cancellation, or cleanup.

Routes without a prepared interactive restoration mapping fail during
route-specific preparation. Do not infer stronger support from provider
family, durable retention, session import, or ordinary load support.
