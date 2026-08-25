# 182 Claude Code Headless Ultracode Binding

Status: blocked; Research 212 empty deliver-now set
Owner: Tom
Created: 2026-08-25
Milestone: [g04.065 Claude Code Headless Ultracode](../065-claude-code-headless-ultracode.md)
Depends on: card 181; non-empty Research 212 deliver-now table

## Goal

Bind only Research 212's exact Ultracode rows through a typed adapter-local
prepared input, immutable evidence, selected-model agreement, and exact child
arguments without changing portable reasoning vocabulary.

## Work

1. Add the smallest opaque adapter-local Ultracode opt-in admitted by Research
   212. Do not add a raw string, bool alias, settings object, environment map,
   generic generation map, or portable `ReasoningMode` value.
2. Admit it only for exact Research 212 version/model/profile rows and carry it
   through prepared input, immutable evidence, bound driver, and launch
   validation before child creation.
3. Reject composition with any separately selected portable reasoning value
   before effects. Preserve the existing reasoning path when Ultracode is
   omitted.
4. Emit only the exact qualified child arguments. Do not mutate user settings,
   pass raw `--settings`, or create a synthetic configuration root.
5. Keep Plan mode, fixed tools, empty MCP, no-session persistence, selected
   model, access, working resource, retention, activity, usage, cancellation,
   deadline, terminal, and cleanup exact.
6. Reject unsupported model, stale version, entitlement/config conflict,
   alias, fallback, or unbounded workflow topology at the strongest qualified
   boundary without another route or mode.
7. Keep hidden reasoning, internal workflow text, and teammate/subagent detail
   out of public output and activity unless already qualified independently.
8. Add focused fixtures, tests, example/API baseline, and guide changes only
   as required by the delivered surface.

## Acceptance Criteria

- [ ] only Research 212 exact rows prepare and dispatch
- [ ] Ultracode is adapter-local and never enters portable reasoning vocabulary
- [ ] separate reasoning selection conflicts before process creation
- [ ] request, evidence, selected model, driver, and child argv agree
- [ ] omission preserves existing command and behavior
- [ ] no raw settings, user config, generic argv/environment, or hidden
      authority surface is exposed
- [ ] dynamic workflow topology stays inside Research 212's exact bounds
- [ ] lifecycle, access, retention, activity, usage, and cleanup remain exact
- [ ] no shared contract/runtime or breaking public API change
- [ ] `cargo fmt -p swallowtail-adapter-claude-agent` passes
- [ ] `effigy validate:focused swallowtail-adapter-claude-agent` passes
- [ ] `effigy package:verify-affected swallowtail-adapter-claude-agent` passes
- [ ] `git diff --check` passes

## Stop Conditions

- Research 212 is empty or contradicts the planned mapping
- safe dispatch requires live confirmation, raw settings/config mutation,
  unbounded process topology, shared authority, or a breaking API

## Out Of Scope

- Fast mode, teams, response-only/ACP, unrelated Claude controls, live provider
  work, currentness, release, merge, generation rollover, or g04 closure
