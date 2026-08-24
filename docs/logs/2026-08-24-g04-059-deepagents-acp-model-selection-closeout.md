# 2026-08-24 g04.059 Deep Agents ACP Model Selection Closeout

Status: complete
Owner: Tom
Milestone: g04.059
Cards: 164 complete; 165-166 blocked

## Result

Research 206 is an honest empty deliver-now set. Exact `deepagents-acp@0.1.25`
and current official docs advertise `--model` / `-m` and `provider:model`, but
authorized evidence does not freeze pre-spawn provider/access agreement,
fail-closed invalid handling, or ACP model confirmation. Cards 165-166 stay
blocked. No production code. No public API change. g04 stays open.

## Evidence Table

| Provider lead | Model form | Access before spawn | Fail-closed invalid | ACP confirmation | Deliver-now |
| --- | --- | --- | --- | --- | --- |
| Anthropic / bare `claude*` | bare or `anthropic:…` | no — generic profile | no | no | no |
| OpenAI | `openai:…` | no | no | no | no |
| Other docs providers | `provider:model` | no | no | no | no |
| Omission | CLI internal default | n/a | n/a | n/a | n/a |

Empty-set basis (authorized):

1. generic `deepagents_provider_api_key_access_profile` cannot prove
   Anthropic vs OpenAI (or other) agreement before spawn without env
   inspection
2. CLI accepts any string and silently retains
   `"claude-sonnet-4-5-20250929"` when `--model` lacks a usable value
3. `createDeepAgent` / `initChatModel` run after spawn at `session/new`
4. initialize / `session/new` expose no model field; `/status` text is not
   Swallowtail confirmation

Exact artifact digests match Research 157. CLI default
`"claude-sonnet-4-5-20250929"` disagrees with `deepagents@1.12.4`
`createDeepAgent` default `"anthropic:claude-sonnet-4-6"`. Omission retains
current empty argv and does not acquire an upstream default-model claim.

## Application State

Unchanged. Production still spawns `deepagents-acp` with no extra argv.
Fixtures keep `pass_model_flag: false` and `model-flag-unmapped`. Guide still
lists `--model` as unsupported. AmbientHost, host-owned keys, permission
cancel, and context-losing restoration remain unchanged.

## Validation

Card 164 gates (docs / focused / diff; no production code):

- `effigy validate:focused swallowtail-adapter-deepagents`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

Code-only gates (`cargo fmt`, `package:verify-affected`, `check:examples`,
`qa:routes`, `package:api`, `doctor`) did not apply: no production code
executed. Inherited doctor baseline remains 378 findings (332 warnings / 46
errors) plus one generated-in-src warning.

## PR

- URL: pending
- base: current pushed `main`
- head: `t3code/review-deepagents-acp-model-selection`
- worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-7a377ef9`
- merge: do not merge; operator authorizes separately

## Shared Closeout

- `docs/research/README.md`: 206 reserved → promoted evidence stop; empty set
- `docs/logs/README.md`: this closeout reserved → complete
- `docs/roadmaps/README.md` Next Task: reassess remaining per-route inventory;
  keep g04 open
- `docs/roadmaps/g04/README.md` and generation index: g04.059 ready → stopped
- `docs/roadmaps/g04/batch-cards/README.md`: card 164 complete; cards 165-166
  blocked
- architecture/contracts/matrix/guide: no claim edit; `--model` remains not
  passed
- `docs/triage/2026-08-21-advanced-route-features.md` Deep Agents model row:
  record Research 206 empty stop
- g04 remains open; no rollover

## Next

Reassess the remaining per-route feature inventory before compiling the next
meaningful route-local lane. g04 stays open until explicit operator direction.
