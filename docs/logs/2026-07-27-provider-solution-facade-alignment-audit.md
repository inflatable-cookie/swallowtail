# Provider Solution Facade Alignment Audit

Date: 2026-07-27

## Question

Does the solution-level feature CSV group routes only when the public adapter
API exposes one solution facade over typed route branches?

## Findings

| Provider surface | Public code shape | CSV disposition |
| --- | --- | --- |
| Amazon Bedrock catalogue and Runtime | one `prepare_bedrock` facade; separate typed branches | one Amazon Bedrock row |
| Codex exec and app-server | one `prepare_codex` facade; explicit `StructuredExec` or `AppServer` selection | one Codex row |
| Kimi Code ACP and local server | separate preparation, access, transport, lifecycle, and management authority | separate rows |
| llama.cpp attached and owned | deliberately separate topology, artifact, serving, and stop authority | separate rows |
| OpenAI background and Realtime APIs | separate hosted operation shapes and facade axes | separate rows |
| Anthropic Messages, Managed Agents, and Claude Agent ACP | separate direct API, provider-hosted harness, and installed harness products | separate rows |
| Gemini CLI ACP and Gemini Live | separate installed harness and hosted realtime products | separate rows |
| Qwen Code and Alibaba Model Studio | separate installed harness and hosted cloud products | separate rows |
| Kimi Code and Kimi Platform | separate membership and Platform access, billing, and support authority | separate rows |

No further route grouping is justified by shared corporate ownership or crate
placement alone.

## Corrections

- Combined the two Codex CSV rows because code already exposes one explicit
  facade selection.
- Kept both Codex route and driver identities visible in the combined row.
- Made the CSV grouping rule explicit in the route-matrix guide.
- Repaired current architecture and README route counts after the additive Kimi
  local-server route.

The route matrix remains route-level with 23 rows. The feature CSV is
solution-level with 21 rows covering the same 23 unique route identities.

## Boundary

A future composite facade may justify another grouped solution row only after
the public code exists. The CSV does not lead architectural consolidation.
Provider name, adapter crate, compatible wire shape, or shared executable
version is insufficient evidence.
