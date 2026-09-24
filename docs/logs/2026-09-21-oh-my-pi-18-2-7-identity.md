# 2026-09-21 Oh My Pi 18.2.7 Identity

Research 345 froze official npm and GitHub `@oh-my-pi/pi-coding-agent@18.2.7`
before any claim moved. npm `latest` is `18.2.7`, published
`2026-09-21T03:12:39.903Z`. GitHub latest release is `v18.2.7`, published
`2026-09-21T02:13:02Z`, tag commit `d716bcf60ab0a2e7ece1fdf382c0d143fef1f307`.
`omp` is not on `PATH`; missing install is not a gap.

Published stables after the previous ceiling `18.1.22` are exactly `18.2.0`
through `18.2.7`. The 9-row ledger, per-hop shipped-tree inventory, and
mapped RPC source classifications live in
`crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.2.7/`.
Research 327's `18.1.22` identity reproduces. Selected JSONL wire files are
byte-identical. Remaining mapped hops are unmapped skill, login-secret,
backpressure-spool, LSP/DAP framing, import-path, or append-only argv
changes. `18.2.8` is the synthetic later-stable. `pi.package` `0.86.1` stays
separate.

Decision: compatible-extension of `18.0.0..=18.1.22` to `18.0.0..=18.2.7` on
`oh-my-pi.rpc-v2-v18.0.0`. Not a major-line reset, new public operation, new
driver/facade, or family flatten.
