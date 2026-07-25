# 035 Ollama Prepared Version Posture Delta

Status: accepted
Owner: Tom
Created: 2026-07-25

## Question

Should the prepared Ollama route retain Research 024's hard
latest-qualified ceiling after Contract 029 and the operator decision that
upper bounds represent guaranteed support rather than automatic denial?

## Evidence

Research 024 established a guaranteed native text window from `0.14.0`
through `0.32.1`, plus one known `0.32.2` release-state exclusion and semantic
prerelease rejection. It predates the three-way compatibility policy promoted
by Contract 029.

Contract 029 now separates:

- qualified versions inside the maintained guarantee
- exact ordered stable versions permitted as unverified newer
- incompatible versions below the baseline, in gaps, explicitly excluded,
  malformed, or prerelease

The operator has accepted this as the general consuming-application posture:
applications may warn or reject, but Swallowtail must not hard-deny a stable
newer point solely because qualification has not caught up.

## Decision

Revise the Ollama claim to `ollama.native-runtime-window-2`:

- keep `0.14.0` through `0.32.1` as the guaranteed maintained window
- keep exact `0.32.2` excluded because Research 024 identified it as a
  non-qualified release point despite its plain semantic version string
- keep semantic prereleases incompatible
- permit exact later stable versions as unverified newer through
  `ollama.native-text-v1`
- preserve the exact observed version in the configured instance and plan
- run the same endpoint, inventory, selected-model, runtime-drift, codec, and
  cleanup checks for qualified and unverified attempts

This grants no model, endpoint, provider, credential, route, lifecycle, or
fallback substitution. It changes execution posture, not the guaranteed
support range.

## Promotion

Contract 031 now carries the durable Ollama mapping. Card 022 owns the
prepared probe, exact assessment evidence, and deterministic qualified,
unverified, excluded, prerelease, and drift coverage.
