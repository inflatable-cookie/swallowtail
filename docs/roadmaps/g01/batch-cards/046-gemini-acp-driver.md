# 046 Gemini ACP Driver

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../014-gemini-acp-proof.md`

## Objective

Implement the first ACP transport and Gemini CLI mapping.

## Scope

- bounded ACP v1 NDJSON framing and correlation in `swallowtail-protocol-acp`
- host-approved `WorkingResourceIo` read callback service
- Gemini CLI `0.51.0` process mapping with isolated provider state,
  `--acp --approval-mode plan`, new sessions, text prompts, updates,
  permission cancellation, active-turn cancellation, and joined process close

## Out Of Scope

- undocumented Gemini extensions
- weakening existing callback/access contracts
- authentication mutation, load/resume, mode/model mutation, MCP injection,
  filesystem writes, terminal callbacks, and native session close

## Acceptance Criteria

- [x] common ACP code contains no Gemini identity branch
- [x] Gemini mapping claims only advertised capabilities
- [x] unknown extensions preserve or reject explicitly
- [x] missing local binary does not affect fixture tests
- [x] no callback or ambient Gemini policy widens the read-only resource

## Validation

- focused driver tests
- dependency scan
- `git diff --check`

## Stop Conditions

- reusable ACP code requires flattening agent differences
- isolated provider state cannot prevent ambient policy or credential fallback

## Auto-Continuation

Yes, after card 047 is ready and focused validation passes.

## Evidence

- `swallowtail-protocol-acp` owns bounded ACP v1 NDJSON framing, message
  classification, correlation-safe request encoding, and explicit error
  responses without Gemini identity.
- `WorkingResourceIo` is a distinct runtime host capability. The local host
  resolves canonical paths under the leased root, bounds line and byte reads,
  and rejects traversal and symlink escape.
- `swallowtail-adapter-gemini` pins Gemini CLI `0.51.0`, one isolated process
  environment, `--acp --approval-mode plan`, API-key access identity, new
  sessions, text prompts, updates, filesystem reads, permission cancellation,
  native turn cancellation, and joined process close.
- Authentication mutation, resume, mode/model switching, MCP injection,
  writes, terminals, native session close, and unknown stable callbacks fail
  before authority widens.
- 29 focused adapter, protocol, and local-host tests pass without a Gemini
  binary or credential.
