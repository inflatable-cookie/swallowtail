# 083 Claude SDK Resume And Session Listing

Status: complete; PR 244 merged at `7cb08b1f`
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-06
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: card 082 merged; Contract 017 (new, load, resume, recovery attachment); Research 280 (`0.3.259`)

## Goal

Resume and `resumeSessionAt` on `claude-agent.sdk` as Contract 017 **resume** (attach without replay), with cwd and account rebinding verified from the host lease and init evidence, plus a session listing for the consumer's transcript store. Fork optional, only if the SDK proves it on `0.3.259`.

## Scope

1. Freeze first, from the pinned `0.3.259` package tree (Research 280 method): the exact option and function surface for `resume`, `resumeSessionAt`, `forkSession`, `persistSession`, and any session-listing or session-message export. Record it in the card Result with file anchors before writing Rust. If listing is absent or unstable on `0.3.259`, listing stays out and the Result says so.
2. Persistence is currently forced off (`persistSession: false` in the sidecar). Resume requires provider-stored session state. Make persistence an explicit per-profile opt-in on the prepared session (`ClaudeAgentSdkSessionProfile`), default unchanged (off). Record where the provider stores it; Swallowtail never reads or rewrites that store.
3. Implement `resume_session` on the SDK driver (today `unsupported("session resume")`, `sdk/driver.rs`) as Contract 017 resume: attach to the bound provider session by `SessionResumeBinding`; no replay; cwd re-derived only from the resolved host lease (never from list metadata or the provider store); account rebinding verified against `accountInfo` and `system/init` evidence from the initialize-first path; mismatch fails typed (`resume_cwd_mismatch`, `resume_account_mismatch`, `resume_session_unknown`).
4. `resumeSessionAt` as the same operation with the message boundary as an additive request input; boundary outside the session fails typed.
5. Session listing, if frozen as available: a route-local prepared query returning provider session ids, cwd, timestamps, and title only; no message bodies; never a resume authority on its own.
6. Fake-SDK proofs: resume attaches and the first turn continues context; wrong cwd and wrong account fail closed before any turn; listing round-trips; persistence-off sessions cannot be resumed and say why.
7. Guide section, `claude-agent.sdk` matrix cells (the "not applicable (27)" listing row moves for this route), changelog `[Unreleased]`, additive baseline in the working baseline directory.

## Out Of Scope

Load with replay (Contract 017 load); recovery attachment; consumer transcript reconstruction; any change to the default profile; MCP; version pins.

## Acceptance Criteria

- [ ] the `0.3.259` resume/listing surface is frozen with anchors before implementation
- [ ] persistence is opt-in per profile and the default profile is byte-identical in behaviour
- [ ] resume rebinding takes cwd only from the host lease and fails typed on cwd or account mismatch
- [ ] no replay is claimed; the operation is documented as Contract 017 resume
- [ ] fake-SDK proofs cover attach, both mismatches, unknown session, and persistence-off
- [ ] guide, matrix, changelog, additive baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: a resumed session runs in exactly the leased cwd under exactly the verified account, or does not run. Smallest counterexample: a resume whose cwd came from the provider's stored session record.

## Stop Conditions

The SDK cannot report the session's account or cwd on resume in any observable way (record; return to Chatterbox rather than trust the store).

## Auto-Continuation

No. Stop for exact-head review.

## Result

Static freeze completed before implementation from `@anthropic-ai/claude-agent-sdk@0.3.259`, npm package shasum `daf465f8231392ab99e1c7fc7f1e14c3d25ea012`.

Frozen anchors in `sdk.d.ts`:

- `persistSession?: boolean` at lines 1674-1680; `resume?: string` at 1907-1910; `sessionId?: string` at 1911-1916; `resumeSessionAt?: string` at 1917-1924.
- `forkSession` at lines 724-738, returning a new resumable session id through `query({ options: { resume: sessionId } })`.
- `getSessionInfo` at lines 759-767 and `getSessionMessages` at lines 787-797; message retrieval is outside this card.
- `listSessions` at lines 973-992, with `dir`, `limit`, `offset`, `includeWorktrees`, `includeProgrammatic`, and `sessionStore` options at lines 997-1034.
- `SDKSessionInfo` at lines 4987-5031: required `sessionId`, `summary`, and `lastModified` at lines 4990-5002; optional `fileSize`, `customTitle`, `firstPrompt`, `gitBranch`, `cwd`, `tag`, and `createdAt` at lines 5004-5030. The route projects only the bounded `sessionId`, leased `cwd`, `createdAt`, `lastModified`, and title fields.

Decision: resume, `resumeSessionAt`, and `listSessions` are present in the pinned artifact, and the `SDKSessionInfo` record shape is frozen above. This card implements resume and the bounded listing surface using the leased cwd; listing returns provider session id, cwd, timestamps, and title only. `getSessionMessages` and `forkSession` remain out of scope. `sessionStore` is not used. The solution feature matrix retains `resume_session=No` and `provider_session_catalogue=No`: this route's exact-binding resume is adapter-local rather than shared load/catalogue authority, and listing is display-only. The lifecycle matrix keeps the persistent-session posture `unsupported` because the route has no management binding, archive, restore, or delete surface. The shared provider-route inventory scripts are outside this card's owned paths and remain unchanged; the owned matrix cells are aligned to their current audited expectations. After the post-Card105 rebase, the current v0.4.3 working API baseline absorbs only Card083's additive exports; the tagged v0.4.2 and v0.4.3 trees remain unchanged. No provider calls or credentials were used for this freeze.
