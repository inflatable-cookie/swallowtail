# 142 Post-Grok-Hold Provider And Transport Evidence

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../048-post-grok-hold-provider-coverage-continuation.md`

## Objective

Select the next high-information provider or transport proof from current
authoritative evidence without requiring a live developer account or heavy
container for deterministic development.

## Governing Refs

- Vision 001
- system architecture and repository authority map
- Research 029-031
- Contracts 005-007, 011, 014, 015, 019, 020, 023, 029, 032, and 035
- roadmaps g01.047-048

## Scope

1. Inventory realized integration families, drivers, transports, execution
   layers, operation shapes, conformance profiles, and maintained ranges.
2. Revalidate current authoritative evidence for:
   - materially different first-party harness routes
   - direct hosted providers not yet represented
   - ACP registry, lifecycle, authentication, and remote composition
   - installed-range depth for narrow existing harness claims
   - attached self-hosted runtimes that do not require Swallowtail to own
     persistent serving
3. Separate user-time access requirements from development-time live-test
   requirements.
4. Rank candidates by architectural information, maintained authority,
   fixture quality, implementation weight, compatibility pressure, account
   dependency, and container dependency.
5. Recommend one exact route and identify every missing shared contract.
6. Rebaseline cards 143-145 to that route only when the decision is
   contractually ready and does not establish unsettled product policy.

## Boundaries

- no provider, transport, protocol, or runtime implementation
- no live account, login, credential, paid request, or provider mutation
- no package installation or executable launch
- no Grok lane work
- no heavy container or persistent model-serving commitment
- no Nucleus, Soundcheck, or Monkey edit
- no inferred compatibility range or implicit fallback

## Acceptance Criteria

- [x] current realized coverage and remaining gaps are explicit
- [x] leading evidence is current and authoritative
- [x] account-at-use and account-required-for-development remain separate
- [x] container weight and model-serving ownership remain explicit
- [x] the recommendation maximizes information rather than provider count
- [x] missing contracts and product-policy decisions are visible
- [x] one exact continuation card is ready or operator input is requested

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` delta review
- `git diff --check`

## Evidence Required

- source URLs and access dates
- provider/driver/transport/profile/range inventory
- ranked candidate matrix
- recommended route, exclusions, contract delta, and fixture plan
- explicit account, authority, topology, protocol, and support risks

## Stop Conditions

- current evidence cannot distinguish the leading candidates
- provider or credential choice would establish unsettled product policy
- the leading route requires a live development account
- the leading route implicitly requires heavy containers or model ownership
- a missing contract prevents a ready corpus card

## Auto-Continuation

Yes, only when card 142 resolves the route without product-policy ambiguity,
promotes every required contract, and rebaselines card 143 to ready.

## Outcome

Research 032 selects Claude Agent ACP as the next proof:

- family `claude-agent`
- ACP v1 over bounded NDJSON stdio
- candidate adapter range `0.52.0..=0.61.0`
- host-approved Anthropic public-API key access
- `Ambient` configuration
- `AmbientHost` isolation
- provider-native read-tool restrictions without a sandbox claim

The maintained adapter supplies transparent tagged source and mock-backed tests
for deterministic development. User-time use needs billable Anthropic access;
default development does not.

Claude subscription login is excluded. Anthropic's Agent SDK documentation
requires prior approval before a third-party product offers claude.ai login or
subscription rate limits. The ACP registry's authorship record does not grant
that approval to Swallowtail.

No new shared contract is required before corpus work. Contracts 015, 017, 023,
029, 032, and 033 govern the selected subset. Card 143 must stop and promote a
narrow contract if exact artifacts expose a composite-version, access, or
lifecycle gap.

Cards 143-145 are rebaselined to the selected route. Card 143 is ready.

## Validation Evidence

- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed
- `effigy doctor` — unchanged inherited 19 oversized-file findings:
  12 warnings and 7 errors
