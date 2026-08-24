# 2026-08-24 g04.058 Antigravity Headless Agent Profile Closeout

Status: complete
Owner: Tom
Milestone: g04.058
Cards: 161 complete; 162-163 blocked

## Result

Research 205 is an honest empty deliver-now set. Exact qualified help and
current official docs advertise `--agent`, `agy agents`, and selected
`init.agent`, but invalid `--agent` selection fail-opens to a successful
provider run on the live host, listing ids are host-local/unstable, and custom
profiles may change instructions and tools. Cards 162-163 stay blocked. No
production code. No public API change. g04 stays open.

## Evidence Table

| Version | Operation | Profile id | Listed | Dispatched | Confirmed | No silent fallback | Authority-safe | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `1.1.9..=1.1.17` | structured run | any caller id | host-local only | argv possible | unproved | no | no | no |
| `1.1.9..=1.1.17` | exact-id continuation | any caller id | host-local only | unproved | unproved | no | no | no |
| live `1.1.19` | any | any | empty later probes | not qualified | not qualified | observed fail-open | not qualified | no |

Official headless docs promise no silent fallback for unknown `--model` only.
They do not make that promise for `--agent`. Live probe
`--agent swallowtail-nonexistent-agent-zzzz` returned JSON `status: SUCCESS`
with nonzero usage. Existing decoder fixtures omit `init.agent`. Production
argv still omits `--agent`.

The empty set is fail-open invalid selection plus host-local identity and
authority composition risk, not missing documentation of the flag.

Omission retains current wire. `UnverifiedNewer` has no private mapping to
inherit. No behavior, driver, claim, matrix, guide, or configured-instance
revision.

## Application State

Unchanged. Structured runs and exact-id continuation still select model,
optional effort/schema, read-plan mode, optional sandbox, and conversation id
only. Init still requires exact model and `permission_mode=request-review` and
does not inspect `init.agent`.

## Validation

Card 161 gates passed on this branch:

- `effigy validate:focused swallowtail-adapter-antigravity` — 33 tests passed
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

No production code. Default validation used no install, login, account
mutation, or further provider prompts after the recorded invalid-selection
observation. Doctor was not re-run; inherited baseline remains 378 findings
(332 warnings / 46 errors) plus one generated-in-src warning.

## PR

- URL: pending
- base: `main`
- head: `t3code/antigravity-agent-profile-selection`
- worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-9c9cb362`
- approved head: pending
- merge: not authorized

## Shared Closeout

Pending review and merge:

- `docs/research/README.md`: 205 reserved → promoted evidence stop; empty set
- `docs/logs/README.md`: this closeout reserved → complete
- `docs/roadmaps/README.md` Next Task: after merge, reassess remaining
  per-route inventory; keep g04 open
- `docs/roadmaps/g04/README.md` and generation index: g04.058 planned → stopped
- `docs/roadmaps/g04/batch-cards/README.md`: card 161 complete; cards 162-163
  blocked
- architecture/contracts/matrix/guide: no claim edit; `--agent` remains not
  passed
- `docs/triage/2026-08-21-advanced-route-features.md` Antigravity agent-profile
  row: record fail-open invalid selection and empty Research 205 stop
- g04 remains open; no rollover

## Next

g04.058 stops after card 161. Reassess the remaining per-route feature
inventory before compiling the next meaningful route-local lane. g04 stays open
until explicit operator direction.
