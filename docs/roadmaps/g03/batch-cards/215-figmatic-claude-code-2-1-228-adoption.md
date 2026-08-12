# 215 Figmatic Claude Code 2.1.228 Adoption

Status: ready
Owner: Tom
Created: 2026-08-12
Milestone: `../068-claude-code-response-only-2-1-228-qualification.md`

## Goal

Adopt the exact Swallowtail response-only qualification in Figmatic and resume
the packaged `g04.005` mutation-runway smoke without changing the prepared API
or downstream acceptance authority.

## Source Identity

- Swallowtail commit: `IMPLEMENTATION_COMMIT_PENDING`
- package: `swallowtail-adapter-claude-agent`
- route: `swallowtail.claude-code.response-only`
- executable claim: exact Claude Code `2.1.228`

## Execution

- check out the exact Swallowtail commit above
- in Figmatic, run `effigy deps link cargo /Users/tom/Dev/projects/swallowtail`
- confirm the linked Cargo dependency state, then run the packaged `g04.005`
  mutation-runway smoke
- record isolated preparation, generation, downstream parsing, and cleanup
  evidence in Figmatic's closeout

## Acceptance

- Figmatic reaches generation through the existing prepared facade
- the host preserves approved `HOME`, `USER`, and `LOGNAME` with no
  `ANTHROPIC_API_KEY`
- Figmatic supplies no schema, working resource, attachment, tool, or callback
- events and terminal outcome drain concurrently and the run closes
- returned `OperationContent` remains untrusted ordinary text under Figmatic's
  existing parsing, validation, compilation, gates, and operator acceptance
- no retry, continuation, fallback, alternate route, or provider authority is
  added

## Stop Conditions

- stop on dependency drift, discovery failure, non-empty tools or MCP, visible
  private thought, prepared-API changes, an API-key requirement, or downstream
  acceptance-policy changes
