# OpenCode Range Corpus And Health Observation

Date: 2026-07-23
Card: `../roadmaps/g01/batch-cards/121-opencode-range-corpus-and-health-observation.md`

## Outcome

The OpenCode candidate range now has exact offline evidence from `1.14.48`
through `1.18.4`.

- 45 stable releases have exact tag commit, publication date, and OpenAPI
  SHA-256 evidence
- recursive closure from six selected operations produces 18 surface revisions
- 20 semantic segments preserve unpublished patch and cross-minor gaps
- every surface has one exact adapter-private behavior revision
- health observation retains only exact `opencode.server` binding and
  compatibility classification
- unsupported, prerelease, malformed, missing, unhealthy, and session-drift
  evidence fails closed

The descriptor and production runtime remain exact `1.14.48`. The candidate
claim is adapter-private until card 122 binds configured instances and
preflight plans to one exact release.

## Validation

- OpenCode adapter: 26 passed; one installed probe remained ignored
- workspace all-target check: passed
- workspace warnings-denied clippy: passed
- formatting: passed

The existing repository `target/debug/deps` directory stalled Rust compiler
metadata scans. Validation was rerun successfully with incremental compilation
disabled and a fresh temporary Cargo target. No cache was deleted or validation
waived.

## Continuation

- card 122 is ready for descriptor publication, exact plan binding, and private
  runtime dispatch
- card 123 remains in bounds for every-span cross-topology conformance and
  roadmap closeout
- no OpenCode range is published yet
