# 299 Claude SDK Live MCP Startup Failure

Status: complete; promoted into g05 Card 150
Owner: Tom
Created: 2026-09-08

## Finding

Desktop Card 314 did not exercise the first Claude query. Its single open
failed before provider readiness with route code
`swallowtail.claude-agent.sdk.open_rejected`, stage `sidecar_rejected`, and
bounded subcode `mcp_server_failed`. The session and turn never existed; the
authorized prompt digest was recorded but not submitted.

This is a material registered-MCP startup defect. The release requires the
same MCP/tool route, so removing MCP from the qualification suite would evade
the failure rather than fix it.

## Frozen Evidence

Desktop task `d76fe0f1-3a54-4116-80f8-1b55c2ed2d39`; PR 177; accepted head
`ff3a28c23ab90924b195edb2e7d42e6071dcb6b9`; review comment `5592884677`;
merge `f29eb0c01ce682da626c7e365a519d23b2c52ca6`; canonical closeout
`5323e088bb6d9781293e7a1a8497bb3f7d1f03d1`.

Capsule SHA-256:
`f9cd0b6f73c1ab89d322ae6d15d46808121db180937f42328ecc47dc0e428877`.
Exact source-linked Swallowtail:
`0d120067cd260b1f5127835cab0b1a3ad020a29d`.

The tuple matches Card 312's successful registered-MCP open: SDK `0.3.259`,
native `2.1.259` Darwin arm64, Node `22.23.2`, sidecar `0.4.4`, MCP
`2025-11-25`, `claude-sonnet-5`, default permission, persistence false, and the
mediated-stdio/private-loopback registered-tool carrier. This makes the
success/failure variance a startup-determinism question, not evidence that MCP
is unsupported.

## Leading Mechanism, Not Yet Proven

Desktop builds the courier at the shared
`target/debug/swallowtail-registered-tool-courier` path, then later gives that
path to the provider for spawn. Card 139 already measured Cargo removing and
recreating an analogous post-build courier path while a sibling build runs;
its fixture fix publishes immutable content-addressed spawn artifacts. The
Desktop runner does not apply that acquisition rule.

The same mechanism is the leading explanation, not a finding from Card 314:
the capsule exposes only MCP status, not courier spawn evidence. Card 150 must
reproduce or falsify it under provider-free target churn and inspect the whole
launch chain before selecting the owner.

## Disposition

Card 150 repairs the Desktop proof harness if its mutable build path is the
cause. It stops with an exact Swallowtail owner handoff if the production
carrier, host, or sidecar owns the defect. No Claude/provider execution or
account mutation is needed. Card 149 remains stopped; both Contract 061 cells
remain unqualified; top-up and release work stay held.
