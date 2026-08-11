# 200 Claude Code Response-Only Live Qualification

Status: completed
Owner: Tom
Updated: 2026-08-11

## Goal

Qualify installed Claude Code `2.1.227` against the response-only structured
boundary before any contract or runtime implementation.

## Scope

- exact executable, version, help, local subscription access, and API-key
  absence
- fresh launch directory with safe mode, empty tools, strict empty MCP,
  no session persistence, explicit model and effort, and no fallback
- text-only and schema-enabled stream envelopes
- valid, unsatisfiable, and malformed schema behavior
- external termination, child-process, and launch-artifact observations

## Out Of Scope

- adapter implementation or version-range extension
- Figmatic edits
- provider billing, release, or tag work

## Acceptance Criteria

- [x] ordinary tool-free invocation reports no tools or MCP servers
- [x] schema invocation exposes its exact tool and terminal envelope
- [x] retries and terminal null behavior are counted
- [x] OAuth subscription works with `ANTHROPIC_API_KEY` absent
- [x] malformed schema and termination evidence remain redacted

## Validation

- [x] valid schema: `StructuredOutput` visible; typed terminal object; exit `0`
- [x] unsatisfiable schema: four tool attempts; six turns; success with null
- [x] malformed schema: exit `1`; no artifact
- [x] termination: exit `143`; no remaining child; no artifact
