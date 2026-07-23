# 2026-07-23 Installed Executable Observation Records

## Changed

- Added Contract 032 for one explicit host-approved installed executable.
- Added safe exact version classification against one maintained claim.
- Kept the executable target out of the observation and stable diagnostics.
- Added a bounded request carrying request, scope, host, target, axis,
  monotonic deadline, and shared discovery cancellation.
- Added an additive target-aware discovery method. Existing general discovery
  behavior is unchanged.
- Added machine-distinct absent, incompatible, malformed, timed-out,
  cancelled, failed, and cleanup-failed outcomes.
- Added local and remote-authoritative testkit assertions.
- Proved the local process host starts only the explicit approved target and
  joins it before completion.

## Boundaries

- no executable search, installation, update, downgrade, or fallback
- no Codex parser or compatibility claim
- no credential, sign-in, model, provider, or consumer request
- no configured-instance creation or execution authority
- no path, raw stdout, stderr, environment, token, or payload in safe results

## Validation

- 165 focused core, runtime, testkit, and local-host tests pass
- focused warnings-denied clippy passes
- `git diff --check` passes

## Next

Card 112 freezes the selected Codex exec and app-server releases, rejection
neighbors, generated schemas, and experimental capability gates. Production
descriptors remain unqualified until card 113.
