# Roadmaps

Roadmaps sequence work after vision, architecture, and contracts provide enough
shape.

## Current Generation

- [g03 Compatibility Maintenance And Consumer-Proven Hardening](g03/README.md) — active
- [g02 Swallowtail Stabilization And Release Discipline](g02/README.md) — completed
- [g01 Swallowtail Foundation](g01/README.md) — completed
- [Generation Index](generation-index.md)
- [Long-Term Plan](long-term-plan.md)

## Next Task

Prepare the exact `v0.3.0` source candidate after the operator-authorized
breaking release selection. Commit and push the prepared version, changelog,
release notes, and candidate state; require exact-commit CI before separate
tag authorization. The release keeps 28 packages, 34 routes, Rust `1.95.0`,
and source-only distribution. Its breaking boundary is the fail-closed
`Option<InterfaceVersionBinding>` return from the Codex and Ollama binding
helpers.

## Index

- [generation-index.md](./generation-index.md) — generation status
- [long-term-plan.md](./long-term-plan.md) — staged multi-consumer adoption
- [backlog/README.md](backlog/README.md) — deferred work and promotion gates
- [g01/README.md](g01/README.md) — completed foundation generation
- [g02/README.md](g02/README.md) — completed stabilization, provider-wide
  facade, activity, compatibility, and lifecycle generation
- [g03/README.md](g03/README.md) — active compatibility-maintenance and
  consumer-proven hardening generation

## Generation Shape

Generations normally collect 30-50 numbered roadmaps. Batch cards sit inside
those roadmaps and do not count toward the generation range. A phase boundary
does not imply a generation rollover.
