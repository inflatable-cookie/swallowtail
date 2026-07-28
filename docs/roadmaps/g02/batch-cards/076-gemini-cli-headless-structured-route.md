# 076 Gemini CLI Headless Structured Route

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../023-installed-and-attached-harness-structured-coverage.md`

## Objective

Add a separately registered Gemini CLI headless JSONL structured-run route
without relabeling ACP.

## Scope

1. Freeze exact headless argv, JSONL events, exit codes, model, usage, tool,
   trust, cancellation, deadline, and cleanup evidence across the maintained
   range.
2. Add headless discovery and compatibility binding where it differs from ACP.
3. Implement one prompt with explicit ambient harness configuration and
   isolation.
4. Expose a typed prepared structured operation and exact solution-facade
   selection.
5. Preserve Gemini ACP and hosted Live routes unchanged.

## Acceptance Criteria

- [x] ACP and headless keep separate driver and transport identities
- [x] exact version evidence qualifies the guaranteed range
- [x] JSONL unknown events and exit codes fail safely
- [x] tool and approval behavior is explicit
- [x] later stable versions remain visible unverified-newer where permitted
- [x] no OAuth, account, provider call, or paid inference is required by tests

## Validation

- frozen headless corpus
- one-shot CLI conformance on both host identities
- Gemini ACP and Live regression
- package API, docs, routes, Clippy, and `git diff --check`

## Stop Conditions

- current releases lack stable machine-readable terminal evidence
- trust or approval needs implicit consumer policy
- the route cannot preserve exact model and version truth

## Auto-Continuation

Yes. Continue to card 077.

## Closeout

- Research 045 qualifies headless `stream-json` across exact
  `0.51.0..=0.52.0` source evidence.
- `swallowtail.gemini.headless` is a separate structured driver over
  `gemini-stream-json-stdio`; ACP remains interactive over ACP v1.
- the public Gemini CLI facade requires explicit `Acp` or `Headless`
  selection before discovery
- headless sends the prompt over stdin, pins `plan`, disables extensions and
  MCP, binds exact model and provider-session identity, and forces no sandbox
- durable local transcript retention, cancellation, deadlines, native exits,
  usage, redaction, process wait, and task join remain explicit
- local and remote-authoritative deterministic fixtures pass without provider
  access
- the solution matrix now reports 16 structured `Yes` and five `No`

Validation evidence:

- `cargo test -p swallowtail-adapter-gemini`: 43 passed; one live probe gated
- final Effigy, example, Clippy, docs, CSV, and diff evidence recorded in the
  2026-07-28 closeout log

Continuation remains cards 077-079. Card 077 is ready.
