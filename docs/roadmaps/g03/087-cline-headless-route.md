# 087 Cline Headless Route

Status: planned
Owner: Tom
Created: 2026-08-18
Wave: Primary
Depends on: g03.086 Cline ACP Route; g03.097 intake cards 260-261; Research 143
Research: 143
Route id: `cline.headless`
Planning state: cards 304-307 planned

## Purpose

Add Cline through its official bounded headless/JSON CLI surface if it proves materially distinct from Cline ACP.

This roadmap owns this route only. Related transports, alternate modes, and
other route IDs remain separate roadmaps or explicit negative dispositions.

## Route Boundary

The route owns programmatic prompt execution, structured output, process supervision, and bounded terminal truth only where documented. It does not inherit ACP continuation or team/scheduling semantics.

The adapter must bind host-approved executable or endpoint, environment,
credential reference, model/agent selection where applicable, working resource,
isolation posture, timeout, and cleanup authority. It must not become a generic
provider router or import consumer workflow policy.

## Work Breakdown

- [304](batch-cards/304-cline-headless-identity-corpus.md)
- [305](batch-cards/305-cline-headless-driver-core.md)
- [306](batch-cards/306-cline-headless-prepared-facade.md)
- [307](batch-cards/307-cline-headless-package-and-route-acceptance.md)

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

The route must remain separate from g03.086 even if both share an executable or package. Distinct event, authority, and lifecycle evidence is required.

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
