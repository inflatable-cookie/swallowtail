# 2026-07-23 Codex Six-Month Legacy Corpora

## Changed

- Froze ten exec checkpoints from `0.80.0` through the current `0.122.0`
  boundary.
- Froze eight app-server checkpoints from `0.80.0` through the current
  `0.110.0` boundary.
- Recorded exact npm artifact versions, integrity values, tagged commits, and
  relevant source digests.
- Split exec behavior into retained boolean-search, retained search-mode,
  ephemeral ambient, and current suppressed segments.
- Split app-server invocation into default-stdio and explicit-listener
  segments.
- Proved the selected v2 request and notification surface at `0.80.0`.
- Kept source-generated schemas at `0.80.0`, `0.81.0`, and `0.84.0` distinct
  from upstream-published schema artifacts at `0.94.0` and later.
- Rejected unpublished `0.82.0`, `0.83.0`, `0.108.0`, and `0.109.0`, plus
  malformed and unknown-newer points.

## Boundaries

- no production descriptor, compatibility claim, driver, or capability change
- no v1 facade, dynamic tools, provider requests, workspace roots, or
  workspace writes in the legacy app profile
- no container, temporary credential home, copied authentication, live
  provider request, or consumer edit
- fixture diagnostics expose only safe classification fields

## Validation

- eight focused current and legacy Codex corpus tests pass
- docs QA and diff checks pass
- doctor retains the inherited seven errors and twelve warnings

## Next

Card 118 adds private exact-version dispatch to the existing Codex drivers.
Ambient configuration and durable local retention require exact request
agreement; current suppressed behavior cannot degrade.
