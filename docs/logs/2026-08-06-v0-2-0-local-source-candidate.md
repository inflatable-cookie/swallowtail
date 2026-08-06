# v0.2.0 Local Source Candidate

Date: 2026-08-06
Roadmap: g03.046
Card: 140

## Outcome

Prepared the coordinated 28-package, 34-route `v0.2.0` source candidate. Muse
joins the package, route, and semantic API baselines. The workspace now has one
explicit Rust `1.95.0` floor. The MSRV raise is the release's intentional
breaking boundary; existing 27-package APIs and route identities remain
compatible.

The floor audit found and fixed two candidate defects:

- removing Bedrock's old override had removed Rust-version metadata instead of
  inheriting the workspace value
- the provider-wide activity corpus contained Muse but still asserted the old
  20-route, 26-profile totals

Historical `v0.1.x` package, route, dependency, toolchain, API, and release-note
evidence remains unchanged. New `v0.2.0` baselines carry the 28-package,
34-route, unified-floor candidate.

## Dependency Refresh

Cargo advances only the 28 coordinated workspace package identities from
`0.1.1` to `0.2.0`. It deliberately retains these available third-party
updates under the declared requirements and selected Rust `1.95.0` graph:

- `agent-client-protocol-schema` 1.5.0; 1.6.0 available
- `async-tungstenite` 0.34.1; 0.35.0 available
- `base64` 0.22.1; 0.23.1 available
- `generic-array` 0.14.7; 0.14.9 available
- `matchit` 0.8.4; 0.8.6 available
- `sha2` 0.10.9; 0.11.0 available

## Preparation And Validation

Effigy cannot atomically raise a coordinated workspace across a pre-1.0 minor
boundary: it updates the workspace package version before gates but does not
update internal dependency requirements. A bounded temporary
`>=0.1.1, <0.3.0` range admitted both preparation sides. Effigy then prepared
`v0.2.0` with all 11 gates passing. The final tree narrows every internal edge
to `^0.2.0`; this expected post-prepare drift makes the preparation fingerprint
stale. All 11 configured gates pass again on that final exact graph:

- semantic API, missing docs, metadata, and source-consumer proof
- unified Rust `1.95.0` floor, full-feature and no-feature lint
- format, complete nextest suite, route/docs QA, and dependency security

No authenticated provider, consumer, commit, push, workflow, tag, registry,
or GitHub Release mutation ran.

## Next

The operator authorized replacement of the obsolete split Rust-floor workflow.
Card 141 now needs the exact candidate commit and canonical CI proof.
