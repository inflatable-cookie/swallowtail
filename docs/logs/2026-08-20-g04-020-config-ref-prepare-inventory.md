# g04.020 Config-Ref Prepare Inventory

Date: 2026-08-20
Roadmap: `../roadmaps/g04/020-config-ref-prepare-handoff.md`
Card: `../roadmaps/g04/batch-cards/056-prepare-input-versus-stored-refs.md`

## Inventory

| Addable route | Stored credential refs | Stored config refs | Prepare input duplicated before this lane |
| --- | --- | --- | --- |
| Anthropic Messages | `api_key` → `CredentialRef` | `endpoint` → `ConfigFieldRef` | API-key reference in `AccessProfile`; host `InstanceTargetRef` |
| DeepSeek continuation | `api_key` → `CredentialRef` | `endpoint` → `ConfigFieldRef` | API-key reference in `AccessProfile`; host `InstanceTargetRef` |
| Codex app-server | none; cached local login | `binary_path`, `environment` → `ConfigFieldRef` | host `InstalledExecutableTarget` and `EnvironmentRef` |
| Claude Agent ACP | none; inherited local subscription | `binary_path`, `environment` → `ConfigFieldRef` | host `InstalledExecutableTarget` and `EnvironmentRef` |
| Ollama attached | none; local unauthenticated | `endpoint` → `ConfigFieldRef` | host `InstanceTargetRef`; model tag and manifest digest stay explicit prepare-time inputs |
| llama.cpp attached | none; local unauthenticated | `endpoint` → `ConfigFieldRef` | host `InstanceTargetRef`; exact build/commit stays adapter-owned prepare-time evidence |

The stored credential and config values are already opaque. The missing seam is
the route-local mapping from admitted field ids to the typed host references
that Contract 037 binds during preparation. A consumer should not retain a
second target or environment reference beside the admitted record.

## Contract Fit

Contract 057 needs a durable handoff rule: the host resolves each admitted
opaque field reference into the typed target or environment reference required
by the selected adapter. The resolved value remains host-private and is used
only for preparation and later host-authorized operation work. Contract 037
still starts after admission and binds the exact resolved target, host,
configured instance, route, and evidence. Admission must not prepare.

Contract 047 remains free of credential references, config references, target
references, paths, URLs, environment bodies, and executable authority. No
overlay or presentation field is involved.

## Validation

Card 056 validation is run after this inventory is added:

- `effigy qa:docs:index:logs`
- `git diff --check`
