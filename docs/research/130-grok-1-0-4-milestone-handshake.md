# 130 Grok 1.0.4 Milestone Handshake

Status: promoted
Owner: Tom
Date: 2026-08-17
Card: g03 batch 234

## Question

Does installed official Grok CLI `1.0.4` map as a same-axis milestone on
`grok-build.executable`, and what behavior revision does it require?

## Method

Reused Research 129 identity. Ran one ACP handshake on the local
`grok 1.0.4 (d846eb93d94d) [stable]` executable:

- `--no-auto-update agent stdio`
- `initialize`
- `authenticate(cached_token, headless=true)`
- `session/new` in an empty temporary cwd

No provider prompt. Account metadata discarded. Session id not retained in
repository evidence.

## Handshake Facts

- ACP protocol version `1`
- agent version `1.0.4`
- `loadSession` and embedded context present
- auth methods `cached_token`, `grok.com`; default `cached_token`
- authenticate succeeded without interactive login
- `session/new` succeeded; stderr empty
- model `grok-4.6` only
- efforts `xhigh`, `high`, `medium`, `low`
- session capabilities advertise `list`, `resume`, and `close`

## Versus `0.2.117`

Same transport, access profile, and activation method. Material deltas:

- default model `grok-4.5` → `grok-4.6`
- effort set gains `xhigh`
- session list/resume/close advertised (not qualified here)

## Decision

Qualify exact `1.0.4` as Maintained milestone
`grok-build.acp-v1.cached-token-model-4-6-v3`. Keep
`0.2.114..=0.2.117` as deprecated segments. Leave mid-gap
`0.2.118..=0.2.121` and unprobed `1.0.0..=1.0.3` Incompatible. Later stables
above `1.0.4` remain AllowUnverified. Do not qualify resume or continuation
recovery from the new advertisements.
