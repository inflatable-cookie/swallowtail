# Kimi Code 0.31 Range And Live Closeout

Date: 2026-07-30
Status: complete

## Changed

- qualified Kimi Code ACP from exact `0.28.1` plus
  `0.29.0..=0.31.0`
- qualified Kimi Code headless across `0.29.0..=0.31.0`
- advanced the ACP and headless compatibility claim identities
- moved the visible unverified-newer test point to `0.32.0`
- retained the independent local-server ceiling at `0.29.2`
- added exact `0.30.0` and `0.31.0` source, activity, and live-proof
  provenance
- declined the separate Python `kimi-cli` route
- kept Grok account-gated and provider-session binding persistence deferred

No compatibility shim, provider fallback, Python route, consumer edit, or
publication task was added.

## Live Evidence

The installed native macOS arm64 Kimi Code executable is `0.31.0`. It lives
under the Kimi-managed home bin directory, which is absent from this Codex
process's PATH. The exact installed probe passes when that directory is
supplied. Official Kimi IDE guidance documents the same GUI PATH condition and
supports absolute executable configuration.

Two authenticated fixed-output probes passed:

- headless stream-JSON returned the exact requested assistant token and the
  qualified resume-hint terminal record
- ACP reported `0.31.0`, initialized, created a session, streamed the exact
  requested assistant token, and ended the turn normally

Both probes used empty temporary workspaces and authorized no callback, tool,
workspace write, destructive action, or local-server launch. Kimi retained
its normal session state. Repository evidence contains no credential, account
identity, raw provider payload, or session identifier.

The ad-hoc ACP launcher needed a terminal interrupt after the completed
long-lived subprocess did not exit within five seconds of SIGTERM. No process
survived. This does not replace deterministic production-driver cleanup
evidence: the driver retains request-stop, force-stop, and joined wait.

## Source Boundary

Exact `0.30.0` selected source is byte-identical to `0.29.2`. At `0.31.0`,
headless rendering and ACP protocol source remain stable. The local WebSocket
broadcaster changes subagent status projection, so the local-server route
remains visible unverified newer.

One executable version therefore maps to different route guarantees:

| Route | Guaranteed ceiling | Later stable posture |
| --- | --- | --- |
| ACP | `0.31.0` | visible unverified newer |
| headless | `0.31.0` | visible unverified newer |
| local server | `0.29.2` | visible unverified newer |

## Validation

- 85 focused Kimi deterministic tests pass across library, headless,
  local-server, ACP, discovery, reasoning, prepared, and activity suites
- the one installed `0.31.0` live probe passes when explicitly gated
- authenticated headless and ACP fixed-output probes pass
- `effigy format:check` passes
- `effigy qa:routes` passes: 26 production routes and 22 solution rows
- `effigy qa:docs` passes
- no broad workspace, package, consumer, or release suite ran

One initial local-server corpus assertion failed because a human-readable
provenance note omitted the `0.30.0` commit already present in the machine
fixture. The note was corrected and the failed suite passed.

## Decisions

- Python Kimi stays declined unless a concrete Kimi Code capability gap
  appears.
- Grok resumes when the operator supplies authorized account state.
- provider-session binding persistence stays deferred until a consumer needs
  management authority after restart.
- registry publication remains closed until a new roadmap follows months of
  consumer usage proof.

## Next

No ready card remains. The operator may provide the Grok account handoff when
available or select another g02 stabilization target. Swallowtail does not
wait on the independent Soundcheck adoption thread.

