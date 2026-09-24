# Claude Code 2.1.280 identity stop

Frozen against npm `@anthropic-ai/claude-code@2.1.280` (`latest`, not the
older `stable` dist-tag `2.1.267`) and GitHub `v2.1.280`. The only published
stable after the qualified `2.1.278` ceiling is `2.1.280`. `2.1.279` is
unpublished on npm and GitHub. First unpublished stable after official latest
at observation: `2.1.281`.

Host `claude` was not on `PATH`. Missing host install is not a gap and was
not installed. Downloaded official `2.1.278` and `2.1.280` binaries were
hashed and never executed.

Wrapper files other than `package.json` are byte-identical, including
`sdk-tools.d.ts`. Platform packages change `claude` and `package.json` only.
Selected flag names, placeholders, choices, and the stream-JSON init object
stay. `--safe-mode`, which the response-only route passes, does not: `2.1.278`
returns before any plugin-hook registration, and `2.1.280` keeps sources
ending in `@builtin` and continues registration. That is a selected-surface
change, so neither axis is raised.

`identity.json` records publication identity and the stop. `protocol.json`
records the mapped-surface classification. `dist-inventory.json` records the
wrapper and platform file delta. No production claim was edited.
