# Antigravity CLI 1.2.11 Retry-Pin Evidence Fixture

Machine-checkable companion to Research 357 (swallowtail#069, Q-003
option 2). The `1.2.11` artifact digests reproduce the frozen
`antigravity-cli-1.2.11` identity corpus; the pin semantics
(`AGY_CLI_MODEL_API_MAX_RETRIES`, `ParseUint(_, 10, 32)` into
`ModelAPIRetryConfig.max_retries`, `0` disables, invalid falls back to the
compiled default of 8 retries) were proven by bounded static analysis plus
loopback execution with no provider call.

`pin-evidence.json` freezes the attempt counts the claim depends on:
`N+1` attempts for pin `N` on every tested retryable class
(502, 503, 504, 429, mid-stream EOF), one attempt for `0` and for terminal
failures, nine for unset/empty/invalid. The gateway URL override was
measurement apparatus only and is not part of the approved environment.
