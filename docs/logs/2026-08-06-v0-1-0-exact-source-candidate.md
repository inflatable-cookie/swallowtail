# v0.1.0 Exact Source Candidate

Date: 2026-08-06
Roadmap: g03.043
Card: 129

## Outcome

The reviewed release-preparation batch is committed. The clean completion
commit containing this log is the exact source candidate. Its immutable SHA is
reported by Git in the card 130 handoff; a commit cannot contain its own hash.

The candidate parent is
`13bcec5124c7db5c9704dadf2df9956cbfe64430`. That parent contains the complete
runtime, package, documentation, CI, security, baseline, and source-consumer
batch over implementation base
`8bd29856cbb449e1268747f6105b3bbbc3e8cca5`.

## Evidence

- `effigy release simulate`: ready; 11 of 11 gates pass
- `effigy release status --check-gates`: ready; 11 of 11 gates pass
- broad test gate: 1,463 passed; 11 skipped
- exact source consumer: representative packages resolve and compile from the
  clean candidate revision
- `effigy release prepare --plan`: one intended changelog promotion
- release state: absent
- local `v0.1.0` tag: absent
- remote ancestry: `origin/main` is an ancestor of the candidate

Deterministic content evidence:

- `Cargo.lock`:
  `55b097bb2a10056018ac064c83d6075701e94b67346d4e69dfca82041e042d06`
- internal dependency topology:
  `2fc3ad4e6e61b9519c4923ad35c7891f7f29816ce9ae50eadd73f12d0030e5e0`
- 27-package inventory:
  `5716070028ada9e88b1ab233df477a778686fca697f29de5418eeec92229faba`
- semantic API inventory aggregate:
  `96740685be17f721cd241490008f90ba50166128cf168dcf9b8d4f4202eac254`

## Boundaries

No release preparation, release execution, tag creation, push, registry
publication, GitHub Release, consumer mutation, or authenticated provider work
ran. Card 130 owns the exact read-only handoff and stops for explicit operator
authorization before every external mutation.
