# 162 Claim Identity And Claim-Less Posture

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../053-claim-and-surface-consistency.md`
Depends on: card 161

## Goal

Standardize claim identity and window numbering, and settle the claim-less
adapter posture.

## Scope

1. Adopt one claim-id scheme across adapters (currently mixed dash and dot
   schemes, for example `alibaba-deployable-models-window-1` versus
   `antigravity.catalogue.release-window-1`) and one window numbering rule.
2. Renumber windows consistently where the audit found drift (kimi at
   `window-4`, pi and qwen at `window-2` for comparable claim ages).
3. Decide the posture for the claim-less adapters (bedrock, llama-cpp): add
   exact compatibility claims like their hosted peers, or record an explicit
   opaque-only disposition in the route matrix and version posture docs.
4. Update claim ids, windows, and any claim-based tests in one batch.

## Out Of Scope

- guaranteed range content, versions, or behavior changes
- public API changes

## Acceptance

- [x] every adapter claim follows one id scheme and window rule
- [x] claim-less adapters carry claims or an explicit recorded disposition
- [x] route and feature matrices stay consistent with claims

## Closeout

Operator decisions: dot-scheme claim ids; opaque-only disposition recorded
for bedrock and llama-cpp (no claims added).

### Claim ids (dot scheme `product.axis.window-N`)

- converted dash ids: `alibaba-model-studio.deployable-models-window-1`,
  `kimi-platform.chat-window-1`
- converted range ids: `gemini-cli.acp.window-1`,
  `gemini-cli.headless.window-1`, `claude-agent.acp.window-2`,
  `claude-code.headless.window-1` (protocol-acp test reference updated)
- drift correction: `kimi.acp.executable-window-4` -> `window-2`,
  `kimi.headless.executable-window-3` -> `window-2`,
  `kimi.local-server.executable-window-4` -> `window-2`, matching pi and
  qwen at `window-2` for comparable claim ages
- the frozen 0.31.1 corpus fixture and historical logs keep their recorded
  ids unchanged (historical evidence, not current-source identity)

### Card-161 relabels (automatic per-revision rule)

- pi: 0.80.10-0.82.1 segments -> `Deprecated` (0.83.0 stays `Maintained`)
- qwen: baseline segment -> `Deprecated`
- claude-agent: 0.53.0-0.63.0 segments -> `Deprecated` (0.64.0 stays
  `Maintained`)
- grok: baseline..0.2.116 -> `Deprecated` (0.2.117 stays)
- codex app-server: base 0.110.0-0.130.0 -> `Deprecated` (workspace stays)
- kimi acp: legacy-reasoning exact -> `Deprecated`
- kimi local-server: baseline, 0.29.0, 0.29.1-0.30.0, 0.31.0 ->
  `Deprecated` (refresh-stable 0.31.1 stays)

### Claim-less disposition

Recorded in the provider route matrix (Claim-Less Disposition) and
Contract 029: bedrock and llama-cpp bind exact opaque revisions only and
carry no compatibility claim by explicit recorded disposition.

### Validation

- focused tests across 16 touched packages: 81 suites green; codex
  `version_range` updated to the decided semantics (0.122.0 is `Deprecated`
  on app-server, `Maintained` on exec)
- clippy clean on all touched adapters
- `effigy qa:routes` passed (route, lifecycle, 27-solution/34-route feature,
  and activity matrices); `effigy check:examples` clean

## Stop Conditions

- stop if renumbering changes claim meaning or support truth

## Auto-Continuation

Yes, to card 163 after acceptance.

## Validation

- `effigy qa:routes`, `effigy check:examples`
- focused validation for every touched adapter
