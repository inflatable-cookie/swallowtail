# 2026-08-24 g04.058 Antigravity Headless Agent Profile Closeout

Status: complete
Owner: Tom
Milestone: g04.058
Cards: 161 complete; 162-163 blocked
Correction: 2026-08-24 review alignment — empty set on authorized evidence;
live `--print` probes are authority-boundary incidents only

## Result

Research 205 is an honest empty deliver-now set. Exact qualified help and
current official docs advertise `--agent`, `agy agents`, and selected
`init.agent`, but authorized evidence does not freeze a portable profile-id
domain, selected `init.agent` confirmation, fail-closed invalid handling on
`1.1.9..=1.1.17`, or authority-safe composition with custom profiles. Cards
162-163 stay blocked. No production code. No public API change. g04 stays
open.

## Evidence Table

| Version | Operation | Profile id | Listed | Dispatched | Confirmed | Fail-closed invalid | Authority-safe | Deliver-now |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `1.1.9..=1.1.17` | structured run | any caller id | host-local only | argv possible | unproved | unproved | no | no |
| `1.1.9..=1.1.17` | exact-id continuation | any caller id | host-local only | unproved | unproved | unproved | no | no |
| live `1.1.19` | any | any | empty later probes | not qualified | not qualified | incident-only | not qualified | no |

Empty-set basis (authorized):

1. host-local unstable `agy agents` listing
2. official custom-agent tool/instruction authority risk
3. missing selected `init.agent` fixture
4. unproved fail-closed invalid `--agent` semantics on the qualified range

Official headless docs promise no silent fallback for unknown `--model` only.
They do not make that promise for `--agent`. Existing decoder fixtures omit
`init.agent`. Production argv still omits `--agent`.

Two unauthorized `--print` probes (nonexistent id and whitespace-only
`--agent`) returned JSON `status: SUCCESS` with provider usage. Host PATH
drifted `1.1.9` → `1.1.19` during the session, so those outcomes are
authority-boundary / `UnverifiedNewer` incidents only. They are not projected
onto exact qualified `1.1.9..=1.1.17`.

Omission retains current wire. `UnverifiedNewer` has no private mapping to
inherit. No behavior, driver, claim, matrix, guide, or configured-instance
revision.

## Application State

Unchanged. Structured runs and exact-id continuation still select model,
optional effort/schema, read-plan mode, optional sandbox, and conversation id
only. Init still requires exact model and `permission_mode=request-review` and
does not inspect `init.agent`.

## Validation

Card 161 gates passed on the evidence head, then re-run after review
corrections:

- `effigy validate:focused swallowtail-adapter-antigravity` — 33 tests passed
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

No production code. Doctor was not re-run; inherited baseline remains 378
findings (332 warnings / 46 errors) plus one generated-in-src warning.

## PR

- URL: https://github.com/inflatable-cookie/swallowtail/pull/57
- base: `main`
- head: `t3code/antigravity-agent-profile-selection`
- prior evidence commit: `8a9e1717`
- review-correction content: `e8b58088`
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
  row: record Research 205 empty stop without projecting live incidents onto
  the qualified range
- g04 remains open; no rollover

## Next

g04.058 stops after card 161. Reassess the remaining per-route feature
inventory before compiling the next meaningful route-local lane. g04 stays open
until explicit operator direction.
