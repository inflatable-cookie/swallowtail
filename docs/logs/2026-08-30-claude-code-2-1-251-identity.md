# 2026-08-30 Claude Code 2.1.251 Identity

## Result

Card 017 froze official npm `@anthropic-ai/claude-code` `2.1.251` against
the `2.1.241` claim. Host `claude` matches official darwin-arm64
`625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5`.
Wrapper files `cli-wrapper.cjs` and `install.cjs` are byte-identical to
`2.1.241`. Official darwin-arm64 `--help` is not byte-identical to
`2.1.241`; selected mapped stream-JSON flags stay. Help extras
`--restricted`, `attach`, `logs`, `stop`/`kill`, `respawn`, and `rm`
stay unmapped. Published intermediates are `2.1.242`, `2.1.243`,
`2.1.245`, `2.1.246`, `2.1.247`, `2.1.248`, and `2.1.250`. Unpublished
gaps are `2.1.244` and `2.1.249`. Production claims stayed at `2.1.241`
in this card. Decision for card 018: compatible extension of both
existing stream-JSON behaviors through `2.1.251`.

## Next

Raise both qualified ceilings on card 018.
