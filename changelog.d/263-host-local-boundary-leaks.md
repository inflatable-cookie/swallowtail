# 263: Close claude-agent and grok host-local boundary leaks

- `swallowtail-adapter-claude-agent` and `swallowtail-adapter-grok` no longer
  depend on `swallowtail-host-local` in `[dependencies]`; it stays a
  dev-dependency for tests that compose a real local host. No adapter now has a
  normal internal edge into host support.
- `swallowtail-runtime` gains the provider-neutral ports both routes needed:
  `RegisteredToolCourierService` and `RegisteredToolCourierLaunch` for the
  mediated-stdio courier, and `ExpectedSource` plus `FrozenGuardedNodeCapture`
  for a frozen guarded capture. `REGISTERED_TOOL_PROXY_SERVER_NAME` and
  `REGISTERED_TOOL_PROXY_MCP_PROTOCOL_VERSION` move beside the existing wire
  tag; `swallowtail-host-local::wire` re-exports both unchanged.
- `GrokRegisteredToolBinding::with_host`,
  `ClaudeAgentSdkRegisteredToolBinding::with_host`,
  `ClaudeAgentSdkRegisteredOnlyBinding::with_host`,
  `ClaudeAgentSdkSessionPreparation::with_registered_tools` and
  `::with_registered_only` accept any `RegisteredToolCourierService`
  implementation, so an existing `LocalHostServices` argument still compiles.
  Their `host()` accessors return the port instead of a local-host type.
- `claude_agent_acp_guarded_v1_frozen` replaces
  `capture_claude_agent_acp_guarded_v1`: the adapter returns the frozen
  manifest, loader and digests it owns, and
  `swallowtail_host_local::capture_frozen_guarded_recipe` performs the capture
  under host custody. `claude_agent_acp_guard_v1_source_manifest` returns
  runtime `ExpectedSource` values.
- `LocalExpectedSource` is now a re-export of `ExpectedSource`, and
  `LocalHostServices` implements the courier port, so host-local call sites keep
  working.
