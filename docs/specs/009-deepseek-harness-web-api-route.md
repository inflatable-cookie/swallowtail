# 009 DeepSeek Harness Web `/api` Route

Status: draft
Owner: Tom
Updated: 2026-08-17

## Purpose

Record the settled Web `/api` subset for DeepSeek Harness so g03 can add one
local-server route without flattening it onto JSON-RPC, ACP, or Open Platform
continuation.

Research 125 answers whether the surface qualifies. This spec holds the first
production subset, identities, and exclusions until cards promote them into
architecture, package topology, and public route truth.

## Scope

In:

- second route on `swallowtail-adapter-deepseek-harness`
- route `deepseek-harness.local-server` / driver
  `swallowtail.deepseek-harness.local-server`
- exact npm `@deepseek-ai/dsh@0.1.0-rc.6` on axis `deepseek-harness.web`
- owned `dsh web` on loopback HTTP + WebSocket
- method allowlist: list, search, create, history, models, prompt, cancel,
  fork, workspace list, archive, `host.describe`
- native turn cancel, fork, and archive
- host-approved Cordis patch, cwd, provider, and model as prepared evidence

Out:

- changes to `deepseek-harness.jsonrpc` or `deepseek.continuation`
- ACP, headless CLI, and the browser UI as a driver
- settings, credentials, llm configuration, directory picker, filesystem
  open, preset authoring, ZIP export
- attachments, queue, subagents, skills, goals, commands
- restore, hard-delete, non-loopback bind, bearer auth
- Contract 054 support until history proof
- DeepSeek-official SSE-specific behavior
- default `danger-full-access`
- version bump, tag, or registry publication
- unverified-newer while the product is an RC

## Decisions Needed

Settled 2026-08-17 from Research 125 and operator direction:

1. identity — second route in the existing Harness package; not an extension
   of JSON-RPC or `swallowtail-adapter-deepseek`
2. keep JSON-RPC — one-shot stdio structured run stays qualified
3. pin — exact `@deepseek-ai/dsh@0.1.0-rc.6` on `deepseek-harness.web`;
   launcher `-V` and `host.describe` are not the version axis
4. bind — loopback only; Host/Origin fence; no credential lease
5. first roles — catalogue, history candidate, structured prompt with native
   cancel, fork, archive
6. deny list — credentials, settings, llm.*, host filesystem, export ZIP
7. live gate — first live proof may use host-local Ollama; that does not
   qualify `deepseek-official`
8. composition — host supplies Cordis patch; Swallowtail does not ship a
   danger-full-access default

No remaining product-policy forks block the first tranche. Corpus must still
prove `session.history` does not resume an Agent before Contract 054 can
move.

## Acceptance Criteria

- [x] Research 125 remains the evidence owner for the pin, fence, and
      allowlist
- [x] milestone g03.070 sequences corpus, driver, facade, and acceptance
- [x] public route truth keeps `deepseek-harness.jsonrpc` unchanged
- [x] first live selector is separately gated and credential-honest
- [x] redacted fixtures contain no prompts, reasoning text, tool bodies,
      secrets, or raw export bytes

The deterministic acceptance surfaces are in place. Card 225 still has one
operator-gated installed/live smoke pending exact local `dsh`, Cordis, cwd,
provider, and model inputs; that gate does not promote this draft to a live
provider qualification.

## Promotion Targets

- `docs/architecture/system-architecture.md` and
  `docs/architecture/release-and-package-topology.md` on package acceptance
- Contract 036 package and route inventories on package acceptance
- route, feature, and activity matrices plus a canonical guide
- `docs/roadmaps/g03/070-deepseek-harness-web-api-foundation.md`
- Research 125 promotion into logs on close
