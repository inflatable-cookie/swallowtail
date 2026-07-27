# Kimi Cross-Transport Binding Import

Date: 2026-07-27
Roadmap: g02.020
Card: 063

## Changed

- added an optional opaque Kimi state-root identity to ACP preparation
- added ACP session import authority issued from one matching resume binding
- added exact local-server import-target snapshots and side-effect-free
  cross-transport preflight
- added one authenticated read-only target lookup before binding issuance
- issued a new local-server archive/restore binding without widening ACP
- covered attached and owned topology, identity drift, missing and archived
  targets, disconnect, cancellation, deadline, exact newer-version acceptance,
  redaction, and joined cleanup

## Boundary

A Kimi session id, list result, filesystem path, or integration-family match
cannot call the import surface directly. Import requires adapter-issued ACP
source authority plus the target snapshot from the selected prepared
local-server instance.

Lookup is identity evidence. It performs no prompt, load, resume, archive,
restore, delete, or provider-state transition. The imported binding retains
only the local-server route's qualified archive and restore capabilities.

## Evidence

- full Kimi adapter: 47 deterministic tests passed; one live installed probe
  remained separately gated and ignored
- strict Kimi Clippy passed
- Effigy Rust, formatting, docs, Northstar, and 22-route checks passed
- doctor remained at the pre-existing 32 findings
- the public-API gate reported the expected held-candidate difference; no
  baseline or publication state changed

## Next

Card 064 adds the distinct Kimi local-server interactive session driver over
the already qualified REST and WebSocket v2 corpus.
