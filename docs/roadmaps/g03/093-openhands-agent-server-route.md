# 093 OpenHands Agent Server Route

Status: planned
Owner: Tom
Created: 2026-08-18
Wave: Secondary
Depends on: g03.097 intake card 286; Research 143; primary route closeout
Research: 143
Route id: `openhands.agent-server`
Planning state: cards 287-290 planned

## Purpose

Add OpenHands through its explicit HTTP/WebSocket Agent Server boundary if remote ownership and cleanup can be represented honestly.

This roadmap owns this route only. Related transports, alternate modes, and
other route IDs remain separate roadmaps or explicit negative dispositions.

## Route Boundary

Remote host, workspace, persistence, agent execution, attachment, cancellation, and cleanup are first-class authority boundaries. Do not infer durable server control from an HTTP/WebSocket handshake.

The adapter must bind host-approved executable or endpoint, environment,
credential reference, model/agent selection where applicable, working resource,
isolation posture, timeout, and cleanup authority. It must not become a generic
provider router or import consumer workflow policy.

## Work Breakdown

- [287](batch-cards/287-openhands-agent-server-identity-corpus.md)
- [288](batch-cards/288-openhands-agent-server-driver-core.md)
- [289](batch-cards/289-openhands-agent-server-prepared-facade.md)
- [290](batch-cards/290-openhands-agent-server-package-and-route-acceptance.md)

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

This is not an ACP wrapper roadmap. Its information gain is the remote agent-server topology.

## Stop Conditions

Stop before implementation if the route is undocumented, prompt-only, UI-only,
requires hidden credential state to establish its wire shape, duplicates an
existing route without information gain, or requires a new public contract that
has not been promoted.

Stop at acceptance if package, guide, example, matrix, authority, or cleanup
truth diverges. Record deferred or negative evidence instead of widening scope.

## Sources

- Research 143: `docs/research/143-new-harness-route-expansion-selection.md`
- [OpenHands Agent Server](https://docs.openhands.dev/sdk/arch/agent-server)
- [OpenHands repository](https://github.com/All-Hands-AI/OpenHands)
- [OpenHands ACP agents](https://docs.openhands.dev/openhands/usage/agent-canvas/acp-agents)
