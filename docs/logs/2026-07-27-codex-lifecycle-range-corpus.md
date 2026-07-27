# Codex Lifecycle Range Corpus

Date: 2026-07-27

## Change

Card 049 qualifies Codex app-server lifecycle behavior across the unchanged
`0.80.0..=0.145.0` executable range.

Research 037 records the tagged-source review. The adapter now publishes a
separate lifecycle compatibility claim beside its existing session claim.
The claim has five behavior revisions:

- archive response at the `0.80.0` baseline
- restore response from `0.92.0`
- archive and restore notifications from `0.104.0`
- best-effort spawned-descendant archive from `0.123.0`
- strict spawned-descendant hard delete from `0.140.0`

The deterministic lifecycle fixture covers 20 release checkpoints. Each
checkpoint records its exact source commit, npm publication date, schema
authority, and four aggregate schema hashes. It retains existing gaps,
prerelease rejection, and permitted visible unverified-newer execution.

## Corrected Truth

Archive guarantees only the target. Codex may attempt descendants from
`0.123.0`, but tagged tests allow the root to succeed while a descendant
remains unarchived.

Delete qualifies provider-declared hard deletion of the target and spawned
descendants from `0.140.0`.

The upstream missing-rollout wording is narrower than Research 036 first
stated. Codex tolerates a missing rollout only after the target is otherwise
known through live state, metadata, or descendants. An unknown target and a
repeated deletion after all target state is gone both fail. The production
mapping cannot report a general already-absent success.

## Boundary

- no archive, restore, or delete request is sent
- no provider-session management binding is returned yet
- no active runtime handle is discovered or closed
- no current-main behavior is projected into a tagged segment
- no supported session version is removed
- no shared contract changes are required

## Validation

- full Codex adapter suite: 99 tests pass
- lifecycle fixture: five focused tests pass
- `effigy check:rust`: passed
- `effigy format:check`: passed
- `effigy qa:docs`: passed
- `effigy qa:northstar`: passed
- `git diff --check`: passed
- final doctor scan: unchanged at 25 pre-existing findings
  (17 warning, 8 error)

## Next

Card 050 is ready. Map the qualified methods through the shared management
role and prepared facade while preserving inactive binding authority,
target-only archive truth, strict descendant delete truth, and exact
missing-target failures.
