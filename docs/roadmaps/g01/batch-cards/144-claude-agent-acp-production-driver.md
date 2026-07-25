# 144 Claude Agent ACP Production Driver

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../048-post-grok-hold-provider-coverage-continuation.md`

## Objective

Implement the exact Claude Agent ACP range and capability subset frozen by card
143 as a separately registered production driver.

## Governing Refs

- Research 032
- card 143 qualified range, contracts, records, and corpus
- Contracts 005-012, 014, 015, 017, 023, 029, 032, and 033
- roadmap g01.048

## Scope

1. Register family `claude-agent` separately from Anthropic direct inference
   and Managed Agents.
2. Observe and assess the installed `claude-agent-acp` artifact before
   configuration or preflight.
3. Bind exact adapter range segment, ACP wire, public Anthropic endpoint,
   API-key lease, model route, resource, access, configuration, isolation, and
   topology before process start.
4. Launch only the frozen ACP v1 stdio invocation with no package-manager,
   installer, updater, terminal-auth, or subscription behavior.
5. Apply the frozen provider-native read-tool subset without a containment
   claim.
6. Implement only the card-143 session, event, permission, cancellation,
   deadline, failure, and close behavior.
7. Preserve redaction and joined process, callback, resource, and credential
   cleanup.
8. Add no implicit provider, model, endpoint, credential, configuration,
   version, or topology fallback.

## Frozen Range

- axis: `claude-agent.acp-adapter`
- baseline: `0.53.0`
- latest qualified: `0.61.0`
- exclusion: unpublished `0.58.0`
- unverified newer: allowed and visible
- revisions:
  - `claude-agent.acp.baseline-v1`
  - `claude-agent.acp.session-config-v2`
  - `claude-agent.acp.provider-capability-v3`
  - `claude-agent.acp.steering-metadata-v4`
- nested native baseline/latest: Claude Code `2.1.195` / `2.1.217`
- wire: ACP v1
- access: Anthropic public API key only
- configuration/isolation: `Ambient` / `AmbientHost`

## Acceptance Criteria

- [x] family, driver, transport, adapter, SDK, wire, access, and route
      identities match the frozen route
- [x] preflight and runtime behavior agree exactly
- [x] qualified and unverified-newer versions dispatch privately and visibly
- [x] provider-native read restrictions do not claim sandboxing
- [x] terminal auth and Claude subscription access remain unavailable
- [x] default tests use deterministic fixtures only
- [x] all owned work joins and all leases release

## Validation

- focused discovery, preflight, production-driver, and fixture tests
- warnings-denied focused clippy
- affected shared tests
- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- card 143 is incomplete
- production behavior diverges from the frozen corpus
- installed observation cannot bind the exact qualified interface
- runtime needs terminal auth, ambient secret exposure, or an uncontracted
  nested-binary override
- implementation needs any other uncontracted authority

## Auto-Continuation

No. Card 145 owns unchanged-profile conformance and full closeout.

## Outcome

Added `swallowtail-adapter-claude-agent` as a separate family and production
driver. Its descriptor exposes only installed discovery and interactive
harness sessions over `acp-v1-stdio`.

Discovery runs the exact host-approved wrapper with only `--version`.
Preflight binds the observed exact version, public Anthropic API-key access,
one exact model and working resource, `Ambient` configuration, and
`AmbientHost` isolation before process start. Runtime launches the wrapper
with no arguments and only the approved environment reference.

Four private revisions cover qualified milestones `0.53.0`, `0.54.0`,
`0.60.0`, and `0.61.0`. Excluded `0.58.0` and incompatible `0.52.0` remain
unavailable. Stable newer points use latest-qualified behavior only as visible
unverified executions.

The frozen session creates one exact-model read-only session with
`Read`/`Glob`/`Grep`, maps bounded reasoning, tool, usage, and text events,
services bounded reads, rejects provider permission requests before cancelling,
supports active cancellation and deadlines, and joins all turn, protocol, and
process work before releasing resource and credential leases. It advertises
no containment, terminal-auth, subscription, persistent-session, write,
shell, provider-switching, or installation authority.

## Evidence

- 9 deterministic adapter tests pass: three unit, four production-driver, and
  two installed-discovery tests
- all 52 shared ACP protocol tests pass, including the eight frozen Claude
  Agent corpus tests
- focused warnings-denied Clippy passes
- workspace all-target checking, docs QA, Northstar QA, formatting, and diff
  checks pass
- the deadline fixture proves timeout, ACP cancellation, task join, and
  credential/resource release
- Doctor remains at the inherited 19 findings: 12 warnings and 7 errors
- no live account, provider request, package installation, or container is used
- card 145 is ready for unchanged-profile topology conformance and full QA
