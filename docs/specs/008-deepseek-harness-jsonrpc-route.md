# 008 DeepSeek Harness JSON-RPC Route

Status: draft
Owner: Tom
Updated: 2026-08-17

## Purpose

Record the settled JSON-RPC subset for DeepSeek Harness so g03 can add one
installed-harness route without flattening it onto Open Platform continuation
or inventing ACP/Web drivers.

Research 124 already answers whether the harness qualifies. This spec holds
the first production subset, identities, and exclusions until cards promote
them into architecture, package topology, and public route truth.

## Scope

In:

- new package `swallowtail-adapter-deepseek-harness`
- one route `deepseek-harness.jsonrpc` / driver
  `swallowtail.deepseek-harness.jsonrpc`
- exact runtime-bin `0.1.0rc6` on Apple Silicon
- owned-process NDJSON JSON-RPC 2.0
- one structured run: initialize, prompt, idle fold, shutdown or kill
- projection of turn/step lifecycle, assistant text, content-free reasoning
  progress, harness-owned tool call/result, usage, and terminal completed /
  error
- host-approved executable, Cordis config, cwd, provider, and model as
  immutable prepared evidence

Out:

- `deepseek.continuation` changes
- ACP, headless CLI, and Web `/api` routes
- session-id interactive continuity
- subagent topology
- model catalogue, load, import, archive
- native protocol cancel or per-session close
- DeepSeek-official SSE-specific behavior
- consumer tools, questions, permission exchange
- default `danger-full-access`
- version bump, tag, or registry publication
- unverified-newer while the product is an RC

## Decisions Needed

Settled 2026-08-17 from Research 124 and operator direction:

1. identity — new family/package/route; not an extension of
   `swallowtail-adapter-deepseek`
2. first surface — JSON-RPC stdio; ACP and Web `/api` later and distinct
3. pin — exact `deepseek-harness-runtime-bin==0.1.0rc6` plus payload digest
   `ac1c91462518427467bd0a0ca3bf1049df62be0dbe8b0ee8014c6761cb8f80bf`;
   `serverInfo.version` (`0.0.1`) is not the version axis
4. first role — structured run only
5. cancel — force-stop the owned process; do not advertise a wire cancel
6. live gate — first authenticated/live proof may use host-local Ollama
   through `dsh-llm-pi-ai`; that does not qualify `deepseek-official`
7. tools — observe harness-owned `bash` / `str_replace_editor` when the
   approved composition mounts them; do not ingest argument or result bodies
8. composition — host supplies Cordis path; Swallowtail does not ship a
   danger-full-access default

No remaining product-policy forks block the first tranche.

## Acceptance Criteria

- [x] Research 124 remains the evidence owner for the pin and live smokes
- [x] milestone g03.069 sequences corpus, driver, facade, and acceptance
- [x] public route truth keeps `deepseek.continuation` unchanged
- [x] first live selector is separately gated and credential-honest
- [x] redacted fixtures contain no prompts, reasoning text, tool bodies, or
      secrets

## Promotion Targets

- `docs/architecture/system-architecture.md` and
  `docs/architecture/release-and-package-topology.md` on package acceptance
- Contract 036 package list on package acceptance
- route, feature, and activity matrices plus a canonical guide
- `docs/roadmaps/g03/069-deepseek-harness-jsonrpc-foundation.md`
- Research 124 promotion into logs on close
