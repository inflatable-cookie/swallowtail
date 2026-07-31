# 083 Post-Qwen Pi RPC Maintenance Selection

Status: accepted
Owner: Tom
Updated: 2026-07-31

## Question

Should the next installed-harness maintenance tranche extend Pi RPC or extract
the standalone Claude Agent ACP work from paused roadmap g03.002?

## Current Evidence

| Candidate | Installed | Current official stable | Existing guarantee | Material delta |
| --- | --- | --- | --- | --- |
| Pi RPC | `0.83.0` | `0.83.0` | exact `0.80.10` | `0.81.0` adds thinking-level discovery and nested usage; `0.81.1` adds summarization-retry events; `0.82.0` correlates direct-bash updates; `0.83.0` changes direct-bash extension handling |
| Claude Agent ACP | `0.63.0` | `0.64.0` | `0.53.0..=0.61.0`, excluding `0.58.0` | `0.63.0` changes tool progress, denial, and nested-subagent correlation; `0.64.0` adds opt-in host-owned steering fallback |

Pi has six published stable points in scope:

- `0.80.10` — `8dc78834cde4e329284cf505f9e3f99763df5529`
- `0.81.0` — `9c480b6ad2c7419875a7a850fb4ad5f9232313b8`
- `0.81.1` — `20be4b18d4c57487f8993d2762bace129f0cf7c6`
- `0.82.0` — `083e61621276bff9f6faefab87ce07fcd98734e2`
- `0.82.1` — `b4f293684bba718d59cc1157679bcf6157b3a7f5`
- `0.83.0` — `845d6ff1f6643aba440341cce877ce1c43ebbc39`

The RPC type declaration changes at `0.81.0`, then remains byte-identical
through `0.83.0`. The RPC documentation adds summarization-retry events at
`0.81.1` and direct-bash update correlation at `0.82.0`; it is byte-identical
from `0.82.0` through `0.83.0`. Swallowtail disables automatic compaction and
retry, does not expose the direct `bash` RPC command, and disables extensions.
Those surfaces still need exact corpus classification, but they do not widen
the selected operation authority.

Pi's session-cwd source is byte-identical across the entire interval. That
confirms the existing load/resume resource-binding gate remains unresolved; it
does not block ephemeral catalogue, structured-run, or interactive RPC range
qualification.

## Decision

Select Pi RPC `0.80.10..=0.83.0` as roadmap g03.010.

This lane maximizes immediate installed proof: the exact current release is
already installed and its selected strict-LF RPC surface is inspectable without
provider authentication or a model prompt. Preserve baseline `0.80.10`, freeze
behavior milestones before widening the claim, and keep later stable versions
visible as unverified newer.

Leave standalone Claude Agent `0.62.0..=0.64.0` paused. It remains valuable,
but the installed wrapper stops at `0.63.0`; extracting it now would close less
of the local installed gap. Gemini maintenance remains paused.

## Contract Result

Contracts 011, 023, 028-029, 032-033, 037, 039, 041, and 044 already govern the
selected work. No new operation, authority, lifecycle, fallback, or portable
capability is required. Architecture changes only after the wider range is
implemented and validated.

Contract 017 continues to govern persisted Pi session attachment. Roadmap
g03.010 must not imply load, resume, provider-session management, sandboxing,
or direct-bash authority.

## Validation Needs

- freeze every stable package identity and selected-source milestone
- prove exact segment membership, exclusions, prerelease rejection, and later
  stable unverified posture
- classify summarization-retry and direct-bash records without exposing new
  commands
- prove the installed `0.83.0` executable through host-approved discovery
- run focused and extracted-package validation without a provider prompt

## Sources

- [Pi `0.83.0` release](https://github.com/earendil-works/pi/releases/tag/v0.83.0)
- [Pi `0.83.0` RPC documentation](https://github.com/earendil-works/pi/blob/v0.83.0/packages/coding-agent/docs/rpc.md)
- [Claude Agent ACP `0.64.0` release](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.64.0)
- npm metadata for `@earendil-works/pi-coding-agent@0.80.10` through `0.83.0`

