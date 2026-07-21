# 2026-07-21 Kimi App Sandbox Runtime Stop

## Changed

- proved real `NSOpenPanel` selection, persisted security-scoped bookmark
  creation, regular bookmark handoff, and inherited App Sandbox access
- passed twelve direct, shell, nested-shell, background-child, outside-root,
  link, rename, and network assertions under the documented helper signature
- tested the exact pinned Kimi Code `0.28.1` arm64 executable under both the
  documented two-entitlement helper signature and a diagnostic combined
  dynamic-code signature
- confirmed both Kimi variants terminate during V8 initialization
- confirmed JITless is not a viable escape: Kimi stalls while loading an
  extracted ad-hoc native module, and the exact child and parent were joined
- promoted Research 011 and tightened Contract 017 around exact executable and
  dynamically loaded code compatibility

## Current State

No portable containment records, host service, or production Kimi driver were
added. The platform bookmark boundary works, but the exact provider artifact
does not qualify under Apple's supported inherited-helper model. Roadmap 018
and cards 057-059 are blocked.

The recommended seamless direction is to retain the deterministic ACP corpus
and wait for or obtain a Moonshot-supported sandbox-compatible macOS artifact
or packaging route. A custom native build, app-hosted runtime, XPC design,
container, VM, weaker access claim, or switch to another provider is an
operator decision.

## Validation

- the concrete descendant probe passed all 12 assertions
- Kimi exited with signal 5 in both entitlement variants
- the JITless diagnostic child and parent were terminated and observed
- full repository QA passes with 259 tests; docs, Northstar, formatting, lint,
  compile, and diff checks pass
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  7 errors
