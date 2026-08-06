# 2026-08-05 v0.1.0 Source Release Readiness Audit

## Result

Swallowtail is functionally healthy but not ready to tag.

Passing evidence:

- 27 packages and 33 production routes
- full QA: 1,459 tests passed across 136 binaries
- Rust 1.90 non-Bedrock floor passed
- Rust 1.94.1 Bedrock floor passed
- complete route and feature guide coverage

Release blockers:

- stale crates.io, package-count, MSRV, and candidate authority
- vulnerable legacy Bedrock TLS dependency path
- 5,897 missing public Rust documentation warnings and a non-semantic API
  baseline
- no concise source-install front door, current release copy, dependency
  policy, security policy, or CI

## Decision

Contract 036 now selects an annotated GitHub source tag `v0.1.0`. It excludes
crates.io publication and a GitHub Release object. All release mutations remain
separately authorized.

Research 111 and roadmap g03.043 own the repair. Card 125 is ready.

## Effects

No code, manifest, consumer, provider, credential, tag, branch, remote, or
publication effect ran.
