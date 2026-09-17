# 2026-09-17 Goose ACP 1.50.1 Identity

Research 328 re-probed the official Goose GitHub release channel. `v1.50.1`
is the published stable at tag commit
`881f96c00d618ab6fca9b2aaa3a0abf07673cc2c`; the Darwin ARM64 archive digest
is `951055fac48e50a2087178307a75a1285f25f6a81675b84974ffc4fe6abe0d62`.

The selected 26-file ACP closure remains byte-identical from `1.50.0` to
`1.50.1` except for the unmapped MCP protocol-version default in
`Cargo.toml` and `crates/goose/src/agents/agent.rs`. The downloaded source was
not executed and no provider prompt was sent. The currentness fixture freezes
the identity and selected-file hashes.
