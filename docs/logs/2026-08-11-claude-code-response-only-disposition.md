# Claude Code Response-Only Structured Route Disposition

Date: 2026-08-11
Roadmap: g03.064
Cards: 200-201

## Outcome

Claude Code `2.1.227` does not qualify for the selected response-only
structured boundary. `--tools ""` produces a true tool-free text run, but
adding `--json-schema` injects the model-visible `StructuredOutput` tool.

An unsatisfiable schema caused four tool attempts across six turns, then the
CLI exited `0` with subtype `success` and `structured_output:null`. Claude Code
exposes no zero-retry binding for this path. Contracts 039-040 therefore block
the capability without an adapter implementation.

## Access And Lifecycle Evidence

- exact installed payload `2.1.227`
- local `claude.ai` Max subscription authenticated
- `ANTHROPIC_API_KEY` absent
- strict empty MCP set and no launch-directory artifacts
- malformed schema exited `1`
- external termination exited `143` with no child or artifact remaining

## Current State

Existing `claude-code.headless` stays unchanged: exact qualified `2.1.220`,
read-only Plan mode, and required filesystem resource. `2.1.227` remains
unverified-newer for that route. No response-only identity, capability,
enforcement claim, fixture, prepared API, consumer example, version bump, or
release was added.

Research, log, g03, and batch-card index gates pass. Broad `effigy qa:docs`
remains blocked by existing Effigy roadmap child-link resolution and
next-action policy defects; the repository requires one front-door Next Task
pointer while that gate incorrectly requires one in every generation and
backlog index. The papercut is recorded separately.

## Next

Figmatic must not adopt Claude Code for this boundary. Reopen only on exact
upstream proof of schema enforcement without a model-visible tool, retry,
filesystem authority, session retention, or null-success terminal behavior.
Otherwise select another provider-specific route or revise the consumer
boundary explicitly. Figmatic may retain approved release `v0.3.1` for
existing integrations; no new Swallowtail commit or release carries this
capability.
