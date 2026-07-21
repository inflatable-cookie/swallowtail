# 012 Local Harness Orchestrator Isolation Posture

Status: promoted
Owner: Tom
Updated: 2026-07-21

## Question

Do applications that relay to installed coding harnesses normally impose an
outer process sandbox, and must Swallowtail require one before exposing a
harness route?

## Current Evidence

T3 Code is the closest current Nucleus analogue inspected. Its server launches
or connects to installed harnesses and maps one product runtime choice into
each harness's native controls. Its documented modes at source commit
`c0bb2373450231e3931937d2f703e35ce906ed85` are:

- **Full access**, the default: approval `never`, sandbox
  `danger-full-access`
- **Supervised**: approval `on-request`, sandbox `workspace-write`

The Codex adapter sends native Codex sandbox and approval values. Claude Code
uses Claude permission or bypass modes. OpenCode receives allow/ask permission
rules. ACP agents either receive approval or surface a permission callback.
No T3-owned filesystem or child-process sandbox was found around those
harnesses. Electron renderer sandboxing is a separate UI-process boundary.

Other maintained tools also expose provider-specific controls instead of one
portable outer sandbox:

- Codex has native read-only, workspace-write, and danger-full-access modes
- Claude Code has native sandboxing plus a distinct permission system
- Cursor exposes a provider-owned agent sandbox
- OpenCode uses tool permission rules; they are not an OS process boundary
- Cline treats permissions and sandbox use as configurable execution choices

This is a control-plane pattern. The application selects and reports the
harness posture; it does not become a replacement harness or silently claim
portable containment.

## Decision

Swallowtail does not require process containment for harness communication.
Every local harness route instead binds one explicit isolation posture:

- `AmbientHost`: the harness and descendants retain the execution host user's
  ambient process, filesystem, and network authority
- `ProviderEnforced`: the provider or harness supplies the exact claimed
  isolation behavior
- `HostEnforced`: the execution host supplies the exact claimed isolation
  behavior

Direct hosted inference and remote services with no local harness process have
no harness-isolation posture.

Isolation is opt-in and capability-specific. `ProviderEnforced` and
`HostEnforced` may be selected only when current evidence and deterministic
tests qualify the exact claim. `AmbientHost` is a supported first-class route,
not a failed sandbox. It must be visible to the consumer and cannot silently
replace either enforced posture.

A working directory, project reference, callback lease, tool permission,
approval mode, or denied search request remains useful policy. None of those
becomes a filesystem boundary under `AmbientHost`. The consumer owns whether
to offer or enable that route and how to explain its risk to users.

## Kimi Consequence

Research 011 disqualifies only the first `HostEnforced` Kimi profile. It does
not disqualify Kimi ACP communication. Kimi Code `0.28.1` may proceed as an
explicit `AmbientHost` route using the pinned executable, isolated
`KIMI_CODE_HOME`, delegated authentication, working-resource location, ACP
callbacks, and existing lifecycle exclusions.

The route makes no bounded filesystem, descendant, or provider-tool network
claim. A later Moonshot-supported sandbox, compatible host package, container,
VM, or remote contained host remains a separate configured capability.

## Promotion

- Contract 013: independent harness-isolation posture and ambient profile
- Contract 017: containment qualification applies only to an enforced claim
- system architecture: relay-first harness control and Kimi ambient route
- roadmap 018: card 057 closes as negative capability evidence; card 058
  resumes with explicit ambient scope

## Primary Sources

- [T3 Code runtime modes](https://github.com/pingdotgg/t3code/blob/c0bb2373450231e3931937d2f703e35ce906ed85/docs/architecture/runtime-modes.md)
- [T3 Code Codex runtime mapping](https://github.com/pingdotgg/t3code/blob/c0bb2373450231e3931937d2f703e35ce906ed85/apps/server/src/provider/Layers/CodexSessionRuntime.ts#L264-L335)
- [T3 Code Claude adapter](https://github.com/pingdotgg/t3code/blob/c0bb2373450231e3931937d2f703e35ce906ed85/apps/server/src/provider/Layers/ClaudeAdapter.ts#L3297-L3470)
- [T3 Code OpenCode runtime](https://github.com/pingdotgg/t3code/blob/c0bb2373450231e3931937d2f703e35ce906ed85/apps/server/src/provider/opencodeRuntime.ts#L328-L342)
- [Codex sandboxing](https://learn.chatgpt.com/docs/sandboxing)
- [Claude Code sandboxing](https://code.claude.com/docs/en/sandboxing)
- [Claude Code permissions](https://code.claude.com/docs/en/permissions)
- [Cursor agent sandboxing](https://cursor.com/blog/agent-sandboxing)
- [OpenCode tools and permissions](https://opencode.ai/docs/tools/)
- [Cline permission handling](https://docs.cline.bot/sdk/guides/permission-handling)
- [Cline CLI reference](https://docs.cline.bot/cli/cli-reference)
