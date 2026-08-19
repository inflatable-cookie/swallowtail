# 096 Deep Agents ACP Route

Status: completed
Owner: Tom
Created: 2026-08-18
Wave: Secondary
Depends on: g03.094; g03.097 intake card 286; Research 143; Research 153
Research: 143; 153; 157
Route id: `deepagents.acp`
Planning state: cards 299-302 completed

## Purpose

Add Deep Agents through an exact ACP executable/package boundary only if the current implementation can provide deterministic protocol evidence.

This roadmap owns this route only. Related transports, alternate modes, and
other route IDs remain separate roadmaps or explicit negative dispositions.

## Route Boundary

Separate LangChain implementation identity, executable/package identity, ACP wire behavior, activity, cancellation/deadline, and cleanup. Do not treat registry membership as release compatibility.

The adapter must bind host-approved executable or endpoint, environment,
credential reference, model/agent selection where applicable, working resource,
isolation posture, timeout, and cleanup authority. It must not become a generic
provider router or import consumer workflow policy.

## Work Breakdown

- [x] [299](batch-cards/299-deepagents-acp-identity-corpus.md)
- [x] [300](batch-cards/300-deepagents-acp-driver-core.md)
- [x] [301](batch-cards/301-deepagents-acp-prepared-facade.md)
- [x] [302](batch-cards/302-deepagents-acp-package-and-route-acceptance.md)

The first identity card freezes the exact executable/protocol evidence before
driver work. The final card may close the route as accepted, deferred, or
negative evidence; it is not required to create a package if the route does not
survive the evidence gate.

## Required Proof

- [x] exact executable, server, or protocol identity and version axis
- [x] deterministic corpus for success, failure, malformed/unknown input,
      bounds, cancellation/deadline, activity, and joined cleanup
- [x] explicit authentication, working-resource, isolation, and remote/local
      authority posture
- [x] bounded event and terminal-outcome mapping without native-field leakage
- [x] prepared facade with immutable preflight evidence and fail-closed
      selection
- [x] route-specific guide, compiling normal-path example, matrices, package
      index, and README truth if the route is accepted
- [x] separately gated live evidence that cannot silently widen deterministic
      claims

Live install, `npx`, and prompt were not justified: this host has no
`deepagents-acp`. Deterministic acceptance stands alone.

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

The executable and package boundary is the primary admission question; stop if it remains only a library integration.

## Stop Conditions

Stop before implementation if the route is undocumented, prompt-only, UI-only,
requires hidden credential state to establish its wire shape, duplicates an
existing route without information gain, or requires a new public contract that
has not been promoted.

Stop at acceptance if package, guide, example, matrix, authority, or cleanup
truth diverges. Record deferred or negative evidence instead of widening scope.

## Sources

- Research 143: `docs/research/143-new-harness-route-expansion-selection.md`
- Research 153: `docs/research/153-secondary-wave-source-and-disposition.md`
- Research 157: `docs/research/157-deepagents-acp-0-1-25-identity.md`
- [Deep Agents ACP](https://docs.langchain.com/oss/javascript/deepagents/acp)
- [ACP latest registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
