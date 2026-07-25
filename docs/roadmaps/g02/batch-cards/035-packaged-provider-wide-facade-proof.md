# 035 Packaged Provider-Wide Facade Proof

Status: completed
Owner: Tom
Created: 2026-07-25
Milestone: `../012-provider-wide-acceptance-and-candidate-return.md`

## Objective

Prove every prepared production route through extracted package artifacts
without credentials, installed providers, or live inference.

## Governing Refs

- Contracts 011, 029, 036-037
- card 034
- package-family validation tasks

## Scope

1. Build a transient clean 23-package candidate.
2. Execute deterministic facade construction and one representative bound
   lifecycle per production route.
3. Run Nucleus and Soundcheck packaged Codex runtime proof unchanged.
4. Verify low-level public roles and adapter packages remain usable.
5. Record exact package, API, MSRV, docs, and route evidence.

## Acceptance Criteria

- [x] 22 prepared route proofs pass from package artifacts
- [x] no proof uses sibling-source assumptions
- [x] exact and unverified-newer cases behave as contracted
- [x] cancellation, deadline, drift, redaction, and cleanup evidence passes
- [x] consumer runtime proofs remain green

## Validation

- package prepare and verify selectors
- provider-wide deterministic facade selector
- packaged Nucleus and Soundcheck selectors
- public API, MSRV, docs, content, and repository QA

## Execution Evidence

`effigy package:candidate:facades` builds and extracts one transient
23-package candidate, then runs 20 prepared-facade suites covering all 22
production route identities. The 65 prepared tests pass from package artifacts
with no sibling-source dependencies, live credentials, installed providers, or
provider calls.

The same candidate runs the unchanged consumer proof:

- Nucleus: 14 passed, two live probes ignored
- Soundcheck: four passed, one live probe ignored
- packaged Codex adapter: 89 deterministic tests passed

The package source snapshot is
`6799329ed47090c915dce907effb2dcf53427fa6`. Regeneration from its bundled
source reproduces the candidate package set and checksum manifest. The package
checksum-manifest digest is
`38721af1840f3246e94e783888523775400b8353bad152e6296789854c85ff39`;
provider evidence is
`0e7082187c4a00487f3e331d1e78a41c13dae8d22df8caaaeeccd357bb97a04c`;
consumer evidence is
`6dd36ffa2ad7fb3946f0cb2dd2c4599aba13855cfd67ac4e9dcd100f50b7ff0b`.

Package metadata, public API, docs, MSRV, content, route, repository, and
worktree checks pass. `effigy doctor` retains the known 19 oversized-file
findings: seven errors and 12 warnings.

## Stop Conditions

- a route can pass only with live credentials or provider mutation
- a package omits a required public facade surface
- consumer proof regresses

## Auto-Continuation

No. Record evidence, then make card 036 ready.
