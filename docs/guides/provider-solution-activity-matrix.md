# Provider Solution Activity Matrix

The
[machine-readable matrix](provider-solution-activity-matrix.csv)
indexes observable activity by provider solution, route, and operation shape.
It is a compiled consumer reference. The exact
`PreparedOperationEvidence::observable_activity()` returned for a prepared
operation remains the runtime source of truth.

The inventory contains 55 route-operation rows:

- 32 ordinary structured-run or interactive-session profiles with
  `available` activity
- 23 catalogue, inventory, provider-session-management, realtime-media, or
  serving operations where ordinary agent activity is `not-applicable`
- all 26 production route identities
- four auxiliary hosted catalogue identities

There is no unexplained whole-operation `unavailable` result. An
`unavailable` feature cell means the operation is usable but its exact
prepared activity profile does not promise that activity.

## Row Identity

`provider` and `solution` are display grouping fields. `route_id` and
`operation_shape` select the technical profile. `route_scope` distinguishes
production routes from auxiliary catalogue branches.

`assistant_intermediate` and `assistant_final` describe semantic channels, not
streaming deltas. A direct-inference final answer may have many updates while
`assistant_intermediate` remains `unavailable`.

## Activity And Content Values

`activity_profile` uses:

- `available` — inspectable semantic activity exists for this operation
- `not-applicable` — the operation is not an ordinary agent run or turn

Content and feature columns use:

- `provider-display` — bounded provider-intended readable display content
- `adapter-summary` — a bounded summary derived from public lifecycle or
  status fields, never hidden reasoning
- `identity-lifecycle` — identity, phase, status, label, or correlation only;
  no portable content body
- `profile-dependent` — exact admitted version or selected sub-profile decides;
  inspect prepared evidence
- `unavailable` — the available operation profile makes no portable promise
  for this feature
- `not-applicable` — the whole operation is outside ordinary agent activity

`tool_display_input` and `tool_display_output` cover only content carried on
activity events. Callback arguments, direct-tool bodies, approval questions,
and results remain on their existing typed exchanges. An
`identity-lifecycle` tool cell must not be upgraded by parsing a provider
label or native envelope.

## Lifecycle And Correlation Values

Tool lifecycle and `lifecycle_fidelity` use:

- `complete` — start, updates where emitted, and completion
- `update-completion` — updates and completion without a portable start
- `completion-only` — one terminal observation
- `mixed-by-kind` — activity kinds within the profile have different fidelity
- `profile-dependent` — the admitted version or selected sub-profile decides
- `unavailable` or `not-applicable` — as defined above

Tool correlation uses:

- `provider-item` — provider-owned item or tool identity
- `provider-request` — provider request identity
- `consumer-callback` — the callback exchange identity
- `direct-tool-call` — the direct inference tool-call exchange identity
- `operation-local` — no stronger portable identity than the current run or
  turn
- `profile-dependent`, `unavailable`, or `not-applicable`

The matrix records `consumer-callback` as an allowed vocabulary even though no
current route has it as its sole route-wide correlation posture. Codex
app-server reports `profile-dependent` because callback-correlated consumer
tools are version-qualified beside other correlation forms.

## Disclosure And Unknown Events

`disclosure_fidelity` uses the content values plus `mixed-by-kind`.
`unknown_event_posture` uses:

- `preserve-namespaced` — safe, correlated unknown semantic activity may be
  preserved under a bounded provider namespace
- `fail-closed` — unqualified native events do not become activity
- `profile-dependent` — inspect the exact prepared profile
- `not-applicable`

Neither posture permits exposing raw provider envelopes.

## Consumer Projection

Inspect the prepared profile before starting effects. Add exact
`ObservableActivity` requirements when a feature is mandatory; otherwise
render only the activity kinds the inspected profile promises.

Treat all readable activity content as sensitive application data. Swallowtail
bounds and classifies it but does not decide retention, encryption, export,
redaction, or audience. Consumers should:

1. preserve runtime event order
2. project activity by operation identity and `activity_id`
3. apply delta or replacement semantics from the typed content update
4. keep callback and direct-tool bodies on their typed exchanges
5. keep final operation output distinct from final-assistant activity
6. render identity-only and namespaced-unknown activity conservatively

Transcript persistence, retries in the UI, collapsed tool groups, display
labels, and thread ownership remain consumer concerns. A consumer may discard
activity after rendering or persist a product-specific projection. Swallowtail
does not create a transcript store or prescribe a chat schema.

For `profile-dependent` cells, never infer the richer value from this CSV.
Use the profile returned by the exact prepared operation. Unverified-newer
provider versions do not widen the last qualified profile.

## Evidence

Every row links:

- `prepared_entry` — the public prepared source path
- `conformance_test` — the prepared-facade proof
- `evidence_ref` — the exact machine-checked route inventory

These links make positive and negative cells auditable without downstream
provider-native parsing. Route QA checks row order, value vocabulary, counts,
operation applicability, file references, and production plus auxiliary route
identity coverage.
