# 168 Kimi Code ACP Extended Effort Binding

Status: ready
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Milestone: [g04.060 Kimi Code ACP Catalogue-Declared Effort Levels](../060-kimi-code-acp-catalogue-declared-effort-levels.md)
Depends on: card 167; non-empty Research 207 deliver-now table

## Goal

Admit only Research 207's exact `xhigh` and `max` rows through the existing
new-session negotiated reasoning path, with exact snapshot membership,
effective confirmation, and version behavior.

## Work

1. Update the Kimi ACP compatibility behavior only as Research 207 requires.
   Split the maintained segment/revision at the exact source milestone if older
   qualified versions cannot advertise extended effort rows.
2. Extend route-local reasoning option validation only for admitted `xhigh`
   and `max`. Do not accept an arbitrary provider string, display label, alias,
   or inferred model capability.
3. Require the current session-open `thinking` select option to advertise the
   exact requested value before sending `session/set_config_option`.
4. Keep exactly one set request and require the response to contain one valid
   `thinking` option whose effective `currentValue` equals the request.
5. Apply Research 207's rule for foreign advertised rows without hiding a
   malformed or ambiguous option. Preserve boolean, always-thinking, and
   legacy `off|on|low|medium|high` behavior.
6. Preserve caller omission, prepared input, plan/request agreement, model
   negotiation, access, resource, provider-state, cancellation, deadline,
   terminal, and joined cleanup behavior.
7. Keep reasoning redeclaration rejected for load and resume before host
   effects. Import and attachment recovery gain no selection path.
8. Add focused exact-version fixtures and API/example/guidance changes only
   where the admitted surface warrants them.

## Acceptance Criteria

- [ ] only Research 207 exact version/value rows prepare
- [ ] `xhigh` and `max` require exact current-snapshot membership
- [ ] one exact selection request precedes readiness and the response confirms
      the same effective value
- [ ] unsupported, absent, malformed, ambiguous, foreign, substituted, and
      drifted values fail closed as Research 207 requires
- [ ] legacy values, boolean options, omission, and exact current wire remain
      unchanged
- [ ] compatibility behavior and configured-instance identity match the proved
      source milestone
- [ ] load/resume/import/recovery gain no reasoning mutation
- [ ] access, resource, provider state, cancellation, deadline, terminal, and
      cleanup truth stay intact
- [ ] no breaking public API or shared contract/runtime change
- [ ] `cargo fmt -p swallowtail-adapter-kimi` passes
- [ ] `effigy validate:focused swallowtail-adapter-kimi` passes
- [ ] `effigy package:verify-affected swallowtail-adapter-kimi` passes
- [ ] `git diff --check` passes

## Stop Conditions

- Research 207 is empty or ambiguous
- implementation needs arbitrary effort strings, model-name inference,
  unconfirmed dispatch, shared contract/runtime change, or a breaking public API
- exact compatibility behavior cannot be represented without widening
  unproved versions

## Out Of Scope

- other Kimi controls/routes, live OAuth/provider work, currentness, release,
  merge, generation rollover, or g04 closure
