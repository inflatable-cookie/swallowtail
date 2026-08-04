# 2026-08-05 Working-State Restoration Facade

Roadmap: `../roadmaps/g03/034-working-state-restoration-facade.md`
Cards: 087-089

## Changed

- promoted Research 104 and Contract 050
- added one consuming `PreparedWorkingStateRestoration` facade
- preserved session reconciliation, run reconciliation, and continuation
  recovery as distinct methods and outcomes
- added Claude Agent ACP and Kimi ACP continuation recovery through their exact
  existing load/replay paths
- wrapped Codex app-server, OpenCode HTTP, Kimi local server, OpenAI background,
  and Anthropic Managed Agents reconciliation
- added common public guidance and a compile-checked Claude Agent example

## Current State

Seven production routes expose `prepare_working_state_restoration`. Consumers
execute all supported methods through `restore` and match one portable outcome.
Route-specific preparation remains exact.

ACP recovery returns bounded replay and one live loaded session. It carries no
lost-turn state. Failed reconciliation never falls back to load.

No authenticated provider work, provider prompt, callback answer,
interruption, session mutation beyond deterministic ACP load fixtures, or live
network work ran.

## Validation

- `effigy validate:focused swallowtail-runtime`: 134 passed
- focused Claude Agent and Kimi validation: 175 passed
- final focused runtime, Codex, OpenCode, and Kimi validation: 504 passed
- final focused OpenAI, Anthropic, and Claude Agent validation: 171 passed
- affected-package proof passed for all seven changed packages
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes`
- `git diff --check`

`effigy package:api` reports the held `0.1.0` declaration baseline diff. The
new runtime and six adapter declarations are intentional; the same report also
contains pre-existing held-candidate drift in unchanged crates. The release
baseline was not rewritten.

## Next Move

Return g03 to its evidence gate. A consumer may adopt the common facade while
keeping route preparation and persistence exact.
