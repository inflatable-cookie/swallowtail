# 090 Mistral Vibe Headless Route

Status: planned
Owner: Tom
Created: 2026-08-18
Wave: Primary
Depends on: g03.089; g03.097 intake cards 260-261; Research 143
Research: 143
Route id: `mistral-vibe.headless`
Planning state: cards 274-277 planned

## Purpose

Add Mistral Vibe through its documented programmatic prompt, JSON, and streaming CLI surface.

This roadmap owns this route only. Related transports, alternate modes, and
other route IDs remain separate roadmaps or explicit negative dispositions.

## Route Boundary

The route is a structured headless process boundary. Prove stream framing, terminal outcome, error behavior, cancellation/deadline, and working-resource authority without importing Mistral API or ACP claims.

The adapter must bind host-approved executable or endpoint, environment,
credential reference, model/agent selection where applicable, working resource,
isolation posture, timeout, and cleanup authority. It must not become a generic
provider router or import consumer workflow policy.

## Work Breakdown

- [274](batch-cards/274-mistral-vibe-headless-identity-corpus.md)
- [275](batch-cards/275-mistral-vibe-headless-driver-core.md)
- [276](batch-cards/276-mistral-vibe-headless-prepared-facade.md)
- [277](batch-cards/277-mistral-vibe-headless-package-and-route-acceptance.md)

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

Any Mistral Vibe ACP surface is a separate disposition and must not be folded into this headless roadmap.

## Stop Conditions

Stop before implementation if the route is undocumented, prompt-only, UI-only,
requires hidden credential state to establish its wire shape, duplicates an
existing route without information gain, or requires a new public contract that
has not been promoted.

Stop at acceptance if package, guide, example, matrix, authority, or cleanup
truth diverges. Record deferred or negative evidence instead of widening scope.

## Sources

- Research 143: `docs/research/143-new-harness-route-expansion-selection.md`
- [Mistral Vibe repository](https://github.com/mistralai/mistral-vibe)
- [Mistral Vibe CLI](https://docs.mistral.ai/vibe/code/cli/work-with-cli)
- [ACP latest registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
