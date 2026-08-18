# 092 Pi ACP Route

Status: planned
Owner: Tom
Created: 2026-08-18
Wave: Primary
Depends on: g03.091; g03.097 intake cards 260-261; Research 143
Research: 143
Route id: `pi.acp`
Planning state: cards 282-285 planned

## Purpose

Extend the existing Pi adapter with the distinct ACP route only if its wire and lifecycle behavior are materially different from `pi.rpc`.

This roadmap owns this route only. Related transports, alternate modes, and
other route IDs remain separate roadmaps or explicit negative dispositions.

## Route Boundary

Reuse the existing Pi package where appropriate, but keep `pi.acp` and `pi.rpc` route identity, version axes, lifecycle, and cleanup truth distinct.

The adapter must bind host-approved executable or endpoint, environment,
credential reference, model/agent selection where applicable, working resource,
isolation posture, timeout, and cleanup authority. It must not become a generic
provider router or import consumer workflow policy.

## Work Breakdown

- [282](batch-cards/282-pi-acp-identity-corpus.md)
- [283](batch-cards/283-pi-acp-driver-core.md)
- [284](batch-cards/284-pi-acp-prepared-facade.md)
- [285](batch-cards/285-pi-acp-package-and-route-acceptance.md)

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

Do not create a duplicate package merely to represent a second transport. The acceptance card must justify the existing-adapter extension.

## Stop Conditions

Stop before implementation if the route is undocumented, prompt-only, UI-only,
requires hidden credential state to establish its wire shape, duplicates an
existing route without information gain, or requires a new public contract that
has not been promoted.

Stop at acceptance if package, guide, example, matrix, authority, or cleanup
truth diverges. Record deferred or negative evidence instead of widening scope.

## Sources

- Research 143: `docs/research/143-new-harness-route-expansion-selection.md`
- [Pi repository](https://github.com/badlogic/pi-mono)
- [ACP latest registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
