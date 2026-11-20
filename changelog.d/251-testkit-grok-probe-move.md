Moved the Desktop Grok ACP client-MCP probe out of provider-neutral
`swallowtail-testkit` and into `swallowtail-adapter-grok`, behind that crate's
`live-probes` feature: the probe module, its fixture test target and both
harness examples. `swallowtail-testkit` no longer depends on
`swallowtail-protocol-acp` and carries `serde_json` only as a dev-dependency,
so non-ACP adapter test builds stop compiling the probe, `serde_json` and the
ACP protocol crate. `run_all_synthetic_profiles()` proves the shared
conformance contract once and records it on every profile report instead of
repeating the same fourteen assertions per profile.
