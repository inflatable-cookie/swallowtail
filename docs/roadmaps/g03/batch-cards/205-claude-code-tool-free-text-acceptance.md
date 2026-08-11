# 205 Claude Code Tool-Free Text Acceptance

Status: completed
Owner: Tom
Updated: 2026-08-11

## Goal

Close g03.065 with mutually honest code, docs, route inventory, validation, and
consumer adoption evidence.

## Scope

- guide, provider route matrix, feature matrix, guide map, and architecture
- public API and minimal consumer example
- focused, affected-package, docs, guide, route, and live validation
- closeout log, roadmap state, and exact adoption commit

## Out Of Scope

- Figmatic edits, version bump, tag, GitHub Release, registry publication

## Acceptance Criteria

- [x] route claims text only and no `StructuredOutput`
- [x] deterministic, package, guide, route, example, and live gates pass;
      broad docs retains the recorded Effigy roadmap-index failure
- [x] existing Claude routes retain behavior and compatibility claims
- [x] Figmatic example compiles against the exact API
- [x] release remains operator-gated

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy qa:docs`
- `effigy qa:guides`
- `effigy qa:routes`
- separately gated authenticated Claude Code response-only probe
