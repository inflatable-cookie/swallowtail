# 089 Post-Claude Maintenance Checkpoint And OpenCode Probe Truth

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

What consumer defect or upstream drift should follow Claude Agent `0.64.0`,
without reopening deferred Gemini work or chasing an immaterial release?

## Method

The checkpoint compared safe local `--version` observations, current official
release metadata, Swallowtail compatibility claims, optional live selectors,
and the latest Nucleus and Soundcheck adoption records. It ran no provider
prompt, authentication flow, model catalogue, session operation, workspace
mutation, installation, or update.

## Currentness Result

| Surface | Local observation | Current external point | Swallowtail boundary | Action |
| --- | --- | --- | --- | --- |
| Codex CLI | `0.146.0` | `0.146.0` | through `0.146.0` | none |
| Claude Code | `2.1.220` | `2.1.220` | exact `2.1.220` | none |
| Claude Agent ACP | `0.63.0` | `0.64.0` | through `0.64.0` | none |
| Gemini CLI | `0.53.0` | `0.53.1` | ACP `0.51.0`; headless through `0.52.0` | record only; future range work remains deferred |
| Kimi Code | `0.31.0` | `0.31.1` | through `0.31.1` | none |
| Grok Build | `0.2.117` | `0.2.117` | through `0.2.117` | none |
| Pi RPC | `0.83.0` | `0.83.0` | through `0.83.0` | none |
| Qwen Code | `0.21.2` | `0.21.2` | through `0.21.2` | none |
| OpenCode | `1.18.10` CLI | `1.18.10` | published server points through `1.18.10` | repair stale live probe |
| Antigravity CLI | `1.1.9` | tag `1.1.9` | exact `1.1.9` | none |
| Cursor Agent | `2026.07.01-41b2de7` | `2026.07.23-e383d2b` | both exact builds | none |

Gemini `0.53.1` is newer external evidence, not a selected implementation
lane. The deferred backlog gate remains unchanged and existing production
support remains intact.

The latest Nucleus and Soundcheck commits contain no recorded new Swallowtail
regression. Consumer adoption evidence therefore does not displace the
maintenance finding below.

## Probe Finding

`probe:opencode-installed` still requires the attached server to report exact
`1.14.48`. The production claim now contains 20 published segments through
exact `1.18.10` and permits later stable versions as visibly unverified newer.
The live selector therefore rejects the current guaranteed server before it
can test the frozen HTTP/SSE subset.

This is validation drift, not a production compatibility gap. The live probe
must:

- require a healthy response and a bounded valid semantic version
- classify the observation through `opencode_http_claim`
- accept qualified points and visible unverified-newer stable points
- reject below-baseline versions, unpublished gaps, prereleases, and malformed
  observations
- retain the existing OpenAPI and selected-path checks

An audit of attached-runtime probes found Ollama already classifies its
observed version through its compatibility claim. OpenCode is the only stale
exact-version assertion in this route family.

## Decision

Compile g03.016 as a bounded attached-probe truth repair. Do not extend an
adapter range, add authority, or require a live endpoint for deterministic
acceptance. Keep the operator-started live selector separately gated.

The isolated repair is now justified: every larger non-deferred range
candidate from Research 074 has closed, while the remaining selector directly
contradicts the current OpenCode guarantee.

## Contract Result

No contract change is required. Contracts 020, 029, and 037 already require
exact attached-server evidence, qualified versus unverified classification,
and no hard denial solely because a stable version is above the guaranteed
ceiling.

## Sources

- [OpenAI Codex npm metadata](https://registry.npmjs.org/@openai%2Fcodex/latest)
- [Claude Code npm metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code/latest)
- [Claude Agent ACP releases](https://github.com/agentclientprotocol/claude-agent-acp/releases)
- [Gemini CLI releases](https://github.com/google-gemini/gemini-cli/releases)
- [Kimi Code npm metadata](https://registry.npmjs.org/@moonshot-ai%2Fkimi-code/latest)
- [Grok npm metadata](https://registry.npmjs.org/@xai-official%2Fgrok/latest)
- [Pi npm metadata](https://registry.npmjs.org/@earendil-works%2Fpi-coding-agent/latest)
- [Qwen Code npm metadata](https://registry.npmjs.org/@qwen-code%2Fqwen-code/latest)
- [OpenCode releases](https://github.com/anomalyco/opencode/releases)
- [Antigravity CLI tags](https://github.com/google-antigravity/antigravity-cli/tags)
- [Cursor ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)

