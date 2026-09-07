# 125 Claude SDK Registered-Tool Route Binding

Status: ready
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: cards 114, 115, 116 merged; the 2026-09-07 reconciliation capsule (cell 1); Contract 063 Mediated Stdio Proxy Attachment

## Goal

Close the Claude SDK's registered-tool cell: today the card 116 carrier and mediator exist but no route API binds a `RegisteredToolPreparation` into open, so Desktop cannot register a tool on `claude-agent.sdk`. Add the route binding on the Codex precedent (`CodexSessionProfileInput::with_registered_tools`, `CodexAppServerDriver::with_registered_tools`).

## Scope

1. `ClaudeAgentSdkSessionProfile::with_registered_tools(RegisteredToolPreparation)` (or the exact preparation object the route uses) and a `ClaudeAgentSdkRegisteredToolBinding::qualify` step mirroring Codex, binding the prepared registration, the `MediatedStdioProxy` attachment, and the resolved `RegisteredToolProxyRecipe` (`RegisteredToolSelection::with_proxy_recipe`) into open.
2. Open sequence per Contract 063: bridge lease minted, rendezvous materialised, the courier declared to the SDK as the reserved `swallowtail-registered-tools` stdio server under `strictMcpConfig`, ready only after the courier authenticates; every registered call through `canUseTool` mediation and the kernel's live verdicts; close joins the single listener. Omission byte-identical.
3. Contract 061 projection: `claude_agent_sdk_registered_tool_qualification()` stays `Unqualified / real_route_gate_pending` until the live gate; this card adds the callable seam and the provider-free proof only, and states that in the row reason.
4. Fake-SDK fixtures: register one bounded tool, observe the courier declaration, one mediated call reaching `RegisteredToolDispatcher::dispatch` through the proxy and one result correlated back, deny path, cancellation during a registered call, close and join; F1–F18 remain green on the mounted path.
5. Guide section, changelog `[Unreleased]`, additive baseline.

## Out Of Scope

The live real-route gate; card 084 consumer-declared servers (unchanged); HTTP/SSE/in-process/managed MCP.

## Acceptance Criteria

- [ ] a consumer can bind a `RegisteredToolPreparation` into a Claude SDK open through a public route API named on the Codex precedent
- [ ] one mediated registered call round-trips through the proxy on the fake SDK; deny, cancel, and close proven
- [ ] Contract 061 row unchanged at `Unqualified / real_route_gate_pending` with the reason text updated to "callable seam present; live gate pending"
- [ ] omission byte-identical; guide, changelog, baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: no registered tool executes on this route except through the kernel-admitted courier path, and the route claims nothing beyond the fixture. Smallest counterexample: a registered call that reaches the dispatcher without a `BeforeDispatch` verdict.

## Stop Conditions

The SDK cannot be given the courier as a strict stdio server at open without an option outside the frozen 0.3.259 declaration (record; return to Chatterbox).

## Auto-Continuation

No. Stop for exact-head review. Provider-free only; the real-route gate runs under separate operator authority.
