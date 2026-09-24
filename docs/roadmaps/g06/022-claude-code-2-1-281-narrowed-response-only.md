# g06.022 Claude Code 2.1.281 Narrowed Response-Only Qualification

Owner: Tom
Created: 2026-09-24
Depends on: Contract 029 (No Terminal Stop); Contract 039 built-in provider hooks ruling; Research 338, 341; completed g06.018
Vision tags: route currentness, Claude Code, safe mode, adaptation

## Outcome

Raise both Claude Code stream-JSON axes through the official stable current at
run time (`2.1.281` at planning): headless as an ordinary extension,
response-only under the narrowed built-in-hook guarantee Tom accepted on
2026-09-24.

## Why It Matters

g06.018 ended on branch 3: nine `@builtin` plugins load under `--safe-mode`
from `2.1.280`, `agents-md` injects project instructions by default at
`2.1.281`, and no writable setting disables the set. Tom ruled that
response-only qualifies anyway with the provider's built-in behaviour
disclosed. Claude Code is two releases behind until this lands.

## Ready-State Rubric

- [x] Research 341 freezes identity, the hook ledger, and the parser findings
      through `2.1.281`.
- [x] Contract 039 records the narrowed guarantee.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Headless: extend through current official on its own evidence. It does not
  pass `--safe-mode` and already admits ambient project instructions.
- Response-only: add an adapter-private milestone from `2.1.280` carrying the
  narrowed guarantee. `2.1.279` becomes an explicit unpublished exclusion.
- Instruction files resolve from the launch directory. With a consumer `Read`
  project location, `AGENTS.md` joins the existing `CLAUDE.md` discovery.
  Without one, prove the adapter-chosen launch directory supplies no
  instruction file, or make it an adapter-owned empty directory.
- Pin a switch only when the frozen artifact proves it: an `instructionFiles`
  value that stops `agents-md` injection, or a supported telemetry opt-out
  (setting or environment variable) on the selected surface. Record each
  candidate tested and its result either way.
- Disclose the kept built-ins in the Claude prepared guide: what each can do
  and which Swallowtail argument blocks which effect.
- Watcher exact `2.1.251`, every feature-specific exact set, and the Claude
  Agent ACP and SDK families stay untouched.

## Dispatch manifest

- **State:** ready; one family adaptation lane.
- **Completion:** both axes qualified through the run's official stable;
  independent exact-head review accepts the head.
- **Owned mutable paths:** Claude Code family code under
  `crates/swallowtail-adapter-claude-agent/src/claude_code_*`; its fixtures and
  identity tests under `crates/swallowtail-adapter-claude-agent/tests/`; the
  Claude prepared guide and the route and feature matrix rows naming this
  family; `CHANGELOG.md` `[Unreleased]`; one new research record or a Research
  341 addendum and its index line; the family claim log and its index line;
  the working public-API baseline if the API changes; `PAPERCUTS.md` append
  only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; contracts; watcher code and exact
  `2.1.251`; Claude Agent ACP and SDK surfaces; other families; Next Task and
  the generation runway; release, tag, publication.
- **Worker evidence:** the authenticated run result and the research record.
- **Escalation:** operator via Chatterbox if the evidence contradicts the
  Contract 039 ruling; Queue coordinator for mechanical blockers.

## Work

1. Re-probe official latest; extend identity past `2.1.281` if it moved.
2. Test each switch candidate against the frozen parsers and record results.
3. Settle the no-project-location launch directory.
4. Compile the headless extension and the response-only milestone, with
   argument tests per segment.
5. Update the guide disclosure, matrices, and claim log.
6. Run the named validation.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Both ceilings move | Either axis ends at `2.1.278` | claims name the run's official stable |
| Headless is not held by response-only | Headless waits on the response-only milestone | independent headless extension |
| Pinned switches are real | A setting the binary ignores is passed | parser citation from the frozen artifact |
| No hidden instruction source | Without a project location the child launches where `AGENTS.md` is reachable | launch-directory test |
| Older segments unchanged | New arguments leak below `2.1.280` | argument tests per segment |
| Disclosure matches the ledger | The guide omits a kept built-in or overstates a guarantee | guide lists all nine with their blocked effects |

## Stop conditions

Stop and ask if a kept built-in can invoke a tool, command or MCP call despite
the route's arguments, if identity disagrees across channels, or if a hop
needs a public lifecycle change.

## Evidence

Research 338 and 341 and the new record. Focused and affected-package
validation for `swallowtail-adapter-claude-agent`, route and docs QA.

## Next Task

Chatterbox reconciles the claims after closeout and decides whether a live
response-only gate is worth requesting.
