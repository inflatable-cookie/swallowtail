# 2026-07-21 Kimi Platform Conformance And Closeout

## Outcome

Card 086 and roadmap 028 are complete. The production Kimi Platform K3 route
passes the unchanged hosted-direct conformance boundary under both supported
execution-host topologies.

## Proof

The local and remote-authoritative fixtures preserve exact execution host,
configured instance, endpoint target, public-platform API-key audience,
catalogue source, provider, route, and model identity. Catalogue discovery
remains a source-scoped observation; it grants no K3 entitlement, route, or
invocation-readiness claim.

The exact successful run performs one inference attempt and emits separate
reasoning, output, and terminal usage. It has no retry, reconnect, fallback, or
detached work. Both endpoint tasks finish before their credential leases are
released, and operation close leaves no owned work.

## Validation

Fifteen Kimi Platform tests pass, including three new conformance tests. The
combined Kimi Platform, compatible-chat, llama.cpp, and testkit round passes 93
tests. Focused warnings-denied clippy passes. Full repository QA passes with
384 tests; the Gemini, Kimi Code, and OpenCode installed or live probes remain
separately gated and ignored by default.

Doctor remains at the inherited 19 oversized-file findings: 12 warnings and
seven errors. No new file from this batch is reported. `git diff --check`
passes.

## Continuation

Roadmap 029 and cards 087-089 keep g01 active. Card 087 is the sole ready task:
revalidate DeepSeek, Z.AI, and Alibaba Model Studio, then select one exact next
direct route. No provider implementation is authorized by stale evidence or
wire compatibility alone.
