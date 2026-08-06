# 2026-08-06 Consumer Front Door And Release Copy

## Result

Card 127 replaces the 877-line root development chronicle with a 177-line
consumer front door. It starts from explicit route selection, exact Git-tag
dependencies, package roles, prepared integration shape, runtime
prerequisites, compatibility, and support instead of roadmap history.

The changelog and `v0.1.0` notes now describe the current 27-package,
33-production-route source-tag candidate. They distinguish completed API,
documentation, security, and guide evidence from the still-pending manifest,
CI, clean-candidate, external-source, and tag-authorization gates. No crates.io
or GitHub Release distribution is implied.

`SECURITY.md`, `SUPPORT.md`, and `CONTRIBUTING.md` establish private security
reporting, safe support evidence, project boundaries, and the contract-first
contribution path. Route-specific installation, authentication, sidecar, and
lifecycle detail remains in canonical guides.

A new deterministic consumer-doc check parses the marked dependency snippet,
requires exact canonical `v0.1.0` Git pins, compares release packages against
the semantic API package baseline, compares release route IDs against the
provider route matrix, and requires the support-policy files. It is part of
ordinary docs QA.

## Validation

- documentation and expanded link checks passed
- consumer front-door check passed: 27 packages, 33 routes, exact source tag
- guide coverage passed: 33 routes, 22 route guides, 32 examples, 14 feature
  families, 11 feature guides, and 43 portable features
- all workspace examples compiled
- Northstar spine checks and `git diff --check` passed

No authenticated provider, consumer, workflow, tag, push, GitHub Release, or
registry effect ran.

## Next

Card 128 converts manifests and stale registry-candidate machinery to the
source-only gate. Its GitHub workflow edit needs explicit operator approval.
