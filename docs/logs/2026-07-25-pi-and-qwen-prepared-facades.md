# 2026-07-25 Pi And Qwen Prepared Facades

Status: complete

## Changed

`swallowtail-adapter-pi` and `swallowtail-adapter-qwen` now expose separate
adapter-local prepared normal paths.

Both adapters probe one exact host-approved executable with `--version`, retain
the installed observation and access provenance, derive a configured instance
and immutable plan, expose typed prepared execution, and keep the low-level
driver callable.

Pi qualifies package 0.80.10. Qwen qualifies package 0.19.11. Later stable
releases proceed as visible unverified-newer observations using the latest
qualified behavior. Older and prerelease versions remain incompatible.

Current tagged source confirms Pi prints its bare package version for
`--version`; Qwen's tagged yargs configuration and maintained CLI
documentation expose the same bare version probe:

- [Pi 0.80.10 CLI source](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/src/main.ts)
- [Qwen Code 0.19.11 CLI configuration](https://github.com/QwenLM/qwen-code/blob/v0.19.11/packages/cli/src/config/config.ts)
- [Qwen Code CLI options](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/settings/)

## Native Boundaries

Pi prepares one long-lived RPC session with explicit provider and model,
provider-suppressed configuration, ambient read-only workspace authority,
prohibited durable provider state, and the restrictive RPC policy. Prompt,
steering, follow-up, abort, UI callback relay, interruption, and cleanup remain
on the unchanged low-level session and turn handles.

Qwen prepares one structured run with explicit provider, model, prompt,
working resource, and deadline. The adapter retains text stdin, streamed JSON,
read-only tool selection, durable provider retention, and the fixed
60-second, 16-tool-call, and 24-turn native bounds.

Neither `ProviderSuppressed`, Pi's disabled configuration sources, Qwen
`--safe-mode`, nor its tool exclusions imply sandboxing or containment. Both
routes remain `AmbientHost`.

## Public Surface

The additions are pre-1.0 additive facade APIs. No compatibility shim or
generic provider router was added. The held public declaration baseline was
refreshed for the accumulated provider-wide facade work and passes for all 23
crates.

## Validation

- Pi all-target suite: 20 deterministic tests pass
- Qwen all-target suite: 20 deterministic tests pass
- prepared paths pass local and remote-authoritative host identities
- qualified and unverified-newer observations remain distinct
- existing Pi scheduling/UI and Qwen native-budget/isolation suites remain
  green
- public examples compile
- full Effigy QA passes across the workspace
- 23-crate public API declaration gate passes
- Doctor remains at the known 19 oversized-file findings: 7 errors and 12
  warnings

## Next

Card 026 adds the attached OpenCode HTTP/SSE prepared facade and reviews remote
ACP only as an explicit composition route. Cards 026-036 remain in the active
g02 provider-wide facade and candidate-return runway.
