# 2026-08-17 Grok 1.0.x Identity Corpus

## Result

Card 233 froze official Grok CLI `1.0.4` against the `0.2.114..=0.2.117`
claim. npm `@xai-official/grok@1.0.4` is still latest. Local CLI is
`grok 1.0.4 (d846eb93d94d) [stable]`. Same package, `grok` bin, and
`agent stdio` invocation. Major line reset from `0.2.121` to `1.0.0` on
2026-08-07.

AllowUnverified would currently classify `1.0.4` as UnverifiedNewer. That
flattening is rejected. Not a new axis. Production claims stayed at
`0.2.117` in card 233.

## Correction

Operator rejected fail-closed. Support path is Contract 029 multi-segment
claims: keep `0.2.114..=0.2.117`, add a `1.0` milestone segment after ACP
handshake evidence. Same pattern as Codex/Qwen multi-window routes.

## Next

Qualify `1.0.4` on card 234 (handshake corpus + milestone claim).
