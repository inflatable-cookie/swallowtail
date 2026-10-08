# 015 Bounded Finite Exact Opaque Compatibility Claims

Status: draft; proposed core representation. Current implemented behavior
stays the one-point Opaque restriction until a separately reviewed
implementation of this spec lands.
Owner: Tom
Updated: 2026-10-08
Authority: Contract 029 Finite Exact Opaque-Set Design Authority; Tom board
decision `81c79a19-0035-4b5e-b241-a400380c7f4a`

## Purpose

Specify a bounded finite set of exact opaque runtime points on one
Contract 029 axis, so a later core implementation can retain existing
one-point callers while admitting independently evidenced extra exact
members. This spec does not change runtime code, production claims, or
route qualification.

## Proposed Versus Current

| Surface | Current implemented behavior | This spec proposes |
| --- | --- | --- |
| Opaque membership | Exactly one `InterfaceVersionSegment` with `minimum == maximum` | One to 32 exact segments on the same axis |
| Opaque ordering | `compare_versions` uses `InterfaceVersion` text `Ord`; construction still demands a single equal pair | Text `Ord` is storage canonicalization only; membership is equality |
| `QualifiedOnly` | Opaque with `AllowUnverified` is refused | Unchanged |
| Classification | Exact equality against the one segment | Exact equality against the membership set; no interior or forward inference |
| Public constructors | `InterfaceCompatibilityClaim::new` plus `InterfaceVersionSegment::exact` | Same constructors; `new` accepts 1..=32 exact Opaque members |
| Production claims | All Opaque claims remain one point | Unchanged by this spec and by the later core implementation |

The one-point restriction in
[`crates/swallowtail-core/src/interface_version/claim.rs`](../../../crates/swallowtail-core/src/interface_version/claim.rs)
remains in force until that implementation is independently reviewed.

## Current Call Sites And Invariants

### Core claim model

`InterfaceCompatibilityClaim` in `claim.rs` stores `id`, `axis`, `scheme`,
`newer_version_posture`, `segments: Vec<InterfaceVersionSegment>`, and
`exclusions: BTreeSet<InterfaceVersion>`. There is no serde. Equality is
structural, so `Vec` order is part of `Eq`.

Construction (`InterfaceCompatibilityClaim::new`) validates then returns.
Opaque-specific rules today:

- `segments` must be non-empty
- `segments.len() == 1`
- `minimum` and `maximum` of that segment must compare `Equal`
- `newer_version_posture` must be `QualifiedOnly`
- exclusions must pass `validate_version` (Opaque accepts any non-empty text)

`InterfaceVersionSegment::exact` clones one `InterfaceVersion` into both
bounds. `InterfaceVersionSegment::new` is also public and is how a two-bound
Opaque window is refused.

`baseline()` is `segments[0].minimum()`. `latest_qualified()` is
`segments.last().maximum()`. For every valid Opaque claim today those two
values are the same point.

`milestones()` yields `segments.iter()` in stored order. For Opaque that is
one exact segment.

`classify` returns `None` for exclusions, invalid scheme text, and
non-members. For Opaque, `segment_contains` uses text `Ord` against the one
equal pair, which is equality. Semantic prerelease exact-segment matching
does not apply.

`assess` maps a classified member to `Qualified`. Opaque never produces
`UnverifiedNewer`: `assess` returns `Incompatible` when
`scheme == Opaque`. `supports` is `classify.is_some()`. `permits` is
`assess.is_permitted()`.

`compare_versions(Opaque, left, right)` is `left.cmp(right)` on the version
text. That path is used for bound checks and the ordered `windows(2)`
non-overlap rule. The overlap rule never runs on a valid Opaque claim
because `len() != 1` is already refused.

Diagnostics use code `swallowtail.interface_compatibility_claim_rejected`
(`error.rs`). Messages are fixed strings. Tests match the code, not the
message, except by going through `unwrap_err()`.

### Registration, instance, preflight, prepared facade

`DriverDescriptor` (`registration.rs`) holds at most one claim per
`InterfaceVersionAxis` in a `BTreeMap`. `with_interface_compatibility`
replaces by axis. `classify_interface_version` / `assess_interface_version`
/ `permits_interface_version` delegate to the claim for that binding's axis,
or `Incompatible` when the axis is absent.

`ConfiguredInstance` stores exact `InterfaceVersionBinding` values
(`axis` + `version`), not the claim. Preflight
(`preflight/validation.rs` `validate_interface_versions`) requires every
required binding to be present on the instance and
`driver.permits_interface_version(required)`. Failure dimension is
`PreflightDimension::InterfaceVersion`. Diagnostic code is
`swallowtail.preflight_rejected`. Messages name the axis and do not include
paths, tokens, or provider payloads. This runs inside `preflight()` before
any endpoint, credential, or process effect.

`PreflightPlan` clones the `DriverDescriptor` and instance into
`PlanBinding`. `classify_interface_version` / `assess_interface_version` on
the plan reuse the frozen driver claim. `validate_current` re-runs
validation and compares `PlanBinding` by `Eq`. A claim-membership edit
changes the descriptor and therefore stale-plans with
`swallowtail.preflight_plan_stale`.

`PreparedInterfaceCompatibility` (`swallowtail-runtime`
`prepared_operation.rs`) stores one binding plus the plan's assessment.
`PreparedOperationEvidence::prepare` builds one entry per instance binding.
Activity-profile basis uses `assessment.behavior_revision()` when present.

### Discovery and connection-lifecycle reuse

`InstalledExecutableObservation::classify` requires the observation axis to
equal the claim axis (`swallowtail.installed_executable.axis_mismatch`)
then stores `claim.assess(version)`. `InstanceUpdateObservation::from_claim`
reuses that classification and does not install or authenticate.

### Public API (v0.5.1 baseline)

Released `swallowtail-core` exports `InterfaceCompatibilityClaim::{new,
baseline, latest_qualified, milestones, exclusions, classify, supports,
assess, permits, scheme, newer_version_posture, id, axis}` and
`InterfaceVersionSegment::{new, exact, minimum, maximum, behavior_revision,
support_status}`. Adapter crates construct claims with `new` + `exact` and
read `scheme`, `baseline`, `latest_qualified`, `milestones`, `assess`, and
`permits`.

### Production Opaque claims

Every production Opaque claim is one exact maintained member,
`QualifiedOnly`, empty exclusions. Axes stay separate. Hosted facades
(OpenAI background, Gemini Live, Bedrock service APIs, Kimi Platform, xAI,
Alibaba, DeepSeek, Anthropic) keep superseded facade strings as historical
evidence, not as extra executable segments. Installed opaque payloads
(Muse signed payload, ZCode runtime, Pi sidecar wire and source-tag) stay
one point. llama.cpp attached `b9910-f5525f7e7` on
`llama.cpp.attached-runtime` and owned `b10069-178a6c449` on
`llama.cpp.owned-runtime` are independent one-point claims
(`selection/interface.rs`). Protocol readiness still matches one
build/commit pair (`protocol.rs` `ATTACHED_VERSION` / `OWNED_VERSION`).
That private mapping is adapter-local and is not this spec's representation
change.

## Recommended Representation

Keep `InterfaceCompatibilityClaim` and `InterfaceVersionSegment`. Opaque
membership is the claim's segment list, with every segment exact
(`minimum == maximum`). Do not add a second claim type, a second scheme, or
a second constructor that callers must choose.

After successful Opaque validation, store members in canonical text order
of `InterfaceVersion` (`Ord` on the validated string). Constructor input
order is not preserved. That sort is storage, `Eq`, `milestones()`
iteration, and diagnostic listing. It is not compatibility order, recency,
or a supported interval.

Classification is a lookup of the observed version string among member
version strings, then exclusion check. Do not call `segment_contains` with
text `Ord` for Opaque once more than one member exists: a string that sorts
between two members is not a member.

`DriverDescriptor` remains one claim per axis. Multiple Opaque points on
one axis live in that one claim. Different axes stay different claims.

## Bounds

Opaque claims are listed points, not intervals. The bound is the finite
substitute for a range.

| Bound | Value | Why |
| --- | --- | --- |
| Maximum exact members | 32 | Research 407 lists 7 owned identities from `b10069` through `b11429` if every hop were later claimed. Research 369's largest published-hop counts on other families stay below this when expressed as exact pins. 32 is large enough for a maintained exact-pin window and small enough that construction, `Eq`, diagnostics, and review stay bounded. |
| Maximum exclusions | 32 | Same identity-object cost as members. Known-bad points may sit outside membership (Contract 029). |
| Maximum UTF-8 bytes per Opaque version text, at claim validation | 256 | Existing facade strings fit (Gemini Live current point is 107 bytes). Blank text is already refused by `InterfaceVersion::new`. This bound is enforced when the version is used as an Opaque member or Opaque exclusion, not by shrinking `InterfaceVersion::new` globally. |
| Maximum UTF-8 bytes per behavior-revision text on an Opaque member | 256 | Same diagnostic bound. Existing revision ids are short. |

Refuse construction when any bound is exceeded. Do not truncate.

A later family that needs more than 32 independently evidenced exact points
on one Opaque axis returns for a separate bound ruling. Do not raise the
bound inside the core implementation task.

## Validation Rules

On `InterfaceCompatibilityClaim::new` when `scheme == Opaque`:

1. Posture is `QualifiedOnly`. Otherwise refuse:
   `Opaque compatibility claims must remain qualified-only`.
2. `segments` is non-empty. Otherwise refuse the existing empty-window
   message.
3. `segments.len() <= 32`. Otherwise refuse:
   `Opaque compatibility claims permit at most 32 exact members`.
4. Each segment: both bounds pass `validate_version`; `minimum == maximum`
   by version text equality. A two-bound Opaque window is still invalid:
   `Compatibility segment boundaries are invalid`.
5. Each member version text is at most 256 UTF-8 bytes. Otherwise refuse:
   `Opaque version text exceeds 256 bytes`.
6. Each member behavior revision is at most 256 UTF-8 bytes. Otherwise
   refuse: `Opaque behavior revision text exceeds 256 bytes`.
7. Member version strings are unique. Duplicates refuse:
   `Opaque compatibility members must be unique`.
8. Two members with the same version and disagreeing behavior revision
   cannot occur once uniqueness holds. Do not add a second path that could.
9. Exclusions: at most 32; each at most 256 bytes; each passes
   `validate_version`. An exclusion equal to a member version refuses:
   `Opaque exclusions cannot name a claimed member`.
10. Support labels:
    - at least one member is `Maintained`
    - every `Maintained` member shares one behavior-revision identity
    - every `Deprecated` member uses a different behavior-revision identity
      from that Maintained revision

    Zero Maintained, split Maintained revisions, or a Deprecated member
    that shares the Maintained revision refuse with:
    `Opaque support status must follow the claim's maintained behavior revision`.

    A single-revision Opaque set is Maintained throughout. Mixed revisions
    mark the current mapping Maintained and older mappings Deprecated.
    "Current" is the unique Maintained behavior-revision identity. It is
    not inferred from version text order.

11. After those checks, sort members by version text and store that order.
    Skip the ordered `windows(2)` non-overlap rule for Opaque. That rule is
    interval logic.

Semantic, integer, and calendar validation is unchanged, including the
ordered non-overlap rule.

Malformed `InterfaceVersion` values (blank) still fail at
`InterfaceVersion::new` with `ValueRequired`, before claim validation.

## Classification And Preflight

For Opaque:

| Observed point | Result |
| --- | --- |
| Exact member, not excluded, Maintained | `Qualified` with that member's behavior revision and `Maintained` |
| Exact member, not excluded, Deprecated | `Qualified` with that member's behavior revision and `Deprecated`; `permits` is true |
| Exact exclusion | `Incompatible` |
| Unknown, interior lexicographic neighbor, prerelease-shaped text that is not a member, empty already impossible | `Incompatible` |
| Any non-member, including text that sorts between members | `Incompatible`; never `UnverifiedNewer` |

The matched behavior revision is the member's stored revision. Point
identity remains `InterfaceVersionBinding { axis, version }` for the
observed string. Do not rewrite the observed string to canonical case or
to a "latest" member.

Preflight keeps today's gate: the instance must bind the required exact
point, and `permits_interface_version` must be true, before host or
provider effects. Unknown, mismatch, and excluded points fail on
`PreflightDimension::InterfaceVersion`. Drift after preparation is
`validate_current` / `swallowtail.preflight_plan_stale`.

Prepared facade assessment follows the same `assess` result for the bound
point. A plan that still binds a superseded hosted facade point stays
incompatible when that point is not a member.

`InstalledExecutableObservation` and `InstanceUpdateObservation` keep
delegating to `assess`. They gain no install, upgrade, or fallback
behavior.

## Public API, Baseline, Latest-Qualified, Diagnostics

Preserve:

- `InterfaceCompatibilityClaim::new` and `InterfaceVersionSegment::exact`
  for one exact Opaque member. Singleton results stay identical: one
  Maintained member, `baseline() == latest_qualified()`, `milestones().len()
  == 1`, `QualifiedOnly`, `assess` of that point is `Qualified`, any other
  point is `Incompatible`.
- Method signatures in the v0.5.1 `swallowtail-core` baseline.
- Diagnostic code `swallowtail.interface_compatibility_claim_rejected`.
- `scheme() == Opaque` as the discriminator that ordering is unavailable.

For Opaque with two or more members:

- `milestones()` yields canonical-text order of exact members.
- `baseline()` is the canonical-text first member version.
- `latest_qualified()` is the canonical-text last member version.
- Those two values are a membership envelope for diagnostics and `Eq`
  stability. They are not a supported interval. A point that sorts between
  them is not qualified unless it is a member.
- Do not render Opaque claims as `baseline..=latest_qualified`. List
  members. `scheme() == Opaque` already tells callers there is no interval.

Add one patch-compatible method:

```text
fn has_version_interval(&self) -> bool
```

False for Opaque. True for Semantic, Integer, and CalendarDate. Consumers
that print a window consult this or `scheme()`.

Do not add `opaque_set(...)` as a required second constructor. Adapters
already call `new`. A private helper inside core tests is fine.

Round-trip: constructing with members in any order, then reading
`milestones()`, `baseline()`, `latest_qualified()`, and `exclusions()`,
yields the canonical membership and the exclusion `BTreeSet` order.
`Eq` ignores constructor order for Opaque. Changing membership, a
behavior revision, support status, exclusions, posture, id, axis, or
scheme is a different claim; frozen plans that cloned the old descriptor
are stale.

There is no durable on-disk claim format. Storage is the in-memory claim
inside `DriverDescriptor`, exact bindings on `ConfiguredInstance`, the
cloned `PlanBinding`, and `PreparedInterfaceCompatibility`. Debug output
of observations must still omit host paths and raw stdout.

## Contract 036 Classification

The later core implementation, if it follows this spec:

- keeps every released method signature
- keeps every currently valid singleton Opaque claim succeeding with the
  same `classify` / `assess` / `baseline` / `latest_qualified` / `permits`
  results
- expands `new` so some inputs that today return
  `InvalidInterfaceCompatibilityClaim` become `Ok`
- may add `has_version_interval`

That is a compatible public-API expansion plus additive diagnostics under
Contract 036 patch rules (`compatible public API and guaranteed-behavior
changes`, `additive public items`, `additive safe diagnostics`). It is not
a removal, a signature break, a capability shrink, or a lifecycle change.

The first production Opaque claim with `baseline() != latest_qualified()`
is a later family adaptation, not this core change. Callers that already
honor `scheme() == Opaque` as exact-only keep working. Callers that ignore
`scheme` and print `baseline..=latest_qualified` for Opaque would mis-render
a multi-member claim; that rendering is already wrong for one-point Opaque
windows framed as ranges. Release notes for the core implementation must
say: Opaque membership is a list; do not print it as an interval.

Treat the core implementation as a **patch** under Contract 036. Treat it
as a **minor** only if the implementation changes singleton results,
allows `AllowUnverified` on Opaque, infers interiors, removes or renames
`baseline` / `latest_qualified`, or changes existing consumer meaning of a
released route claim. Those deviations are out of this spec.

Route-family adaptation that adds members to llama.cpp or any other
production claim is a separate Contract 029 qualification. Newly qualified
provider-interface versions remain a Contract 036 patch when membership
widens without shrinking guaranteed behavior. Narrowing or a public
lifecycle change still returns under Contract 036 / 029.

## Alternatives

| Option | What it does | Why not recommended |
| --- | --- | --- |
| **A. Bounded exact-member set on the existing claim (recommended)** | `new` accepts 1..=32 exact Opaque segments; canonical text sort; equality classify; `QualifiedOnly` | Matches the approved extension, preserves callers, avoids a second type |
| **B. New `OpaqueSet` type** | Parallel claim type and constructors | Splits every `scheme` match and `DriverDescriptor` path; larger public API; easy to desync validation |
| **C. Reuse Semantic with `min == max` segments** | Several exact Semantic segments | Semantic admits `AllowUnverified` and interval `segment_contains` unless extra guards; weakens Opaque fail-closed |
| **D. Integer scheme on llama.cpp build numbers** | Parse `bNNNN` as ordered integers | Fabricates compatibility order; forbidden by the brief; lexicographic and integer order both disagree with source chronology once commit suffixes exist (`b9910-…` sorts after `b11429-…`) |
| **E. One claim per point, several claims per axis** | Multiple `DriverDescriptor` claims on one axis | `BTreeMap<InterfaceVersionAxis, Claim>` stores one claim per axis; replacing that is a different public authority change |

Do not rename llama.cpp runtime axes, fold attached into owned, or treat
`v0.6.0` as an Opaque runtime member.

## Design Tables

These tables exercise the proposed rules. They do not qualify `b11429`,
move production claims, or credit route adaptation. Version text for a
future `b11429` member follows the existing `bNNNN-<9 hex chars>` encoding
as `b11429-d81235049` from Research 407 commit
`d81235049384534c167caea52b85a694f6103d14`. That string is a classification
fixture, not a production identity.

### Attached current singleton

Axis `llama.cpp.attached-runtime`. Members:
`b9910-f5525f7e7` / `llama-cpp.attached-openai-chat-b9910` / Maintained.

| Observed | Assessment |
| --- | --- |
| `b9910-f5525f7e7` | Qualified, that behavior, Maintained |
| `b11429-d81235049` | Incompatible |
| `b10069-178a6c449` | Incompatible (owned identity, wrong axis even if the string were reused) |
| `v0.6.0` | Incompatible |
| `b9910` | Incompatible (not the stored identity) |
| empty / blank | `InterfaceVersion::new` fails |

`baseline()` and `latest_qualified()` are both `b9910-f5525f7e7`.

### Attached hypothetical two-member set

Same axis. Members: `b9910-f5525f7e7` Deprecated with the b9910 behavior
revision, and `b11429-d81235049` Maintained with a distinct b11429
behavior revision. This shape is only valid after independent attached
evidence. This spec does not supply that evidence.

| Observed | Assessment |
| --- | --- |
| `b9910-f5525f7e7` | Qualified, b9910 behavior, Deprecated; executable |
| `b11429-d81235049` | Qualified, b11429 behavior, Maintained |
| `b10566-bb4caa754` (Research 407 hop, not a member) | Incompatible; lexicographic interior is not membership |
| `v0.6.0` | Incompatible |

Canonical text order puts `b11429-d81235049` before `b9910-f5525f7e7`
(`'1' < '9'`). Envelope `baseline()` is therefore `b11429-d81235049` and
`latest_qualified()` is `b9910-f5525f7e7`. That envelope is not chronology
and not a range. Listing members is the diagnostic.

Same two version strings with one shared behavior revision: both
Maintained. Same two version strings both Maintained with disagreeing
revisions: construction refuse.

### Owned current singleton on its own axis

Axis `llama.cpp.owned-runtime`. Members:
`b10069-178a6c449` / `llama-cpp.owned-openai-chat-b10069` / Maintained.

| Observed | Assessment |
| --- | --- |
| `b10069-178a6c449` | Qualified, owned b10069 behavior, Maintained |
| `b11429-d81235049` | Incompatible until an owned-axis claim lists it |
| `b9910-f5525f7e7` | Incompatible (attached identity) |
| `v0.6.0` | Incompatible. Research 407 correlates source tag `v0.6.0` to `b11429`; the tag is not the owned runtime identity |

Attached and owned cannot share one claim. `DriverDescriptor` keys claims
by axis. Putting both points on one axis would collapse the route
separation.

### Missing, malformed, conflicting, excluded

| Input | Result |
| --- | --- |
| Opaque `new` with `minimum=b9910-f5525f7e7`, `maximum=b11429-d81235049` | Construction refuse, boundaries invalid |
| Two members both `b10069-178a6c449` | Construction refuse, uniqueness |
| Member `b10069-178a6c449` also listed in exclusions | Construction refuse |
| Exclusion `b10566-…` not a member | Construction ok; that point is `Incompatible` |
| 33 members | Construction refuse, oversized |
| Member version 257 bytes | Construction refuse |
| `AllowUnverified` | Construction refuse |
| Hosted singleton Gemini Live current facade | Unchanged: that point Qualified; superseded Live facade strings remain Incompatible because they are not members |

### Source tag versus runtime axis

`v0.6.0` may exist as a source or package identity on a different axis in
a later family task. It must not appear as an Opaque member on
`llama.cpp.attached-runtime` or `llama.cpp.owned-runtime`. Semantic scheme
on those runtime axes is alternative D and is refused.

## Core Implementation Plan

Separate later task. This PR does not implement it. No route-family
adaptation, no llama.cpp claim edit, no protocol `ObservedVersion` change,
no host mutation.

### Code

- `crates/swallowtail-core/src/interface_version/claim.rs` validation,
  canonical sort, Opaque classify/assess equality lookup
- `crates/swallowtail-core/src/interface_version.rs` additive
  `has_version_interval`
- `crates/swallowtail-core/src/interface_version/tests.rs` and existing
  singleton tests
- Preflight, installed-executable, and prepared-facade tests that bind
  fixture Opaque sets; no production adapter claim edits

### Named Effigy selectors (add in that implementation PR)

```text
check:opaque-set-claim
validate:opaque-set-claim
```

`check:opaque-set-claim` compiles `--locked --all-targets` for
`swallowtail-core` and `swallowtail-runtime`.

`validate:opaque-set-claim` runs nextest `--locked --profile ci` on
`swallowtail-core` `--lib` and `swallowtail-runtime` `--lib` with a filter
that covers the tests below. Do not run `validate:focused`, `test:rust`,
or `qa`.

### Tests the implementation must add or keep

Construction and bounds:

- singleton `new` + `exact` still succeeds with today's results
- two exact members succeed and canonicalize constructor order
- 32 members succeed; 33 refuse
- duplicate version refuse
- `min != max` refuse
- `AllowUnverified` refuse
- member-also-excluded refuse
- oversized version text refuse
- zero Maintained, split Maintained revisions, Deprecated sharing the
  Maintained revision refuse
- empty claim still refuse
- Semantic overlapping windows still refuse (non-regression)

Classification:

- member Maintained / member Deprecated / unknown / lexicographic neighbor
  / exclusion / `v0.6.0` fixture against an Opaque runtime-shaped claim
- matched behavior revision is the member's revision, not the Maintained
  neighbor
- Opaque never returns `UnverifiedNewer`

Preflight and drift:

- required listed member permits
- unknown required point fails `PreflightDimension::InterfaceVersion` with
  `swallowtail.preflight_rejected` before any effect
- axis mismatch stays instance-missing or driver-incompatible as today
- editing Opaque membership on the descriptor stale-plans with
  `swallowtail.preflight_plan_stale`

Serialization / diagnostics / prepared:

- accessor round-trip equals canonical membership
- `Eq` independent of constructor order
- diagnostic code unchanged
- `baseline` / `latest_qualified` envelope on a two-member set whose text
  order disagrees with numeric build order (`b11429-…` before `b9910-…`)
- `has_version_interval()` false for Opaque, true for a Semantic fixture
- `PreparedInterfaceCompatibility` exposes the matched member assessment

Singleton regressions:

- keep `qualified_only_and_opaque_claims_do_not_infer_forward_execution`
- keep `opaque_windows_are_exact_only` as `min != max` refusal
- keep `installed_executable` and `connection_lifecycle/update` tests
- fixture shaped like a hosted facade singleton (long opaque string)
- fixture shaped like attached `b9910-f5525f7e7` and owned
  `b10069-178a6c449` on **separate** axes; do not merge them

Public API: if `has_version_interval` is added, refresh the current
public-api baseline in that implementation PR. Do not rewrite historical
`release-baselines/public-api-0.5.1/`.

## Out Of Scope

- llama.cpp attached or owned qualification of `b11429` or `v0.6.0`
- adapter `ObservedVersion`, facade id, or driver-id edits
- hosted facade second executable segments
- live, auth, install, artifact execution, tags
- changing Semantic / Integer / CalendarDate interval rules
- raising the 32-member bound
- process or Queue status text in the repository

Route-family adaptation after core implementation needs full exact
published-hop and selected-behavior proof on one family. This design does
not credit that work.
