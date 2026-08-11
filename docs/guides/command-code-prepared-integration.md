# Command Code Prepared Integration

Use `swallowtail-adapter-command-code` for the installed Command Code harness.
The route is `command-code.headless`; the driver ID is
`swallowtail.command-code.headless`. It owns exact read-only structured runs and
Contract 043 interactive continuity over Command Code NDJSON AgentEvent stdio.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

This package is additive unreleased source after `v0.3.1`. Consumers must pin
an explicitly reviewed commit containing it. Immutable `v0.3.1` and earlier
tags do not contain the package or route.

## Selected Boundary

The route selects:

- exact npm release `1.15.1` on axis `command-code.npm`
- unambiguous executable basename `command-code`
- one explicit model id
- one read-only filesystem working resource
- ambient local-account configuration
- plan-mode permissions (`--permission-mode plan`)
- structured runs with `--no-session` and prohibited retention
- interactive turns that omit `--no-session` and privately resume with exact
  `--resume <sessionId>`
- no auto-update (`--no-auto-update`)
- one attached child process per structured run or interactive turn

Prefer `command-code` over `cmd` / `cmdc`. Swallowtail does not search PATH for
family crossover, install Command Code, run login, choose a model, invoke
taste/mods/skills, scan `~/.commandcode/projects`, or call the Provider API.

## Local Account Access

Call `command_code_local_account_access_profile` with an application-owned
`AccessProfileId`. Supply matching `PreparedAccessEvidence` with:

- `CredentialState::NotRequired`
- `EntitlementState::Available`
- `EndpointAuthorization::Allowed`
- `RuntimeReadiness::Ready`
- `SupportAuthority::ProviderSupported`

This means Command Code owns its local account state. It does not mean
anonymous access. Swallowtail receives no credential reference and opens no
credential lease. The approved process environment may carry `HOME` or another
operator-approved configuration root. Do not copy tokens into an environment
value or portable evidence.

## Prepare The Installed Executable

Construct `CommandCodePreparationInput` with:

- configured instance ID and revision
- execution host ID
- `InstalledExecutableTarget` for the approved Command Code launch
- approved `EnvironmentRef`
- local-account profile and matching evidence

The npm `command-code` bin is a Node shebang launcher. Local hosts must approve
an interpreted launch via `LocalExecutableLaunch::interpreted_script(node, script)`
(or `approve_installed_executable_launch` with that recipe). Approving the
script alone fails closed: ambient `PATH` is cleared, so `env node` cannot run.

Construct `CommandCodePreparationProbe` with a request ID, scope, deadline, and
`DiscoveryCancellation`. Then call `prepare_command_code_headless`.

Preparation runs only `--version`. It requires the exact line `1.15.1`, binds
the resulting compatibility observation, and returns
`CommandCodePreparedIntegration`. It sends no prompt and does not test account
credits. A wrong axis, wrong version, wrong host, unavailable discovery result,
or mismatched access profile fails closed.

## Structured Run

Create `CommandCodeHeadlessModelSelection` with an application-owned route ID
and revision, provider id `command-code`, and one explicit model id. There is
no default model and no first-route effort selection.

Create `CommandCodeRunProfileInput` with request ID, model selection, prompt
`OperationContent`, read-only `WorkingResourceRef`, and deadline. Call
`prepare_run`, inspect evidence/plan/request, then `start_run`.

The command fixes print mode, JSON output, plan permission mode, skipped
onboarding, `--no-session`, no auto-update, trust, no skills, a bounded turn
cap, and explicit model. The prompt is written to stdin and stdin is closed.
Never use `--yolo` on this route.

## Interactive Continuity

Create `CommandCodeSessionProfileInput` with request identity, exact model,
working resource, and deadline. Call `prepare_session`, then `open_session`.
Each turn starts one joined Command Code child in the same approved working
directory.

The first turn omits `--no-session` and omits resume selectors. After a clean
completed turn, Swallowtail privately retains only the exact `sessionId` from
the result line and passes `--resume <sessionId>` on later turns. Failed,
cancelled, timed-out, or mismatched turns do not advance that private state.
Ambient `--continue` and `--fork-session` are never emitted. The id never
becomes a public `SessionResumeBinding`, load, or resume operation.

For every turn, drain events and terminal concurrently, then close the turn.
Closing the session joins local work and leaves Command Code-owned project
transcripts alone.

## Observed Activity

Observed activity includes assistant text, reasoning summaries, provider-owned
tools, usage, and namespaced unknowns. Tool input/result bodies and
`run_end.result.nextState` are not projected into stable diagnostics. Exit
`10` / credit exhaustion maps to portable `QuotaExhausted`.

Keep terminal status separate from cleanup. Cancellation force-stops the child;
deadline and cancellation remain distinct; close joins the pump task.

## Unsupported

This route exposes no:

- model catalogue
- public load, resume, import, catalogue, or provider export
- write/shell/`--yolo` authority
- effort/reasoning selection
- permission or typed-question callback
- attachment or consumer-tool exchange
- structured output or output-token ceiling
- retained background work, stream reattachment, or recovery
- subagent topology or control
- taste, mods, skills, or Provider API
- TTY `/export` / `/sessions` automation or home-directory session scanning

Promotion gate for catalogue/export: reopen only when Command Code exposes a
non-interactive session list or export surface on a qualified release. Do not
scan `~/.commandcode/projects` as a catalogue substitute.

### Working-State Restoration Disposition

Prepared sessions expose `prepare_working_state_restoration` as fresh
context-losing replacement only. Structured runs disable session retention.
Interactive continuity is private exact-id within one process handle and does
not recover after process loss. There is no public load/resume path.

The normal public shape is shown in
[`prepared_command_code_headless`](../../crates/swallowtail-adapter-command-code/examples/prepared_command_code_headless.rs).

## Validation And Optional Probes

Deterministic validation:

```sh
effigy validate:focused swallowtail-adapter-command-code
effigy package:verify-affected swallowtail-adapter-command-code
```

Optional operator-gated evidence:

```sh
SWALLOWTAIL_LIVE_COMMAND_CODE=1 effigy probe:command-code-installed
SWALLOWTAIL_LIVE_COMMAND_CODE_PROMPT=1 \
  SWALLOWTAIL_LIVE_COMMAND_CODE_MODEL=deepseek/deepseek-v4-flash \
  effigy probe:command-code-plan
SWALLOWTAIL_LIVE_COMMAND_CODE_PROMPT=1 \
  SWALLOWTAIL_LIVE_COMMAND_CODE_MODEL=deepseek/deepseek-v4-flash \
  effigy probe:command-code-interactive
```

Live probes spend subscription credits through an already configured local
account. They must not mutate the workspace. Neither probe exposes credential
values to Swallowtail. The interactive probe requires the same working
resource for both turns.
