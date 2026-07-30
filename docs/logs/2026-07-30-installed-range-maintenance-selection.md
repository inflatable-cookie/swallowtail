# Installed Range Maintenance Selection

Date: 2026-07-30
Status: completed

## Changed

- compared installed and current stable harness versions
- retained Grok, Kimi, and Claude at their current qualified points
- selected Codex `0.146.0` and OpenCode `1.18.5..=1.18.10`
- promoted Research 071
- compiled roadmap g02.044 and cards 146-149

## Decision

This is batched Contract 029 maintenance, not a provider-release treadmill.
Later upstream stable points remain executable as visible unverified newer and
do not force a Swallowtail release.

Codex `0.146.0` has exact additive activity drift already frozen by Research
064. OpenCode's selected HTTP/SSE and lifecycle source is unchanged through
`1.18.10`; exact `1.18.8` adds then `1.18.9` removes one unrelated OAuth
callback field from the full OpenAPI artifact.

No contract, common API, live provider prompt, install, update, consumer edit,
or publication is required.

## Next

Execute card 147's Codex `0.146.0` range extension.
