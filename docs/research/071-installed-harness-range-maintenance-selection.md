# 071 Installed Harness Range Maintenance Selection

Status: promoted
Owner: Tom
Updated: 2026-07-30

## Question

Which installed harness ranges should advance after Grok g02.043, without
turning every upstream release into a Swallowtail release or one-card churn?

## Method

Evidence was accessed on 2026-07-30.

- probed only installed executable versions
- checked current npm stable tags and exact publication records
- reused Research 064's exact Codex `0.146.0` tagged-source and activity delta
- compared OpenCode tags `1.18.4..=1.18.10`
- compared OpenCode's full OpenAPI artifact, selected HTTP/SSE routes, session
  implementation, deletion implementation, authorization middleware, and
  server tests
- kept downstream provider transforms and harness prompt text outside the
  client-server interface claim

No provider prompt, login, credential read, account mutation, installation,
update, container, model request, or consumer edit ran.

## Current Observations

| Route | Guaranteed upper | Current stable | Local observation | Decision |
| --- | --- | --- | --- | --- |
| Codex exec and app-server | `0.145.0` | `0.146.0` | `0.146.0` | qualify after focused conformance |
| OpenCode HTTP/SSE | `1.18.4` | `1.18.10` | `1.18.9` | qualify exact `1.18.5..=1.18.10` after corpus extension |
| Grok Build ACP | `0.2.114` | `0.2.114` | `0.2.114` | no movement |
| Kimi Code routes | `0.31.0` | `0.31.0` | `0.31.0` | no movement |
| Claude Code headless | `2.1.220` | `2.1.220` | `2.1.220` | no movement |

Grok alpha `0.2.116` remains a prerelease and does not create a stable
milestone.

## Codex Delta

Research 064 already freezes exact `0.146.0` source and fixtures.

The release adds:

- optional `pluginId` and `scriptPath` command-action fields
- a deferred-query web-search lifecycle

Both are additive under the existing app-server and exec decoders. Existing
prepared profile tests already prove that `0.146.0` can execute under visible
unverified-newer posture without gaining broader disclosure.

Qualification still requires the full selected exec, app-server, lifecycle,
continuity, discovery, prepared, and activity assertions. Exact prereleases
remain rejected. A synthetic later stable point remains unverified newer after
the upper bound moves.

## OpenCode Delta

Stable releases `1.18.5` through `1.18.10` are contiguous and published.

The exact selected source is unchanged from `1.18.4` through `1.18.10`:

- `GET /global/health`
- `GET /provider`
- `POST /session`
- `POST /session/{sessionID}/prompt_async`
- `GET /event`
- `POST /session/{sessionID}/abort`
- `DELETE /session/{sessionID}`
- session implementation and deletion behavior
- optional Basic authentication middleware
- selected HTTP session tests

The full OpenAPI SHA-256 remains
`063e1cc745665f3846be7911e1eb793dcfe45bca5ae3cc425ab246b80eeec4ce`
except at `1.18.8`.

Exact `1.18.8` adds optional `iss` to an OAuth callback request outside the
selected transitive closure. Exact `1.18.9` reverts it. That point needs a
distinct full-artifact record but not a new selected-surface behavior
revision.

Provider transform logic and harness-owned prompt text changed. Those changes
belong to OpenCode's downstream provider behavior, not the selected
`opencode.server` client protocol. They do not authorize model, provider,
credential, endpoint, generation, or retry fallback.

## Maintenance Decision

Select one batched range-maintenance roadmap:

1. extend Codex through exact `0.146.0`
2. extend OpenCode through exact `1.18.10`
3. close through focused cross-host, package, route, and release-note evidence

This is normal Contract 029 maintenance:

- no Swallowtail release is required now
- later stable points remain executable as visible unverified newer
- qualification changes only after exact corpus and conformance evidence
- baselines do not move
- no common API or compatibility shim is added

## Contract Fit

Contracts 029, 032, 036, 037, 038, 039, 040, and 044 are sufficient.

No new provider-neutral record or provisional spec is justified. A selected
surface, lifecycle, access, cleanup, or activity break is a stop condition and
requires a new research or contract delta before implementation continues.

## Sources

- [Official Codex app-server documentation](https://developers.openai.com/codex/app-server/)
- [Official Codex non-interactive documentation](https://developers.openai.com/codex/noninteractive/)
- [Official Codex releases](https://github.com/openai/codex/releases)
- [Codex `0.146.0` source](https://github.com/openai/codex/tree/rust-v0.146.0)
- [Official Codex npm package](https://www.npmjs.com/package/@openai/codex)
- [OpenCode releases](https://github.com/anomalyco/opencode/releases)
- [OpenCode `1.18.4` source](https://github.com/anomalyco/opencode/tree/v1.18.4)
- [OpenCode `1.18.10` source](https://github.com/anomalyco/opencode/tree/v1.18.10)
- [OpenCode server documentation](https://opencode.ai/docs/server/)
- [OpenCode npm package](https://www.npmjs.com/package/opencode-ai)

## Promotion

- roadmap g02.044 owns the selected maintenance tranche
- card 146 records currentness and corpus authority
- card 147 owns Codex qualification
- card 148 owns OpenCode qualification
- card 149 owns cross-route closeout
