# 089 Pi SDK Sidecar Protocol And Corpus

Status: ready
Owner: Tom
Created: 2026-08-21
Milestone: `../033-pi-sdk-sidecar-route.md`
Depends on: Research 181; amended Contracts 019 and 029

## Goal

Create the smallest source-tagged Node sidecar over Pi's public SDK and freeze
its private protocol before adding a production Rust driver.

## Scope

1. Add an application-local sidecar entry point owned by the Swallowtail
   source tag; do not create or publish a new npm package.
2. Use only public `@earendil-works/pi-coding-agent@0.84.2` exports.
3. Freeze strict correlated LF-JSON `swallowtail-pi-sdk-jsonl-v1` command,
   response, event, terminal, diagnostic, and bounds fixtures.
4. Bind one exact Node runtime satisfying the upstream `>=22.19.0`
   requirement and record it separately from the SDK package.
5. Construct in-memory settings, explicit resource loading, explicit provider
   and model, and exactly `read`, `grep`, `find`, and `ls` tools.
6. Disable ambient extensions, skills, prompts, context files, themes,
   settings, aliases, catalogue refresh, update checks, retry, and fallback.
7. Define commands for bootstrap, fresh session, switch with expected cwd,
   typed replay, prompt, steer, follow-up, abort, state, and close. Later cards
   may expose only the subset they prove.
8. Add deterministic sidecar and protocol fixtures that need no install,
   account, credential, provider, or network access.

## Out Of Scope

- production Rust driver behavior (card 090)
- portable session load/resume (card 091)
- prepared or addable connection surfaces (card 092)
- Pi deep imports, session-file parsing, shell/write tools, or containment
- modifying the existing `pi.rpc` route

## Acceptance Criteria

- sidecar, wire, Node, and SDK identities remain separate
- every frame is bounded, correlated, redacted, and fail-closed on unknown
  semantics
- bootstrap exposes the effective cwd before readiness
- no ambient loader or automatic network work is enabled
- fixtures execute deterministically without resolving a mutable package

## Validation

- `effigy validate:focused swallowtail-adapter-pi swallowtail-testkit`
- `effigy qa:northstar`
- `git diff --check`
- `effigy package:api` if public API changes

## Auto-Continuation

Yes, into card 090.

## Stop Conditions

- Stop if implementation requires non-public Pi imports.
- Stop if default SDK construction remains ambient or performs network work.
- Stop if the protocol cannot bound and correlate every frame.
- Stop if validation requires `npm install` or provider access.
