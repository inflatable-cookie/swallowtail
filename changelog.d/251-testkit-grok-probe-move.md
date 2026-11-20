Moved the Desktop Grok ACP client-MCP probe out of provider-neutral
`swallowtail-testkit` and into `swallowtail-adapter-grok`, behind that crate's
`live-probes` feature: the probe module, its fixture test target and both
harness examples. `swallowtail-testkit` no longer depends on
`swallowtail-protocol-acp` and carries `serde_json` only as a dev-dependency,
so non-ACP adapter test builds stop compiling the probe, `serde_json` and the
ACP protocol crate. The probe's offline fixture tests keep running through
`validate:grok-mcp-probe-helper-path`, which now selects the Grok package with
that feature. The `0.6.0` public-API baseline records the removal from
`swallowtail-testkit` and the additive probe module in
`swallowtail-adapter-grok`.

`run_all_synthetic_profiles()` proves the shared conformance contract once and
records it on every profile report, instead of repeating the same fourteen
assertions in each of the thirteen profile runs. Every individual
`run_*_profile()` entry point still proves and reports it.

`scripts/check-public-api.sh` now applies its zero-removal rule only on patch
bumps, where the previous tag's items are immutable. A pre-1.0 minor bump may
remove public items under Contract 036, and the gate's exact working-baseline
snapshot diff is what reviews each removal.
