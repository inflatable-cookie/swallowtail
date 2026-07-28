# Pi RPC And OpenCode Structured Runs

Date: 2026-07-28
Card: `../roadmaps/g02/batch-cards/075-pi-rpc-and-opencode-structured-runs.md`

## Changed

- Added independent `StructuredRun` roles, requirements, validation, runtime
  handles, and typed prepared operations to Pi RPC and OpenCode HTTP.
- Pi now projects one exact `--no-session` RPC process into one bounded run,
  including qualified UI callbacks, native abort, deadline, terminal, and
  joined process/resource/credential cleanup.
- Pi requires prohibited provider retention and exposes no provider run,
  reusable session, resume, or management authority.
- OpenCode now creates one operation-private session, prompts once over the
  attached HTTP/SSE route, awaits terminal evidence, closes, deletes only that
  session, and releases access last.
- OpenCode reports temporary retention and confirmed or unconfirmed
  operation-owned session deletion without claiming attached-server lifecycle
  authority.
- Expanded the shared structured-harness boundary pack across prohibited,
  temporary-with-deletion, and durable-without-deletion retention cases.
- Changed the Pi RPC and OpenCode HTTP solution-matrix structured cells from
  `No` to `Yes`.

## Current State

Roadmap g02.023 has completed its ACP, RPC, and attached-HTTP work. Its final
batch is the separately qualified Gemini CLI headless JSONL route in card 076.

The 21-row solution matrix now reports 15 structured `Yes` and six `No`.
Kimi deletion remains unsupported by operator decision. Gemini headless, Kimi
headless, and Kimi local-server work plus the llama.cpp owned-serving
reclassification remain in cards 076-079.

## Validation

- Pi, OpenCode, and testkit focused suites pass
- strict all-target Clippy passes for the three affected crates
- docs and provider-route checks pass
- the provider-sorted 21-row CSV parses with 45 columns
- no live account, credential, provider request, consumer edit, package
  publication, or attached-server lifecycle mutation occurred
