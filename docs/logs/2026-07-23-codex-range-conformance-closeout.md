# 2026-07-23 Codex Range Conformance Closeout

## Changed

- Proved both Codex drivers against every exact release in their frozen
  compatibility corpora.
- Proved discovery and execution under local and remote-authoritative hosts.
- Proved app-server behavior dispatch on both sides of the `0.131.0`
  workspace-root milestone.
- Kept stable sessions free of experimental negotiation and required the gate
  for dynamic tools, provider requests, and bounded workspace roots.
- Rejected missing, below-floor, prerelease, malformed, and unknown-newer
  observations before harness work.
- Left both provider-neutral conformance profiles unchanged.

## Maintained Windows

- exec: `0.122.0..=0.145.0`, behavior `codex.exec.jsonl-v1`
- app-server base: `0.110.0..=0.130.0`, behavior
  `codex.app-server.v2.base`
- app-server workspace roots: `0.131.0..=0.145.0`, behavior
  `codex.app-server.v2.workspace-roots`

These are closed corpus-qualified windows. They do not imply support below the
baseline, for prereleases, or above the latest-qualified point.

## Validation

- focused Codex discovery, range, workspace, and conformance tests pass
- workspace all-target check and warnings-denied clippy pass
- full QA passes with a 549-test inventory: 545 pass and four separately gated
  installed/live probes remain ignored
- doctor remains at the inherited 19 findings: seven errors and twelve warnings
- `git diff --check` passes

macOS stopped one unchanged generated Alibaba test binary in `_dyld_start`.
Temporarily ad-hoc signing that target artifact allowed the authoritative QA run
to proceed. No source, installed executable, provider access, or repository
artifact changed.

## Next

Card 115 evaluates the January-to-April Codex legacy span. It may compile a
separate bounded legacy proof, reject the span at a technical boundary, or
return an unresolved consumer support-floor decision to the operator. It cannot
widen the current claims by inference.
