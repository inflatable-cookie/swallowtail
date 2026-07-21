# 2026-07-20 Kimi Native macOS Containment Selection

## Decision

Select a deployment-owned native macOS App Sandbox launcher/helper as Kimi's
first complete process-filesystem containment proof. Do not use a container or
VM for the first slice. Preserve the earlier operator sequence: active card 063
remains first, while the Kimi proof is ready but queued.

## Intended Experience

- the deploying app owns helper packaging, signing, entitlements, and lifecycle
- the user selects a project folder once through standard macOS interaction
- a persisted security-scoped bookmark restores that exact project authority
- Kimi runs with an isolated app-owned `KIMI_CODE_HOME`
- delegated Kimi login remains inside that isolated state and exposes no token
  to Swallowtail
- later sessions launch as ordinary native children without container images,
  mounts, daemons, or a second filesystem

## Evidence Delta

Apple's current documentation supports App Sandbox, externally built embedded
helpers, Developer ID distribution, inherited child restrictions, and
security-scoped persistent file grants. Static helper inheritance does not by
itself prove dynamic project-grant propagation. Card 057 must prove the exact
bookmark handoff through the launcher, Kimi, shell, and descendants before any
portable containment claim.

Current official Kimi evidence also supersedes the legacy Python provider pin.
The successor Kimi Code line retains `kimi acp`, supports an isolated data root,
and adds built-in shell, background, subagent, MCP, and plugin behavior. Card
065 now gates production mapping on a fresh exact successor corpus.

The planning snapshot selects current Kimi Code `0.26.0`, released 2026-07-16,
for revalidation. It is not a floating dependency. Official ACP evidence now
documents load and session listing, client file reads and writes, local shell
execution without terminal reverse RPC, MCP forwarding, and one unstable model
method. Card 065 must pin ACP, SDK, schema, wire, executable, and source
versions separately, exclude the wider surface, and reject self-upgrade or
unknown executable versions before provider work.

## Roadmap State

- roadmap 018 moves from paused to planned
- cards 057 and 065 are ready but queued
- cards 058-059 remain blocked by those proofs
- card 063 remains the sole active `Next Task`

## Risks

- the security-scoped bookmark may not propagate through the proposed launcher
  shape without an explicit helper handoff
- project executables, build tools, shell children, and background descendants
  may expose App Sandbox incompatibilities
- bundling or resolving Kimi needs exact artifact, signature, update, and
  license evidence
- isolated OAuth state needs a first-run flow without importing ambient
  `~/.kimi-code` credentials
- the successor ACP surface may differ materially from the legacy fixtures

## Next

Continue card 063. After the owned llama.cpp checkpoint, execute cards 065 and
057 before Kimi driver card 058.
