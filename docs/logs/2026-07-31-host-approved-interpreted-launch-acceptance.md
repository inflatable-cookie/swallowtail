# 2026-07-31 Host-Approved Interpreted Launch Acceptance

## Changed

- promoted Research 084 into Contracts 010 and 032
- added one redacted `LocalExecutableLaunch` for an exact program, immutable
  prefix arguments, and bounded bootstrap environment
- retained native executable approval as the zero-prefix case
- applied `env_clear()`, bootstrap environment, then explicit request
  environment in a fixed order
- counted immutable prefix and driver arguments under one host limit
- moved the Pi installed selector from direct `Command` execution to the real
  discovery role and local host services

## Current State

The local host can now launch interpreted npm, Python, Ruby, JVM, or similar
entrypoints without exposing launcher mechanics to adapters or portable
runtime records. Selection remains explicit host policy. The host does not
search PATH while executing, invoke a shell, or inherit ambient environment.

The installed Pi `0.83.0` npm script is accepted through an exact Node program
plus exact script prefix. The resulting observation still describes Pi
`0.83.0`; Node remains private launch machinery. No automatic shebang resolver
or provider-specific package-layout rule was added.

## Validation

- 29 host-local tests passed
- 41 Pi tests passed
- the gated installed Pi selector passed in 0.97 seconds
- extracted 37-file host-local and 85-file Pi packages compiled in six seconds
- no broad workspace suite ran
- no provider prompt, authentication, credential acquisition, network request,
  installation, or workspace effect ran

## Residual Risk

- the consumer or host selection layer must still choose and approve exact
  interpreter and script paths; automatic shebang resolution is not implied
- launcher bootstrap environment is host authority and must not be used for
  credentials or provider configuration
- remote-authoritative hosts must implement the same opaque recipe semantics
  locally; local paths never transfer across hosts

## Continuation

Run the g03 compatibility-maintenance checkpoint before compiling g03.012.
Standalone Claude ACP, further Gemini range qualification, Pi persisted
load/resume, provider-session binding persistence, and registry publication
remain paused or deferred under their existing gates.
