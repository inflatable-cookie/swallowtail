# Provider Route Matrix And Packaged Proof

Date: 2026-07-25
Roadmap: g02.012
Cards: 034-035

## Outcome

Swallowtail now publishes one exact matrix for all 22 production routes across
six implementation families. Each route names its package, driver, roles,
transport, explicit target and access inputs, version posture, prepared
constructor, bound operations, and low-level escape hatch. Remote ACP remains
an explicit composable transport, not a separate provider route.

The package gate now executes provider-wide facade behavior. It builds one
transient 23-package candidate, extracts the artifacts, runs 20 prepared-facade
suites covering all 22 route identities, then reuses the same candidate for
the existing Nucleus and Soundcheck proof.

## Package Evidence

- candidate source:
  `6799329ed47090c915dce907effb2dcf53427fa6`
- package checksum-manifest digest:
  `38721af1840f3246e94e783888523775400b8353bad152e6296789854c85ff39`
- provider evidence:
  `0e7082187c4a00487f3e331d1e78a41c13dae8d22df8caaaeeccd357bb97a04c`
- consumer evidence:
  `6dd36ffa2ad7fb3946f0cb2dd2c4599aba13855cfd67ac4e9dcd100f50b7ff0b`
- package count: 23
- prepared facade suites: 20
- production route proofs: 22
- prepared facade tests: 65 passed
- Nucleus: 14 passed, two live probes ignored
- Soundcheck: four passed, one live probe ignored
- packaged Codex adapter: 89 passed
- credentials: absent
- installed providers: not required
- provider calls: none

The extracted suites cover exact and unverified-newer observations, binding
drift, failure-before-effects, cancellation, deadlines, redaction, cleanup,
local hosts, and remote-authoritative hosts. They retain each adapter's
low-level public roles.

## Validation

- exact route inventory and uniqueness: pass
- public example compilation: pass
- package assembly, extraction, content audit, and test compilation: pass
- bundled-source regeneration and package/checksum comparison: pass
- prepared package execution for all 22 routes: pass
- packaged Nucleus and Soundcheck proof: pass
- metadata, public API, docs, MSRV, repository QA, and worktree checks: pass

`effigy doctor` retains the known 19 oversized-file findings: seven errors and
12 warnings. No category or count changed.

The first full QA run exposed a fixture-only Gemini Live race: after a second
`goAway` terminal event, the server attempted to emit impossible success
events to an already-closed peer. The fixture now ends that scenario after the
terminal event. Its focused regression and full QA pass.

## Continuation

Card 036 is ready. Replace the held unpublished `0.1.0` candidate from one
clean source commit, refresh its handoffs, and stop before every registry, tag,
push, workflow, or release mutation.
