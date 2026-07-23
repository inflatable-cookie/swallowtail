# DeepSeek V4 Production Conformance Closeout

Date: 2026-07-22

## Changed

- added the separately registered `swallowtail.deepseek.direct` production
  driver with catalogue and locally continued direct-session roles
- added host-approved bearer API-key HTTP and SSE transport for exact
  `/models` and `/chat/completions` routes without `/v1`, redirect, proxy,
  retry, alias, facade, model, or credential fallback
- added a distinct direct-tool exchange: Swallowtail reports one bounded call,
  the consumer executes it, and only the exact correlated result authorizes
  the next provider attempt
- added bounded zeroizing private reasoning and provider-message history,
  same-route replay, per-attempt usage/cache/finish/request evidence, and
  credential-last cleanup
- added deterministic production and common-profile conformance under local
  and remote-authoritative execution hosts
- closed roadmap 036 and opened roadmap 037 with card 105 as the sole next
  task

## Evidence

The exact corpus produces one buffered tool attempt, one SSE final attempt,
then one later-user SSE attempt. Captured requests match all three frozen JSON
documents. No second provider request occurs while the consumer tool result is
pending. A third user turn rejects before effects.

Account-concurrency failure, disconnect, active-stream cancellation, and a
deadline during tool wait keep distinct terminal truth. Stream and blocking
work join before the session credential is released. Raw reasoning, prompts,
tool arguments, tool results, credentials, and provider error payloads do not
enter stable diagnostics or debug output.

DeepSeek exposes cache hit and miss counts in the selected response usage, so
the driver emits them per attempt. The selected fixture exposes request ids
and account-concurrency failure but no trustworthy remaining-capacity or
account-balance observation. The driver therefore does not fabricate rate-
remaining or quota evidence. Catalogue presence still does not prove
entitlement, balance, or invocation capacity.

## Validation

- focused DeepSeek suite: 15 tests pass
- focused runtime and DeepSeek warnings-denied clippy passes
- full `effigy qa` passes
- workspace inventory: 489 tests; 486 run and pass, three gated probes ignored
- `effigy doctor` retains the inherited 19 findings: seven errors and twelve
  warnings; the new files add no oversized-file debt
- `git diff --check` passes

## Next

Card 105 is ready. Refresh the twenty-route coverage inventory, classify exact
pins versus maintained compatibility windows, revalidate current candidates,
and select one bounded next lane only when authority and information gain are
clear.
