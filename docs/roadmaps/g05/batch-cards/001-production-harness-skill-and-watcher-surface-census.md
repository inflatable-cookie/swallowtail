# 001 Production Harness Skill And Watcher Surface Census

Status: ready
Owner: Tom
Created: 2026-08-28
Milestone: `../001-harness-skill-and-watcher-surface-inventory.md`
Research: `../../../research/255-production-harness-skill-and-watcher-surface-census.md`

## Goal

Build one exact prompt-free evidence matrix for skill discovery and
background-process control across every production harness route.

## Scope

1. Enumerate production harness routes from canonical route and package
   matrices. Exclude direct API, SDK-only, and attached-runtime routes unless a
   harness process owns the questioned surface.
2. For each exact qualified route/version, inspect official docs, frozen source
   or distribution manifests, prompt-free help/list commands, and existing
   protocol fixtures.
3. Record skill listing, provenance, host/project state access, model/session
   visibility, mutation, authentication, and freshness truth.
4. Record native background task identity, start, status, wait, output, stop,
   terminal event, process-tree join, cancellation, deadline, and turn-complete
   truth.
5. Separate provider, harness, Swallowtail, host, and consumer ownership.
6. Promote Research 255 with a closed matrix, evidence gaps, and no design
   recommendation disguised as fact.

## Out Of Scope

- model prompts, credentials, paid work, install/update, login, or account state
- recursive user-home or project scanning
- skill injection, process start/stop, watcher implementation, or public API
- route currentness changes, new routes, consumer edits, or parked Bedrock work

## Acceptance Criteria

- [ ] every production harness route has one exact disposition
- [ ] bundled, host-configured, project-local, plugin, and unknown provenance
      stay separate
- [ ] distribution membership never proves model visibility by implication
- [ ] native background state never proves host-controllable watcher authority
- [ ] Claude/T3 Code remains a research lead, not a portable claim
- [ ] Research 255 names every unavailable or unsafe evidence gate
- [ ] no production or shared-contract change

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

- exact route/version identity or authoritative evidence cannot be bounded
- a useful result requires a provider prompt, credential, mutation, or broad
  ambient filesystem inspection
- the census would expose private skill names, paths, commands, arguments,
  environment, or output
- evidence starts selecting architecture or product policy

## Auto-Continuation

No. Return Research 255 to the orchestrator. Card 002 remains planned until
the evidence is reviewed.
