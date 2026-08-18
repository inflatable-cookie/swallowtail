# 2026-08-17 Grok 1.0.4 Milestone Claim

## Result

Card 234 qualified exact Grok CLI `1.0.4` on `grok-build.executable`. ACP
handshake confirmed `cached_token` activation, `session/new`, model
`grok-4.6`, and effort `xhigh`. Production claim now keeps deprecated
`0.2.114..=0.2.117`, adds maintained exact `1.0.4` behavior
`grok-build.acp-v1.cached-token-model-4-6-v3`, and leaves mid-gap
`0.2.118..=0.2.121` / unprobed `1.0.0..=1.0.3` incompatible. Resume and
continuation recovery stay unqualified.

## Validation

- `effigy validate:focused swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-grok`
- `effigy qa:northstar` plus named research/log/roadmap indexes

## Next

Reassess remaining Research 127 families one at a time. Rank after Grok:
exact-pin host drift, then AllowUnverified cluster. Gemini stays deferred.
