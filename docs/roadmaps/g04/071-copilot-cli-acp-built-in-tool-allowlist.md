# g04.071 Copilot CLI ACP Built-In Tool Allowlist

Status: stopped after evidence
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Depends on: g04.040; per-route feature completion programme
Vision tags: explicit selection, provider truth, installed-route isolation
Contract refs: 011, 023, 029, 033, 037, 041, 044, 052
Research: 149, 188, 218

## Problem

Production route `copilot-cli.acp` owns one bounded Copilot CLI ACP stdio child
and one interactive session. It deliberately launches only
`copilot --acp --stdio`; consumers cannot restrict the provider-owned built-in
tools visible to that child.

Current official ACP-server documentation exposes server-start
`--available-tools` and `--excluded-tools` filters inherited by every session.
It says the available-tools allowlist takes precedence over exclusions. That
current documentation is a lead, not proof for exact qualified package
`1.0.80`, its tool identifiers, parser, registry, extensions, or failure
behavior. A flag name alone cannot establish a safe or useful allowlist.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind the smallest closed
Copilot-owned built-in tool allowlist on the existing one-child/one-session ACP
route. Do not expose raw tool strings, a denylist, consumer tools, MCP,
permission broadening, or an isolation claim.

## Goals

- [x] freeze current official documentation and exact `1.0.80` package/source
      evidence for parser syntax, delimiter rules, built-in tool identifiers,
      registry composition, precedence, defaults, and failures
- [x] identify a useful closed built-in allowlist whose membership and effect
      do not depend on model, account, extension, MCP, ambient configuration,
      or live-provider inference
- [x] distinguish requested restriction, startup dispatch, parser acceptance,
      registry filtering, permission request, tool invocation, and effect
- [x] promote Research 218 with an exact deliver-now table or honest empty set
- [ ] conditionally expose only a closed adapter-local selection admitted by
      Research 218
- [ ] bind that selection through prepared input, immutable evidence, driver,
      and exact child argv
- [ ] preserve current argv and behavior when the allowlist is absent
- [ ] preserve observe-and-stop permission handling and exact ACP lifecycle

## Non-Goals

- `--excluded-tools`, arbitrary raw tool names, generic tool/permission
  settings, or a portable provider-tool API
- consumer-declared `ToolCalls`, tool execution callbacks, MCP servers,
  extensions, plugins, skills, slash commands, or custom agents
- `--yolo`, `--allow-all`, persistent permission, one-shot auto-approval, or
  any widening of provider or host authority
- filesystem, descendant-process, network, sandbox, read-only, or
  `ProviderEnforced` isolation claims derived from tool filtering
- Copilot model selection, reasoning effort, TCP, login, BYOK, account
  inspection, billing, entitlement, or provider acceptance
- currentness, another route, release, merge, generation rollover, or g04
  closure

## Named Scope

The lane is restricted to route `copilot-cli.acp`, driver
`swallowtail.copilot-cli.acp`, axis `copilot-cli.package`, exact qualified
package `1.0.80`, behavior `copilot-cli.acp.stdio-v1`, ACP v1 stdio, host-
account access, and one bounded prepared interactive session.

Card 195 must freeze exact `@github/copilot@1.0.80` and platform-package
evidence for command parsing, value tokenization, normalization, duplicates,
unknown and empty values, available/excluded precedence, registry assembly,
built-in identifiers, extension/MCP/custom-agent contributions, and the point
where filtering is enforced. Current official documentation corroborates the
package; moving documentation does not qualify delivery.

The evidence must identify a closed useful subset, not merely prove that one
arbitrary string reaches argv. Built-in membership must be fixed independently
of selected model, account, extensions, MCP, user settings, and live service
state. If no such subset exists, Research 218 must record an empty deliver-now
set and the milestone stops.

The only candidate public shape is an adapter-local closed profile or typed
frozen identifier set selected by Research 218. Raw strings and shared
provider-tool vocabulary are forbidden. Omission must retain exact current
argv. An admitted selection is preparation-fixed for the owned child and fresh
context-losing replacement.

Tool filtering is a provider-native behavioral restriction under Contract
023. It does not prove process containment, filesystem/network restriction,
permission approval, or that a tool was invoked. Existing permission requests
remain observe-and-stop and are cancelled; the allowlist grants nothing.

## Execution Plan

### Batch 71.1 — Exact Allowlist Evidence

- [x] Execute card 195.
- [x] freeze exact package, parser, registry, identifier, filtering, lifetime,
      permission, and failure truth
- [x] promote Research 218 with a non-empty exact table or honest empty set

### Batch 71.2 — Conditional Adapter-Local Binding

- [ ] Execute card 196 only when Research 218 admits a non-empty set.
- [ ] bind only the exact version/profile/tool rows through prepared evidence
      and child argv

### Batch 71.3 — Route-Local Acceptance

- [ ] Execute card 197 only after card 196.
- [ ] prove dispatch, omission, rejection, replacement, permissions,
      lifecycle, docs, and API truth

## Evidence Stop

Research 218 admits no deliver-now row. Exact `1.0.80` parses
`--available-tools` through commander `[tools...]` plus `T5`/`xW` and stores
the list on ACP `session/new`. Bare or empty input collapses to omitted.
Unknown names warn rather than fail spawn. Documented identifiers are not a
closed JS table; bare names match any registry source; ACP still loads host
MCP and `github-mcp-server` when the client sends `mcpServers: []`; available/
excluded precedence is unfrozen. Current `copilot --acp --stdio` argv, unmapped
fixtures, observe-and-stop permission, and `AmbientHost` stay unchanged. Cards
196 and 197 are blocked.

## Acceptance Criteria

- [ ] only Research 218 deliver-now rows prepare
- [ ] input, plan/evidence, driver, and child argv agree exactly
- [ ] omission preserves the prior argv and route behavior
- [ ] unknown, duplicate, empty, unsupported, ambient, and drifting rows reject
      before process work when knowable
- [ ] allowlisting never becomes permission, consumer-tool, MCP, extension,
      network, filesystem, sandbox, or isolation authority
- [ ] existing initialize, session, prompt, activity, permission,
      cancellation, failure, replacement, and joined-cleanup truth remains exact
- [ ] docs claim no invocation, effect, safety, provider acceptance, account
      support, or isolation beyond frozen evidence
- [ ] default QA performs no install, login, account inspection, provider
      prompt, external tool work, or paid request

## Lane Runway

- predecessor: g04.067 OpenCode HTTP web-search evidence stop
- this milestone: Copilot CLI ACP built-in tool-allowlist evidence and
  conditional binding
- execution topology: one serial worker lane, cards 195-197
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact `1.0.80` parser, identifier, registry, filtering, precedence,
  or failure behavior cannot be frozen.
- Stop if useful membership depends on model, account, extension, MCP, custom
  agent, user configuration, live provider work, or mutable ambient state.
- Stop if unknown/unsupported names are silently ignored in a way Swallowtail
  cannot reject before spawn or if omission cannot remain exact.
- Stop if delivery needs raw strings, a denylist, generic tools/permissions,
  shared contract change, sibling-route promotion, currentness movement, or a
  breaking lifecycle.

## Batch Cards

- [195-copilot-cli-acp-built-in-tool-allowlist-evidence.md](batch-cards/195-copilot-cli-acp-built-in-tool-allowlist-evidence.md)
- [196-copilot-cli-acp-built-in-tool-allowlist-binding.md](batch-cards/196-copilot-cli-acp-built-in-tool-allowlist-binding.md)
- [197-copilot-cli-acp-built-in-tool-allowlist-acceptance.md](batch-cards/197-copilot-cli-acp-built-in-tool-allowlist-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 149 Copilot CLI ACP 1.0.80 Identity](../../research/149-copilot-cli-acp-1-0-80-identity.md)
- [Research 188 Copilot CLI ACP Effort](../../research/188-copilot-cli-acp-effort-evidence.md)
- [Research 218 Copilot CLI ACP Built-In Tool Allowlist](../../research/218-copilot-cli-acp-built-in-tool-allowlist-evidence.md)
- [Contract 023 Harness Operation Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 041 Input, Callback, And Provider-Tool Admission](../../contracts/041-input-callback-and-provider-tool-admission.md)
- [Copilot CLI ACP Prepared Integration](../../guides/copilot-cli-acp-prepared-integration.md)
- [GitHub Copilot CLI ACP Server](https://docs.github.com/en/copilot/reference/copilot-cli-reference/acp-server)
- [GitHub Copilot CLI Tool Permissions](https://docs.github.com/en/copilot/how-tos/copilot-cli/use-copilot-cli/allowing-tools)
