# 2026-08-02 ACP Stable Session List Codec

## Result

Card 055 is complete. Swallowtail now has a bounded, request-correlated common
ACP v1 `session/list` codec without granting list, load, resume, or deletion
authority to every ACP agent.

## Evidence

Research 094 rechecked the current official stable schema. The deterministic
corpus pins capability, request, response, pagination, session identity, cwd,
title, RFC 3339 update time, `_meta`, and the independently gated
`additionalDirectories` addition.

List request construction fails before encoding unless the exact negotiated
capability is present. Response projection requires the original JSON-RPC id,
the requested working resource, unique bounded session ids, bounded content,
valid timestamps, valid metadata, and bounded cursors. Provider errors,
cross-request responses, malformed shapes, duplicates, resource drift, and
oversized content fail closed.

Unknown metadata and additive fields remain stored as opaque bounded protocol
extensions. Their values are unavailable through accessors and absent from
`Debug` and diagnostics. No candidate, cursor, load binding, resume binding,
or lifecycle authority is minted by the codec.

## Validation

- `effigy validate:focused swallowtail-protocol-acp` passed 93 tests
- `effigy package:verify-affected swallowtail-protocol-acp` passed
- `git diff --check` passed
- no provider process, authentication flow, prompt, or broad suite ran

## Next

Execute card 056. Qualify Kimi Code ACP session catalogue and explicit import
against every maintained milestone, using the common codec without widening
the separate local-server route.
