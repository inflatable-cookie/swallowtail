# 086 Cline ACP Route

Status: completed
Owner: Tom
Created: 2026-08-18
Wave: Primary
Depends on: g03.097 intake cards 260-261; Research 143
Research: 143
Route id: `cline.acp`
Planning state: cards 262-265 completed

## Purpose

Add Cline through its exact Agent Client Protocol surface if the current official client/server boundary and lifecycle evidence support a distinct Swallowtail route.

This roadmap owns this route only. Related transports, alternate modes, and
other route IDs remain separate roadmaps or explicit negative dispositions.

## Route Boundary

ACP wire compatibility is separate from Cline release compatibility. Prove initialize, capabilities, bounded activities/events, terminal outcomes, cancellation/deadline, and cleanup without assuming every ACP registry feature.

The adapter must bind host-approved executable or endpoint, environment,
credential reference, model/agent selection where applicable, working resource,
isolation posture, timeout, and cleanup authority. It must not become a generic
provider router or import consumer workflow policy.

## Work Breakdown

- [262](batch-cards/262-cline-acp-identity-corpus.md)
- [263](batch-cards/263-cline-acp-driver-core.md)
- [264](batch-cards/264-cline-acp-prepared-facade.md)
- [265](batch-cards/265-cline-acp-package-and-route-acceptance.md)

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

Cline headless JSON execution is a separate route and is owned by g03.087; do not silently combine it here.

## Stop Conditions

Stop before implementation if the route is undocumented, prompt-only, UI-only,
requires hidden credential state to establish its wire shape, duplicates an
existing route without information gain, or requires a new public contract that
has not been promoted.

Stop at acceptance if package, guide, example, matrix, authority, or cleanup
truth diverges. Record deferred or negative evidence instead of widening scope.

## Sources

- Research 143: `docs/research/143-new-harness-route-expansion-selection.md`
- [Cline repository](https://github.com/cline/cline)
- [Cline CLI reference](https://docs.cline.bot/cli/cli-reference)
- [ACP latest registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
