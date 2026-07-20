# Nucleus Adoption Handoff

Date: 2026-07-19
Status: recorded

## Result

g01 card 029 and roadmap 008 are complete.

- current Nucleus sources were inspected read-only; no consumer file changed
- the downstream plan replaces Codex transport mechanics behind the existing
  `AgentSessionRuntime` and live registry rather than replacing Nucleus control
- Nucleus retains project/resource authority, tools, receipts, persistence,
  tasks, Goals, memory, review, UI, and broader supervision
- the first slice runs only on the embedded authority host and rejects remote
  placement until Nucleus supplies a real host invocation route
- tool-enabled persisted chats migrate transcript context into a fresh session;
  provider-thread-only resume is not treated as a complete resume binding
- provider, runtime, callback, session, turn, task, and receipt identities are
  mapped as distinct namespaces
- exact-revision pinning, a temporary feature gate, downstream validation,
  rollback, and legacy removal are explicit

## Evidence

- `docs/roadmaps/g01/nucleus-adoption-handoff.md`
- current Nucleus `nucleus-agent-protocol`, `nucleus-agent-adapters`,
  `local_codex_chat`, `project_resource_target`, and desktop Tauri sources
- Swallowtail Contract 012, Codex app-server callback tests, and local/remote
  topology fixtures
- full Effigy QA

## Next Lane

The operator must choose the first non-Codex harness proof from promoted
integration evidence. No implementation card is ready until that choice is
made.
