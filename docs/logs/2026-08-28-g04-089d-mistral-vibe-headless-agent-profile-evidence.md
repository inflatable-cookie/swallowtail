# 2026-08-28 g04.089d Mistral Vibe Headless Agent-Profile Evidence

Status: complete
Card: 255
Research: 252

## Boundary

Evidence only. This lane owns card 255, Research 252, this log, and optional
new Mistral Vibe-local frozen evidence. Shared planning and production stay
unchanged.

## Target

Close exact profile membership, resource/tool authority, application,
terminal, lifecycle, cleanup, and omission truth.

## Finding

Honest empty deliver-now set.

Exact `mistral-vibe.headless` `2.24.2` freezes builtin `--agent` membership
and profile overrides from tagged `models.py`. Fixed Plan stays the only
Swallowtail argv agent. Stops that closed the set:

1. `ask` drops Plan's plans-directory `read_file` allowlist and replaces
   write/edit `never` with tool-default `ask`.
2. `accept-edits` sets `write_file`/`edit` to `always` — wider write.
3. `auto-approve`, `--auto-approve`, and `--yolo` stay excluded.
4. Parser accepts any `NAME`, including ambient custom agents;
   `enabled_agents` / `disabled_agents` gate availability.
5. Headless `ASK` → `deny_callback` is not Plan-equivalent `NEVER` authority.
6. `AgentSafety` is UI-only; frozen streaming corpus has no applied-agent
   confirmation before provider effects.

Omission retains exact `--agent plan`. Invalid/excluded/subagent-as-primary
names raise at `AgentManager` before `act(prompt)`.

## Evidence

- Research 252 promoted empty set
- `crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-2.24.2-agent-profiles/`
- tagged sources at `v2.24.2` /
  `5e6aa0f6beb3454454f4c1de74a7652ba577ab05`

## Validation

```sh
effigy validate:focused swallowtail-adapter-mistral-vibe
effigy qa:northstar
git diff --check
```

## Unresolved / Later

Reopen only if an exact package point proves a non-widening profile with
closed tool/resource authority, pre-effect rejection, applied-agent
confirmation without provider work, and unchanged terminal lifecycle. No
production binding from this lane.
