# Qwen Code v0.19.11 headless fixture

Frozen: 2026-07-21

Primary evidence:

- release `v0.19.11`, commit
  `f22cf5009ee3eb26b5c5de2eca6e1f1d0ffee0ad`
- `docs/users/features/headless.md`
- `packages/cli/src/config/config.ts`
- `packages/cli/src/nonInteractive/types.ts`
- `packages/cli/src/utils/errors.ts`
- `packages/core/src/tools/tool-names.ts`

Sources: <https://github.com/QwenLM/qwen-code/tree/v0.19.11> and
<https://qwenlm.github.io/qwen-code-docs/en/users/features/headless/>.

The corpus represents text stdin plus line-delimited stream-JSON stdout. Fatal
wall/tool budget, turn-limit, and signal cancellation paths are process exits;
v0.19.11 does not promise a terminal stream-JSON result for those paths. The
terminal-observation fixture therefore keeps exit truth separate from parsed
provider events.

The synthetic values `fixture-private-workspace`,
`fixture-provider-secret-never-diagnose`, and `fixture-private-prompt` exist
only to prove public diagnostics do not copy raw provider content. No real
credential, provider request, paid inference, executable, or container is
used.
