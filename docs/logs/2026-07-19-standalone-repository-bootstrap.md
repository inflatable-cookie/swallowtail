# Standalone Repository Bootstrap

Date: 2026-07-19
Status: recorded

## Decision

Swallowtail is a standalone strict-Northstar project. Its repository owns the
shared runtime vision, contracts, architecture, and delivery roadmap.

## Context

Nucleus contains the broadest current agent-harness implementation. Soundcheck
has already duplicated a narrower Codex CLI connector. Reuse is valuable, but
placing authority inside either consumer would couple the other application to
foreign product policy.

## Result

- Swallowtail owns portable mechanisms.
- consumers retain prompts, tools, workflows, authority, and state.
- the first code boundary is pure provider-neutral records plus fixtures.
- process, transport, async, and provider extraction wait for later contracts.

## Validation

- `effigy tasks`
- `effigy doctor` — no findings
- `effigy test --plan`
- `effigy qa`
- all Markdown link checks
- `git diff --check`
