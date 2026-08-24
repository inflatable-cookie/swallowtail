# 204 Grok Build ACP Reasoning-Selection Evidence

Status: reserved
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Card: g04.057 / 158

## Question

Can exact `grok-build.acp` sessions and operation-private structured runs map
portable `ReasoningSelection` through Grok Build's ACP configuration channel,
with exact model/value/version qualification and confirmed effective selection
before the first prompt?

## Current Boundary

The route fixes `grok-4.5` on deprecated `0.2.114..=0.2.117` and `grok-4.6`
on maintained `1.0.4..=1.0.5`. Existing secret-free handshake evidence records
`low|medium|high` for the former and adds `xhigh` for the latter. Prepared
interactive sessions reject non-empty `SessionOptions`; structured runs expose
no reasoning input. Neither advertisement nor current official CLI spelling
proves the ACP option id, request payload, application lifetime, or effective-
value confirmation required by Contract 034.

## Evidence Required

Card 158 must freeze current official documentation, exact published package
or source evidence, and the existing no-prompt handshake corpus. It must name:

- the exact option snapshot and lifecycle on which effort appears
- provider option id, value encoding, ordering, defaults, and model binding
- the one `session/set_config_option` request shape, response, and any
  `config_option_update` confirmation
- exact new-session support for interactive and operation-private run shapes
- omission, unsupported values, ambiguity, drift, and post-allocation failure
- whether later stable `UnverifiedNewer` points may inherit the latest
  qualified private mapping
- the exact behavior, driver, claim, and configured-instance revision posture

No value is deliver-now until that evidence is frozen. Existing no-prompt
handshakes, public documentation, and secret-free exact package/source
inspection are allowed. Authentication, account inspection, a provider prompt,
credential capture, installation, or paid work is not authorized.

## Decision Pending

Replace this reservation with a source-backed version/model/value and
application-state table. An empty deliver-now set is an acceptable result and
blocks cards 159-160 without weakening the current route.
