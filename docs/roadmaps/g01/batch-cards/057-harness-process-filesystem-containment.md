# 057 Harness Process Filesystem Containment

Status: complete
Owner: Tom
Updated: 2026-07-21
Milestone: `../018-kimi-code-acp-portability-proof.md`

## Objective

Qualify the selected macOS execution-host containment mechanism against the
exact Kimi artifact before claiming it as a supported capability.

## Governing References

- `../../../contracts/010-execution-host-services-and-inputs.md`
- `../../../contracts/013-interactive-session-access-policy.md`
- `../../../contracts/017-provider-owned-session-load-replay-and-host-containment.md`
- `../../../research/006-kimi-code-acp-currentness-and-persistent-session-evidence.md`
- `../../../research/007-harness-process-filesystem-containment-evidence.md`
- `../../../research/009-native-macos-kimi-containment-and-successor-delta.md`
- `../../../research/010-kimi-code-successor-and-artifact-currentness-repair.md`
- `../../../research/011-kimi-macos-app-sandbox-runtime-compatibility.md`

## Scope

- prove a deployment-owned macOS App Sandbox launcher/helper as the selected
  first mechanism
- exact read-only and one-root `ReadWrite` containment requirements
- signed helper inheritance and exact security-scoped bookmark handoff
- isolated app-container provider state through `KIMI_CODE_HOME`
- child-process inheritance, links, inherited descriptors, shell/background
  descendants, network independence, cancellation, and cleanup behavior
- provider-neutral preflight and host-service records
- deterministic local and remote-authoritative host fixtures
- one concrete macOS mechanism probe before portable containment records

## Out Of Scope

- Kimi provider mapping
- consumer permission or tool-approval policy
- container, VM, remote-host, Linux, or Windows containment
- consumer app UI, production signing identity, notarization, or distribution
- widening provider network or terminal authority

## Acceptance Criteria

- [x] a working directory or callback capability cannot satisfy containment
- [x] the generic root grant reaches a compatible helper, shell, and every
      tested descendant without broad home-directory access
- [x] exact Kimi artifact runtime compatibility is tested, not inferred
- [x] the incompatible artifact produces no portable containment record
- [x] unsupported `HostEnforced` Kimi execution remains distinguishable from
      ambient execution

## Evidence

- Research 007 compares current Linux, macOS, and Windows authority surfaces
- Linux Landlock is inherited and unprivileged but leaves documented
  filesystem syscall and pre-opened-descriptor gaps
- macOS `sandbox-exec` is deprecated; supported App Sandbox requires signed,
  entitlement-bearing application and helper deployment
- Windows' bounded process-sandbox API is explicitly experimental
- the current macOS local host has no supportable dynamic mechanism
- Contract 017 now rejects partial, best-effort, deprecated, private,
  experimental, and locally fabricated remote containment claims

The first stop condition fired under the original local-only authority. No
containment record, host service, process mapping, provider binary, or
credential behavior was implemented.

The operator subsequently authorized deployment-owned containment and selected
the recommended native macOS App Sandbox helper on 2026-07-20. Research 009
confirms supported helper embedding, Developer ID distribution, inherited
child restrictions, security-scoped project grants, and isolated Kimi state.
Research 010 confirms `0.28.1` is the maintained successor and pins the exact
arm64 archive, upstream executable, signature, entitlements, isolated state,
and upgrade gate. Its provider signature lacks App Sandbox inheritance, so
this card must re-sign the deployment helper, record its new digest, and prove
the Node runtime entitlements remain compatible. The dynamic bookmark handoff
remains a proof requirement, not an assumed capability. Roadmap 019 and card
065 are complete. The later optional-isolation rebaseline is recorded below.

The concrete native probe resolved that uncertainty. A user-selected
security-scoped root propagated through the documented two-entitlement helper
to direct, shell, nested-shell, and background descendants. All twelve inside,
outside, symlink, rename, and network checks passed. The exact Kimi `0.28.1`
artifact did not remain runnable: the documented helper signature and a
diagnostic signature retaining Kimi's four dynamic-code entitlements both
terminated during V8 initialization. A JITless diagnostic then stalled while
loading an extracted ad-hoc native module. Research 011 records the complete
result.

The exact-artifact compatibility stop condition therefore fired. No portable
containment record, host service, Kimi process mapping, or production driver
was added.

Research 012 and the operator's later decision make containment optional. This
card is complete as negative capability evidence: `HostEnforced` Kimi is
unavailable for the pinned artifact. It does not block an explicit
`AmbientHost` route.

## Validation

- deterministic host and preflight tests
- focused clippy and all-target compile
- `git diff --check`

## Validation Result

- the dynamic bookmark and inherited sandbox probe passed all 12 filesystem,
  shell, background-child, link, rename, and network assertions
- both exact Kimi entitlement-signing variants terminated during V8
  initialization with signal 5
- the JITless diagnostic stalled during extracted native-module loading; its
  exact child and parent were terminated and observed
- no Rust containment tests exist because the exact provider artifact did not
  qualify for portable implementation
- full repository QA passes with 259 tests; the two installed probes remain
  separately gated and ignored
- docs QA, Northstar QA, formatting, lint, compile, and diff checks pass
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  7 errors

## Stop Conditions

- a portable claim would depend on an undocumented platform behavior
- the mechanism needs ambient root, container, VM, or deployment authority not
  granted to Swallowtail
- deployment signing cannot preserve the exact provider artifact's runtime
  behavior under the documented inherited-helper model

## Auto-Continuation

No. Card 066 rebaselines the shared policy. Card 058 follows under explicit
ambient execution.
