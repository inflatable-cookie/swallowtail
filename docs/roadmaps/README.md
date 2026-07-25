# Roadmaps

Roadmaps sequence work after vision, architecture, and contracts provide enough
shape.

## Current Generation

- [g02 Swallowtail Stabilization And Release Discipline](g02/README.md) — active
- [g01 Swallowtail Foundation](g01/README.md) — completed
- [Generation Index](generation-index.md)
- [Long-Term Plan](long-term-plan.md)

## Next Task

Return the canonical `0.1.0` candidate to the publication decision. The
operator must name the exact crates.io owner username and explicitly authorize
the desired bounded mutations: push the local canonical-history commits to
`origin/main`, publish the 23 crates sequentially in the recorded three-stage
order, create and push `v0.1.0` at candidate source commit
recorded in `release-candidates/0.1.0/candidate.env`, and create the GitHub
release. Workflow, owner-team, and consumer changes remain out of scope unless
separately authorized.

## Index

- `generation-index.md` — generation status
- `long-term-plan.md` — staged multi-consumer adoption
- `backlog/README.md` — deferred work and promotion gates
- `g01/README.md` — completed foundation generation
- `g02/README.md` — active stabilization, provider-wide facade, and release
  runway

## Generation Shape

Generations normally collect 30-50 numbered roadmaps. Batch cards sit inside
those roadmaps and do not count toward the generation range. A phase boundary
does not imply a generation rollover.
