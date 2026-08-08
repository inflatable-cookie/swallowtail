# 160 Provider-Wide Scaffolding Migration

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../052-shared-adapter-scaffolding.md`
Depends on: card 159

## Goal

Close the scaffolding milestone with a provider-wide migration pass and
measured acceptance.

## Scope

1. Re-scan the adapter duplication families after cards 156-159 and migrate
   any remaining near-identical modules that fall under the shared helpers.
2. Run the full provider-wide evidence round: focused, affected-package,
   extracted-package, examples, public API baseline, and route/feature
   matrices.
3. Record the measured before/after duplication and the remaining
   intentionally adapter-local differences in the milestone closeout.

## Out Of Scope

- new contracts or provider-neutral vocabulary changes
- public API or behavior changes

## Acceptance

- [x] every near-identical module under the six shared families is migrated
      or explicitly recorded as adapter-local with a reason
- [x] the full deterministic round passes with an unchanged public API
      baseline
- [x] the closeout records measured duplication before and after

## Closeout

### Migration result

Duplicated installed-executable probe machinery, measured by files containing
the shared `next_output`/`probe_process` core at HEAD:

- before: 12 discovery files across 11 adapters
- after: 4 files, all intentional (recorded below)
- migrated this pass: claude-agent (acp + claude-code discovery),
  gemini (both routes via `ProbeRoute` claim/binding closures)
- earlier cards 156-159 migrated: pi, oh-my-pi, qwen, muse, kimi
- shared scaffold: `swallowtail-runtime/src/installed_discovery.rs` (465
  lines, codes macro + exact-version parse + bounded probe)

### Remaining adapter-local, with reasons

- antigravity, cursor: staged outcome codes (per-stage `spawn_failed` /
  staged-outcome diagnostics) beyond the scaffold's failure classification
- codex: distinguishes `ProcessExit`-class stages (nonzero probe exit with
  stderr sanitization) the scaffold does not capture
- grok: probe needs stderr capture beyond the shared stdout cap

Each was tested; a migration attempt that would change classified failure
stages was reverted per the card stop condition.

### Validation

- `cargo test -p` focused round: 42 suites green across runtime and all
  migrated adapters (pi, oh-my-pi, qwen, muse, kimi, claude-agent, gemini)
- MSRV clippy `-D warnings` workspace: clean; MSRV + current-stable workspace
  tests: 138 suites green
- `effigy package:api`: 28 packages at the v0.3.0 candidate baseline,
  unchanged
- `effigy package:check`: metadata and API gates pass; the release-floor
  gate's lockfile precondition is pending commit of the card-155 runtime
  dependency sync (`futures-channel`, `semver` in Cargo.toml/Cargo.lock)
- note: the scaffold's two new runtime dependencies were never synced into
  Cargo.lock during card 155; fixed here (Cargo.lock +1 package-block, no
  third-party bumps)

## Stop Conditions

- stop if a migration changes public behavior or classification

## Auto-Continuation

Yes, to card 161 after acceptance.

## Validation

- `effigy qa`, `effigy package:check`, `effigy package:api`
