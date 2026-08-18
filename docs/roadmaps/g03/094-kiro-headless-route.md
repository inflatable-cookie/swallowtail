# 094 Kiro Headless Route

Status: planned
Owner: Tom
Created: 2026-08-18
Wave: Secondary
Depends on: g03.093; g03.097 intake card 286; Research 143
Research: 143
Route id: `kiro.headless`
Planning state: cards 291-294 planned

## Purpose

Add Kiro through its documented headless CLI mode if API-key, workspace, and process authority fit existing contracts.

This roadmap owns this route only. Related transports, alternate modes, and
other route IDs remain separate roadmaps or explicit negative dispositions.

## Route Boundary

Keep Kiro API-key handling, workspace selection, resource permissions, and cleanup explicit. Do not import Kiro product policy or IDE semantics.

The adapter must bind host-approved executable or endpoint, environment,
credential reference, model/agent selection where applicable, working resource,
isolation posture, timeout, and cleanup authority. It must not become a generic
provider router or import consumer workflow policy.

## Work Breakdown

- [291](batch-cards/291-kiro-headless-identity-corpus.md)
- [292](batch-cards/292-kiro-headless-driver-core.md)
- [293](batch-cards/293-kiro-headless-prepared-facade.md)
- [294](batch-cards/294-kiro-headless-package-and-route-acceptance.md)

The first identity card freezes the exact executable/protocol evidence before
driver work. The final card may close the route as accepted, deferred, or
negative evidence; it is not required to create a package if the route does not
survive the evidence gate.

## Required Proof

- [ ] exact executable, server, or protocol identity and version axis
- [ ] deterministic corpus for success, failure, malformed/unknown input,
      bounds, cancellation/deadline, activity, and joined cleanup
- [ ] explicit authentication, working-resource, isolation, and remote/local
      authority posture
- [ ] bounded event and terminal-outcome mapping without native-field leakage
- [ ] prepared facade with immutable preflight evidence and fail-closed
      selection
- [ ] route-specific guide, compiling normal-path example, matrices, package
      index, and README truth if the route is accepted
- [ ] separately gated live evidence that cannot silently widen deterministic
      claims

## Boundaries

- no installation, update, login, credential extraction, or provider mutation
- no unproved session import, archive, restore, delete, resume, retry, or
      continuation semantics
- no broad version range from semver, registry metadata, or release notes alone
- no UI automation, screen scraping, TUI scraping, or foreign SDK bridge
- no flattening onto ACP, OpenCode, Pi, or another route when this route's
      transport or lifecycle is materially distinct
- no release tag, registry publication, or consumer-repository adoption

## Route-Specific Notes

ACP alignment is a separate question and is not implied by this headless roadmap.

## Stop Conditions

Stop before implementation if the route is undocumented, prompt-only, UI-only,
requires hidden credential state to establish its wire shape, duplicates an
existing route without information gain, or requires a new public contract that
has not been promoted.

Stop at acceptance if package, guide, example, matrix, authority, or cleanup
truth diverges. Record deferred or negative evidence instead of widening scope.

## Sources

- Research 143: `docs/research/143-new-harness-route-expansion-selection.md`
- [Kiro headless mode](https://kiro.dev/docs/cli/headless/)
