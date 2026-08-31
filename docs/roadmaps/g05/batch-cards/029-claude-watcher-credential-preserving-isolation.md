# 029 Claude Watcher Credential-Preserving Isolation

Status: ready
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../011-watcher-route-admission-recovery.md`
Depends on: completed card 026; Contracts 059-060; exact Claude Code `2.1.251`

## Goal

Replace the watcher-only `--bare` boundary only if exact provider-free evidence
proves a command shape that preserves configured Claude authentication while
still excluding ambient user, project, and local hooks, skills, MCP servers,
and settings.

## Scope

1. Freeze exact `2.1.251` help and prompt-free parser evidence for `--bare`,
   `--restricted`, `--setting-sources`, explicit `--settings`, `--mcp-config`,
   `--strict-mcp-config`, `--add-dir`, and the fixed built-in tool set.
2. Compare the current `--bare` command, a watcher-only `--restricted`
   replacement, and any smaller exact setting-source composition that does not
   reintroduce ambient authority.
3. Prove the selected candidate preserves the private MCP tools, Stop hook,
   injected skill, Plan mode, working-resource confinement, and unchanged
   watcher omission.
4. If one candidate closes those facts, change only the watcher command and
   deterministic fixtures. Preserve the normal non-watcher command.
5. Prove provider-free that unreserved MCP tools and ambient configuration stay
   unavailable and every terminal path joins and releases private material.
6. Reassess one later exact live turn. Do not authorize or run it here.

## Out Of Scope

- provider prompts, login, credential reads, paid work, or live acceptance
- watcher capability, version-range, Contract 059, or Contract 060 changes
- generic auth, settings, MCP, sandbox, restricted-mode, or route redesign
- skill inventory, feature-façade, currentness, papercut, or release work

## Acceptance Criteria

- the current auth failure and every candidate authority delta are explicit
- one candidate is selected only with exact ambient-isolation and private
  watcher composition evidence, or the lane stops honestly
- omission and the normal non-watcher command remain byte-for-byte unchanged
- no provider turn or credential is consumed

## Review Oracle

- **Invariant:** watcher opt-in admits only the operation-private MCP, Stop
  hook, injected skill, working resource, fixed built-ins, and one configured
  authentication path. It never admits ambient user, project, or local
  settings, hooks, skills, or MCP servers.
- **Smallest counterexamples:** a candidate preserves authentication by
  reopening any ambient setting source; a candidate preserves private watcher
  composition but still removes every configured authentication path; watcher
  omission or the normal non-watcher argv changes.
- **Expected stop:** reject the candidate before production editing when exact
  help/parser evidence cannot prove both authentication preservation and
  ambient exclusion without reading a credential or contacting the provider.
- **Required proof:** freeze exact candidate argv and setting-source deltas;
  exercise each counterexample in deterministic fixtures; prove the six
  reserved tools, Stop hook, skill, Plan mode, resource confinement,
  unreserved-tool rejection, terminal join, private-file cleanup, omission,
  and normal-command identity.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Stop for exact-head review and live-readiness reassessment.
