# 129 Grok 1.0.x Identity Corpus

Status: promoted
Owner: Tom
Date: 2026-08-17
Card: g03 batch 233
Correction: 2026-08-17 operator rejected fail-closed; support via milestone

## Question

Is official stable Grok CLI `1.0.4` a new axis, a compatible `0.2`
UnverifiedNewer point, or a same-axis milestone that Swallowtail must
qualify?

## Method

Compared npm `@xai-official/grok@1.0.4`, local `grok --no-auto-update
--version`, `grok agent stdio --help`, the frozen
`0.2.114..=0.2.117` compatibility corpus, Research 127, and Contract 029's
major-line and upgrade rules.

No provider prompt, install, update, ACP initialize, or claim edit in card
233.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `1.0.4` (published 2026-08-13T20:20:06.229Z) |
| npm alpha | `1.0.5` (ignored) |
| npm integrity | `sha512-Nu3SFXTqwvCQr/LQFwrQYgngJhUQwX2h9ZSgzW4HowidjbPBWtMVO0xI88d2z6/zlDSNaT5YP/uk+2DthKQMsg==` |
| npm shasum | `7fc774bb08e0d8a9c5e9eb15712b46ee31dfe762` |
| gitHead | `d846eb93d94d603191984d97f5d9f48170e93c6a` |
| platform package | `@xai-official/grok-darwin-arm64@1.0.4` integrity `sha512-ddb7tn+7ygDCpqGAsw1ZQkirePoPa7bm91wxWVxv9ePqIyrOiaDBluE3NMSjt2JMwqbcUbMtmg0CMKVb4N9oHw==` |
| local CLI | `grok 1.0.4 (d846eb93d94d) [stable]` |
| local executable | `/Users/tom/.grok/downloads/grok-1.0.4-macos-aarch64` SHA-256 `39366f7756a090b735cc1df8c93a8c0c3c7871555cf6cbb28f9351ca82936485` |
| last `0.2` stable | `0.2.121` (2026-08-05) |
| first `1.0` stable | `1.0.0` (2026-08-07) |

Invocation still advertised: `grok agent stdio`. Discovery still uses
`--no-auto-update --version`. Version text still matches
`grok <semver> (<hex>) [stable]`.

## Versus the `0.2` claim

Qualified window remains `0.2.114..=0.2.117` on axis `grok-build.executable`,
AllowUnverified. Discovery pins source revisions only for those four
qualified points. `1.0.4` therefore parses and currently classifies as
UnverifiedNewer, inheriting
`grok-build.acp-v1.cached-token-task-control-v2`.

That flattening is dishonest. A major-line reset is not a later `0.2`
stable. Contract 029 already names the fix: observe identity, freeze corpus,
then extend or add a milestone segment on the same axis.

Same package and `grok` bin are not a new axis. No ACP handshake was run in
card 233, so `1.0.x` is not yet a qualified mapping.

## Multi-version route shape

One axis. Ordered non-overlapping segments. Per-segment behavior revisions.
Gaps between segments are Incompatible. AllowUnverified only above the
latest qualified maximum. Codex and Qwen already use this for multi-window
routes.

After a `1.0` milestone lands, mid-gap `0.2.118..=0.2.121` become
Incompatible unless an explicit segment is added. Prefer leaving that gap
unless evidence demands those points; do not keep them as UnverifiedNewer by
flattening across the major reset.

## Segment decision for card 234

Support. Same axis. Not UnverifiedNewer. Not fail-closed. Not a new axis.

Card 234 qualified exact `1.0.4` through ACP handshake evidence and added
milestone behavior `grok-build.acp-v1.cached-token-model-4-6-v3` (model
`grok-4.6`). Kept `0.2.114..=0.2.117`. Raised latest qualified to `1.0.4`.
No new public operation.
