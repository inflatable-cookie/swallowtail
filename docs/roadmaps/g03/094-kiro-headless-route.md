# 094 Kiro ACP Route

Status: completed
Owner: Tom
Created: 2026-08-18
Wave: Secondary
Depends on: g03.093; g03.097 intake card 286; Research 143; Research 153
Research: 143; 153; 156
Route id: `kiro.acp`
Planning state: cards 291-294 completed; first route retargeted from `kiro.headless` by card 286

## Purpose

Add Kiro through official `kiro-cli acp` stdio JSON-RPC if identity proves a
distinct ACP wire. Headless `--no-interactive` stays a later sibling.

This roadmap owns this route only. Related transports, alternate modes, and
other route IDs remain separate roadmaps or explicit negative dispositions.

## Route Boundary

Keep Kiro API-key handling, workspace selection, resource permissions, and
cleanup explicit. Do not import Kiro product policy, IDE semantics, or
`--cloud` sessions. Do not inherit continuation recovery from advertised
`loadSession`.

The adapter must bind host-approved executable or endpoint, environment,
credential reference, model/agent selection where applicable, working resource,
isolation posture, timeout, and cleanup authority. It must not become a generic
provider router or import consumer workflow policy.

## Work Breakdown

- [x] [291](batch-cards/291-kiro-headless-identity-corpus.md)
- [x] [292](batch-cards/292-kiro-headless-driver-core.md)
- [x] [293](batch-cards/293-kiro-headless-prepared-facade.md)
- [x] [294](batch-cards/294-kiro-headless-package-and-route-acceptance.md)

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

Live install, login, and prompt were not justified: this host has no
`kiro-cli`. Deterministic acceptance stands alone.

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

Card 286 retargeted this roadmap from `kiro.headless` to `kiro.acp` because
official `kiro-cli acp` is the machine-facing stdio wire. `kiro.headless`
(`kiro-cli chat --no-interactive`) remains deferred. Batch-card filenames
still say `kiro-headless`; the route id is `kiro.acp`.

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
- Research 156: `docs/research/156-kiro-acp-2-18-1-identity.md`
- [Kiro ACP](https://kiro.dev/docs/cli/acp/)
- [Kiro headless mode](https://kiro.dev/docs/cli/headless/)
