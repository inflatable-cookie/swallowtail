# Research 328: Goose ACP 1.50.1 Failure-Binding Reopen

Status: complete
Date: 2026-09-17
Route: `goose.acp`
Task: g05.081

## Finding

The official Goose release channel is at `v1.50.1`, published
2026-09-14T21:00:01Z. The GitHub tag resolves to commit
`881f96c00d618ab6fca9b2aaa3a0abf07673cc2c`. The Darwin ARM64 release archive
has SHA-256
`951055fac48e50a2087178307a75a1285f25f6a81675b84974ffc4fe6abe0d62`.
Identity was checked against the official GitHub release and tag endpoints;
the source archive was retrieved for deterministic comparison and was not
executed.

The selected ACP source closure was compared at every stable point from the
`1.46.0` baseline through `1.50.1`. The historical `1.46.0..1.47.0` change
remains the selected failure-binding boundary: provider authentication moves
from generic error text plus `end_turn` to typed `auth_required` on both
`session/new` and `session/prompt`. The 1.50.0 to 1.50.1 patch changes only
the MCP protocol-version default and initialization in `Cargo.toml` and
`crates/goose/src/agents/agent.rs`. The selected ACP server, dispatch,
session-new, prompt, and conversation-message sources are byte-identical
across that patch hop. Because this route always sends `mcpServers: []`, the
MCP default change is unmapped.

## Claim decision

The adapter binds the typed failure semantics on a new private behavior
revision, `goose.acp.stdio-v2.auth-required`. Its exact `QualifiedOnly` point
is `1.50.1`; no range or unverified-newer posture is inferred. The adapter
maps the exact typed `auth_required` message to
`swallowtail.goose.acp.auth_required` while retaining the existing
provider/model-resolution diagnostic for other failures. Builtin, mode,
lifecycle, permission, process, and advertised sibling surfaces remain
independently gated.

The provider-free result is frozen in
`crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.50.1/currentness.json`.
The fixture records release identity, the compared stable sequence, the
failure binding, the patch classification, and SHA-256 values for the changed
and selected ACP sources. It records that no provider prompt or downloaded
artifact execution occurred.

Evidence sources:

- [Goose v1.50.1 release](https://github.com/aaif-goose/goose/releases/tag/v1.50.1)
- [Goose v1.50.1 tag](https://github.com/aaif-goose/goose/tree/v1.50.1)
- Research 319: [Goose ACP 1.50.0 identity stop](./319-goose-acp-1-50-0-identity-stop.md)
