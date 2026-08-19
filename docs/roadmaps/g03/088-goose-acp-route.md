# 088 Goose ACP Route

Status: completed
Owner: Tom
Created: 2026-08-18
Wave: Primary
Depends on: g03.087; g03.097 intake cards 260-261; Research 143
Research: 143
Route id: `goose.acp`
Planning state: cards 266-269 completed

## Purpose

Add Goose through its maintained ACP client/provider boundary without flattening Goose extensions or local authority into generic ACP claims.

This roadmap owns this route only. Related transports, alternate modes, and
other route IDs remain separate roadmaps or explicit negative dispositions.

## Route Boundary

Prove the exact ACP role, executable boundary, extension/capability behavior, activity projection, and cleanup posture. Keep Goose provider configuration and extension policy outside the stable adapter contract.

The adapter must bind host-approved executable or endpoint, environment,
credential reference, model/agent selection where applicable, working resource,
isolation posture, timeout, and cleanup authority. It must not become a generic
provider router or import consumer workflow policy.

## Work Breakdown

- [266](batch-cards/266-goose-acp-identity-corpus.md)
- [267](batch-cards/267-goose-acp-driver-core.md)
- [268](batch-cards/268-goose-acp-prepared-facade.md)
- [269](batch-cards/269-goose-acp-package-and-route-acceptance.md)

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

A Goose route is not evidence for every Goose provider, extension, or remote session feature.

## Stop Conditions

Stop before implementation if the route is undocumented, prompt-only, UI-only,
requires hidden credential state to establish its wire shape, duplicates an
existing route without information gain, or requires a new public contract that
has not been promoted.

Stop at acceptance if package, guide, example, matrix, authority, or cleanup
truth diverges. Record deferred or negative evidence instead of widening scope.

## Sources

- Research 143: `docs/research/143-new-harness-route-expansion-selection.md`
- [Goose repository](https://github.com/block/goose)
- [Goose ACP clients](https://goose-docs.ai/docs/guides/acp-clients/)
- [Goose ACP providers](https://goose-docs.ai/docs/guides/acp-providers/)
- [ACP latest registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
