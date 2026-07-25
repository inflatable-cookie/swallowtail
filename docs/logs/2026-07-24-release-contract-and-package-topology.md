# Release Contract And Package Topology

Date: 2026-07-24

## Decision

The operator approved provisional Spec 004.

Swallowtail now has one durable release boundary:

- all 23 current crates are public and separately consumable
- crates.io is the initial registry
- packages share a coordinated pre-1.0 version beginning at `0.1.0`
- internal normal and build dependencies use local paths plus ordinary
  compatible registry requirements
- publication follows core and protocols, runtime, then support, transport, and
  adapters
- the initial MSRV is `1.93`, except Bedrock at `1.94.1`
- Apple Silicon macOS is verified; other targets remain unverified, not
  prohibited
- package preparation is deterministic and credential-free
- every upload, owner change, tag, push, GitHub release, workflow edit, and
  consumer change remains separately human-approved

## Promotion

- Added accepted release and package topology architecture.
- Added active Contract 036.
- Promoted Research 033.
- Archived Spec 004.
- Completed g02 card 002.
- Rebaselined card 003 to ready and card 004 to the promoted contract.

No Cargo manifest, package archive, registry, credential, tag, workflow, or
consumer changed in this promotion batch.

## Validation

- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed
- `effigy doctor` — unchanged inherited 19 oversized-file findings: 12
  warnings and seven errors

## Lane State

Card 003 subsequently realized the package graph and deterministic local gates.
Card 004 is now ready. Actual release mutation remains unauthorized.
