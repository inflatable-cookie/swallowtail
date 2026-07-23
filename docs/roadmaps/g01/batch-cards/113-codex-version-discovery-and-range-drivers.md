# 113 Codex Version Discovery And Range Drivers

Status: planned
Owner: Tom
Updated: 2026-07-23
Milestone: `../039-installed-harness-compatibility-range-audit.md`

## Objective

Bind both Codex production drivers to exact observed executable versions and
publish only the ranges proven by card 112.

## Scope

- target-aware `codex --version` discovery for both driver registrations
- separate `codex.cli` compatibility claims for exec and app-server
- exec candidate range `0.122.0..=0.145.0`
- app-server candidate segments `0.110.0..=0.130.0` and
  `0.131.0..=0.145.0`
- behavior selection from the immutable exact version binding
- experimental app-server opt-in and field emission matched to the selected
  behavior revision
- omit default-false model fallback fields where explicit experimental use is
  unnecessary
- preserve `codex-exec-jsonl` and `codex-app-server-v2` as separate facades
- no executable search, install, update, auth flow, model fallback, consumer
  edit, or compatibility alias

## Acceptance Criteria

- [ ] both descriptors reject missing, below-floor, prerelease, and unknown
      newer executable versions before provider work
- [ ] discovery and preflight bind the same exact version
- [ ] exec and app-server claims remain independent
- [ ] app-server private dispatch follows the `0.131.0` milestone
- [ ] stable sessions send no unrequested experimental field
- [ ] experimental features always negotiate explicit capability
- [ ] public diagnostics expose no path, raw version output, config, or payload

## Validation

- focused Codex discovery, descriptor, driver, and rejection tests
- focused warnings-denied clippy
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

Yes. Continue to card 114 when both production drivers pass focused validation.
