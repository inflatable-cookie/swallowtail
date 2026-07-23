# 2026-07-23 Codex Version Discovery And Range Drivers

## Changed

- Added independent maintained compatibility claims for Codex exec and
  app-server on the shared `codex.cli` executable axis.
- Added one bounded `codex --version` probe for both drivers.
- Added exact discovery-to-configuration-to-preflight promotion fixtures.
- Required one qualified exact version in every production execution plan.
- Added app-server behavior dispatch at `0.131.0`.
- Removed unconditional `allowProviderModelFallback: false` emission.
- Required explicit `experimentalApi` negotiation for dynamic tools, provider
  requests, and runtime workspace roots.

## Published Windows

- exec: `0.122.0..=0.145.0`, behavior `codex.exec.jsonl-v1`
- app-server base: `0.110.0..=0.130.0`, behavior
  `codex.app-server.v2.base`
- app-server workspace roots: `0.131.0..=0.145.0`, behavior
  `codex.app-server.v2.workspace-roots`

The two claims have distinct identities. Sharing one observed executable does
not merge their transport, facade, role, behavior, or maintenance evidence.

## Discovery

The probe executes only the opaque host-approved target with `--version`. It
uses no environment, working resource, credential, sign-in flow, model,
prompt, or fallback. Output is bounded to 128 bytes and must contain one exact
`codex-cli` semantic version.

The probe runs inside the host's scoped task. Success, incompatibility,
malformed output, cancellation, timeout, failure, and cleanup failure do not
return before child and task join. Safe outcomes contain no executable
reference or raw output.

## Behavior

Stable read-only sessions send no experimental field and do not enable
`experimentalApi`. Dynamic tools, observed provider requests, and bounded
workspace roots enable it explicitly. `0.130.0` rejects bounded workspace
access before resource or process work. `0.131.0` is the first accepted
workspace-root point.

## Validation

- 63 Codex adapter tests pass
- three focused runtime discovery tests pass
- focused warnings-denied clippy passes
- `git diff --check` passes
- god-file scan remains at the inherited 19 findings: seven errors and twelve
  warnings

## Next

Card 114 proves both windows across local and remote-authoritative topology,
range boundaries, behavior milestones, stable and experimental sessions, and
the unchanged public conformance profiles before full repository closeout.
