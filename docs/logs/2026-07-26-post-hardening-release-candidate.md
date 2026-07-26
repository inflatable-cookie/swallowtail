# Post-Hardening Release Candidate

Date: 2026-07-26
Roadmap: g02.014
Card: 044
Candidate: `0.1.0`

## Outcome

The active non-published candidate now freezes the accepted Nucleus and
Soundcheck hardening corrections. Its exact clean source is
`f63a9a3653c7e17e795b06935b280a8fbbcf87cb`, with
`5326e6f4b24d7d05978b9bd4dc8407ccb3b9a565` as parent.

The pre-hardening `f142d927767f` candidate remains immutable under
`.effigy/release-candidates/superseded/0.1.0-f142d927767f/`. An intermediate
`5326e6f4b24d` rebuild passed package and route gates but contained stale
packaged front-door wording. It remains immutable under
`.effigy/release-candidates/superseded/0.1.0-5326e6f4b24d/`.

## Change Classification

Public declaration hashes are unchanged across all 23 crates. Package
versions, provider ranges, operation roles, and access boundaries are also
unchanged.

The guaranteed-behavior delta is corrective:

- prepared Codex tool bounds derive from the actual bounded declarations
- prepared Codex sessions bind the promised time service
- failed Codex version probes retain their stable code while reporting safe
  exit status and bounded sanitized stderr

## Evidence

- package manifest:
  `e2037b0c931b0ecf37a18491f5c233324f02417987baea00c9b322802edfbfad`
- source bundle:
  `19ade5f8fc51c988504f392610902a8495cc950466ba323cd41df66582dce1be`
- evidence manifest:
  `6ab674a25db82173b4105a92d38fa75d9000c14149c83449ed4266552c0f5ec3`
- provider evidence:
  `d6f58b0cc4ed4cfc1cd3a43d2f0513ce700956128685cad01c576d9c0a22f0b2`
- consumer evidence:
  `f2587c5aebe6b55d6442fed936b7453f6dea8d8b84f4c14cf71395d3ab47ee95`
- 23 package archives reproduce from the retained source bundle
- 20 prepared-facade suites cover all 22 production routes
- Nucleus: 15 passed, 2 live probes ignored
- Soundcheck: 6 passed, 1 live probe ignored
- packaged Codex: 93 passed
- credentials: absent
- provider calls: none

The isolated consumer evidence binds exact synthetic source snapshots. It did
not mutate Nucleus, Soundcheck, Soundcheck Library, or Signal.

## Reassessment

The candidate passes the current technical gates. Publication is not
recommended yet. The operator requires ordinary working-application soak
before crates.io.

Remaining risks are separately gated hosted authentication and installed
runtime probes, Apple Silicon-only target evidence, mileage-may-vary
unverified-newer interfaces, synthetic consumer snapshots, and uneven
consumer command-boundary harness coverage.

This exact candidate is the soak baseline. Concrete defects should be
reproduced at the narrowest owning adapter or consumer-backend boundary before
repair. No synthetic UI workload is required.
