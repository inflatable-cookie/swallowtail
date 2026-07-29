# 053 Pi RPC Session Continuity Currentness And Resource-Binding Gate

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Can current Pi RPC load or resume one persisted session while preserving the
exact host-leased working resource required by Contract 017?

## Method

Evidence was accessed 2026-07-28.

- checked every stable package release from the existing `0.80.10` baseline
  through current `0.82.1`
- compared tagged RPC documentation, command types, command handling,
  session-runtime replacement, startup selection, and cwd validation
- checked the existing Swallowtail Pi plan, launch, binding, lifecycle, and
  cleanup paths
- applied Contracts 009, 017, 029, and 038 before designing production
  fixtures
- used no installed executable, account, credential, provider request,
  container, or model server

## Current Release And Milestones

The current maintained release is `0.82.1`. The complete stable interval after
the existing baseline is:

- `0.80.10`
- `0.81.0`
- `0.81.1`
- `0.82.0`
- `0.82.1`

The continuity subset is unchanged across those points:

- `switch_session` accepts one session-file path
- the correlated response follows runtime replacement and RPC rebinding
- `get_state` returns model, session file, session id, counts, and scheduling
  state, but not the effective working directory
- `get_messages` returns the active conversation projection in provider order
- `get_entries` returns append-order session entries and the current leaf
- prompt acknowledgement follows prompt preflight
- `agent_settled` remains the terminal turn boundary

`0.82.1` adds a separate thinking-level query and a bash correlation detail.
Neither changes continuity.

Sources:

- [Pi `0.80.10` RPC](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/docs/rpc.md)
- [Pi `0.81.0` RPC](https://github.com/earendil-works/pi/blob/v0.81.0/packages/coding-agent/docs/rpc.md)
- [Pi `0.81.1` RPC](https://github.com/earendil-works/pi/blob/v0.81.1/packages/coding-agent/docs/rpc.md)
- [Pi `0.82.0` RPC](https://github.com/earendil-works/pi/blob/v0.82.0/packages/coding-agent/docs/rpc.md)
- [Pi `0.82.1` RPC](https://github.com/earendil-works/pi/blob/v0.82.1/packages/coding-agent/docs/rpc.md)
- [Pi `0.82.1` release](https://github.com/earendil-works/pi/releases/tag/v0.82.1)

## Usable Continuity Semantics

If resource binding becomes enforceable, the portable projection is clear.

- The opaque provider reference is the session-file locator inside a complete
  `SessionResumeBinding`. The locator alone grants no authority.
- Load issues `switch_session`, verifies the resulting state, then uses the
  correlated `get_messages` response as the bounded replay phase. Its response
  is the readiness boundary.
- `get_messages` is preferable to `get_entries`: it represents the active
  model context, while append-order entries can include abandoned branches and
  pre-compaction history.
- Resume issues `switch_session` and `get_state`, then returns readiness
  without requesting history.
- The selected Swallowtail launch disables extensions, so
  `session_before_switch` cannot cancel or redirect the qualified profile.
  A malformed or unexpected `cancelled: true` result must still fail closed.
- Process exit remains attachment cleanup, not provider-native session close.

These semantics are not production-qualified because the resource gate below
precedes replay.

## Blocking Resource Behavior

Pi session files store their working directory. Public RPC
`switch_session` passes the requested path to `SessionManager.open` without a
cwd override. The runtime then:

1. accepts the stored cwd when it exists
2. tears down the old session
3. recreates cwd-bound services using the stored cwd
4. returns the successful switch response

The validation helper rejects only a missing stored directory. It does not
compare that directory with the process cwd or the host-leased working
resource. `get_state` does not expose the effective cwd, so Swallowtail cannot
corroborate it after switching.

The CLI `--session` path has the same problem. Startup calls
`SessionManager.open(path, sessionDir)` without an override. No public RPC or
non-interactive CLI argument binds an expected cwd for session attachment.

Sources:

- [Pi `0.82.1` RPC switch implementation](https://github.com/earendil-works/pi/blob/v0.82.1/packages/coding-agent/src/modes/rpc/rpc-mode.ts)
- [Pi `0.82.1` session runtime](https://github.com/earendil-works/pi/blob/v0.82.1/packages/coding-agent/src/core/agent-session-runtime.ts)
- [Pi `0.82.1` cwd validation](https://github.com/earendil-works/pi/blob/v0.82.1/packages/coding-agent/src/core/session-cwd.ts)
- [Pi `0.82.1` CLI session startup](https://github.com/earendil-works/pi/blob/v0.82.1/packages/coding-agent/src/main.ts)
- [Pi `0.80.10` session runtime](https://github.com/earendil-works/pi/blob/v0.80.10/packages/coding-agent/src/core/agent-session-runtime.ts)

This is an authority failure, not a sandbox requirement. `AmbientHost` permits
the harness to access the ambient host. It does not permit provider state to
replace the working-resource identity fixed by preflight.

## Contract Result

Contracts 009, 017, 029, and 038 remain sufficient. Contract 017 deliberately
requires the provider working directory to derive only from the host lease and
requires exact binding agreement before provider or process work. Weakening
that rule would allow a persisted transcript to redirect later harness tools
to another existing directory.

No positive continuity corpus should certify the current route. Cards 097 and
098 remain paused. Existing ephemeral Pi sessions, structured runs, catalogue
discovery, attachments, usage, and other qualified behavior are unchanged.

## Unpause Condition

Revalidate Pi continuity when a maintained public interface can do both:

- attach the requested session with an exact caller-supplied cwd override or
  reject a stored-cwd mismatch before session replacement
- expose enough correlated state to prove the resulting runtime uses that
  exact cwd

That may be an additive RPC field, a separate attach command, or an equivalent
documented non-interactive launch surface. A Swallowtail-specific source patch,
direct session-file parsing, post-switch path guess, or trust in a previously
observed header does not qualify.

## Correction To Research 051

The two Pi cells classified `R` in Research 051 are resource-binding blocked.
The corrected starting-inventory disposition is:

| Code | Cells |
| --- | ---: |
| `R` | 5 |
| `C` | 4 |
| `B` | 3 |
| `U` | 10 |
| `M` | 36 |
| **Total** | **58** |

The matrix itself remains correct: Pi load and resume stay `No`.

## Promotion

- Closed exact currentness through `0.82.1`.
- Preserved the `0.80.10` baseline without inventing a continuity guarantee.
- Identified one exact public-interface gate instead of a shared-contract gap.
- Paused Pi load and resume without changing existing Pi behavior.
- Returned the active matrix lane to provider-retention currentness.
