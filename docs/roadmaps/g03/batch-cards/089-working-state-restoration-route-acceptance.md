# 089 Working-State Restoration Route Acceptance

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../034-working-state-restoration-facade.md`
Depends on: card 088

## Goal

Expose the common facade across all qualified reconciliation routes and close
consumer-facing route truth.

## Scope

1. Wrap Codex, OpenCode, Kimi local-server, OpenAI background, and Anthropic
   Managed Agents reconciliation preparation.
2. Preserve each existing exact input and outcome.
3. Add compile-tested public integration guidance.
4. Run focused and affected-package verification across the seven mappings.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-codex swallowtail-adapter-opencode swallowtail-adapter-kimi`
- `effigy validate:focused swallowtail-adapter-openai swallowtail-adapter-anthropic swallowtail-adapter-claude-agent`
- affected-package verification for the same exact packages

## Stop Conditions

- stop if a wrapper changes provider dispatch or failure behavior
- stop if route-specific preparation is replaced by generic routing policy

## Closeout

- wrapped all five qualified reconciliation routes without changing dispatch
- added public guidance and a compile-checked integration example
- focused validation and affected-package proof passed across all seven
  runtime/adapter packages
- g03 returned to its evidence gate
