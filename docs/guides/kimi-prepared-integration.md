# Kimi Code Prepared Integration

Use the prepared facade for the normal Kimi Code ACP path. It discovers one
host-approved executable, preserves its exact compatibility result, and
derives the configured instance, preflight plan, access policy, and session
request echoes.

## Inputs That Stay Explicit

Preparation still requires:

- one configured-instance identity and revision
- one execution host and approved executable target
- one isolated Kimi state environment
- one membership OAuth access profile and observed or caller-asserted evidence
- caller-owned probe cancellation and deadline

The session profile still requires the selected model route, working resource,
and optional reasoning mode. Swallowtail does not select a model, workspace,
account, credential, or fallback route.

## Prepare The Installation

Construct `KimiPreparationInput` and `KimiPreparationProbe`, then call
`prepare_kimi`. The supplied host services must describe the same host that
will execute the session and include the session services used by preflight.
Only task, time, and process services are used by the version probe.

The result exposes the exact installed observation, access provenance,
configured instance, approved target, and a low-level `KimiAcpDriver` escape
hatch.

## Prepare One Persistent Session

Create a `KimiSessionProfileInput` with:

- request identity
- `KimiModelSelection`
- working-resource reference
- `SessionOptions`

`KimiPreparedIntegration::prepare_session` derives one immutable plan and its
matching new-session request. The plan visibly binds:

- Kimi ACP and the exact executable version
- ambient harness configuration
- `AmbientHost` isolation
- ambient read-write workspace access
- provider-owned durable state prohibited by Swallowtail
- load replay, resume, streaming, active-turn interruption, bounded writes,
  and the selected reasoning mode

Ambient execution is not containment. It makes no filesystem, descendant, or
provider-tool network isolation claim. A future provider- or host-enforced
route requires a separately qualified profile; failure cannot fall back to
this ambient profile.

## Bound Operations

The prepared value exposes distinct operations:

| Operation | Prepared method | Result |
| --- | --- | --- |
| new session | `open_session` | interactive session handle |
| provider load | `load_session` | replay plus interactive session handle |
| provider resume | `resume_session` | interactive session handle, no replay |

Load and resume accept the exact `SessionResumeBinding` previously returned by
a session handle. The prepared facade derives working resource, access,
provider-state, configuration, and plan agreement. It does not reconstruct or
guess a lost binding.

Prompt content remains explicit through `InteractiveSessionHandle::start_turn`.
Active-turn interruption remains explicit through the returned turn handle's
cancellation control. Bounded filesystem writes and delegated authentication
continue through host services; the adapter does not execute consumer tools or
extract credentials.

`plan`, `request`, `evidence`, `low_level_driver`, derived load/resume request
helpers, and `into_parts` remain available for diagnostics and advanced use.

See the compile-tested
[`prepared_acp` example](../../crates/swallowtail-adapter-kimi/examples/prepared_acp.rs).
