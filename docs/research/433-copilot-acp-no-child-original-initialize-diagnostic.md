# Research 433: Copilot ACP No-Child Original Initialize Diagnostic

Date: 2026-10-09

## Scope and authority

Task 167 consumes Tom's board answer “Approve one isolated initialize
attempt” to decision `e05b517f-e4fc-46c4-9d52-435b2422b5cf`. The operation ID
is `8cc634d0-762a-448e-abc5-f43b06a100e0`. It authorizes exactly one
credential-free, provider-free Copilot `1.0.93` initialize under the
[Research 432](./432-copilot-acp-no-child-initialize-diagnostic.md) / PR 484
reviewed no-child isolation. This is a separate diagnostic invocation, not a
permission-proof retry or qualification. Parent 119 remains blocked.

## Bound identities

The original ran on macOS `26.6.2`, build `25G83`, arm64. The imported
`dyld-support.sb` closure is one file, size 2,655 bytes, SHA-256
`06215a5d32689aefe395c29710e182eb54ba22162f50df8b4842290f8a19bf1c`. The
reviewed original-profile template SHA-256 is
`4ec0e6643d8ff8e87e1186b4fb6daf3e1968a820203e32f28e0c2d2e38705023`. The
consumed attempt binds rendered-profile SHA-256
`d2cb9e59d2d20263022e60a35c7a474a863bfc3eed5c6d30870af0d5d6d2401c`.

The retained original is Darwin ARM64 Copilot `1.0.93` executable SHA-256
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`. The
inventory `1.0.93` native archive SHA-256 is
`f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb`. This
diagnostic copied the retained `1.0.93` executable as data into private task
scratch and did not download, install, or replace artifacts. The executed
bytes are that executable.

### Archive-identity gate exception

The brief named archive
`98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71`.
[Research 404](./404-copilot-cli-acp-offline-authentication-stop.md) records
that digest as the Darwin ARM64 `1.0.80` native archive; the `1.0.93` archive
is `f254651a…`. Research 432's immutable `frozen_original_target` labels
`98640ca0…` as the `1.0.93` archive. The original diagnostic hard-codes
`f254651a…`. The executed pairing is executable `df347f…` with archive
`f254651a…`. The brief's stop rule (“any drift refuses originals”; stop
before original if identity differs) would have refused this original. The
run proceeded without an operator ruling on that difference. Authority,
attempt, and execution records have no exception field. Whether the consumed
attempt stands under decision `e05b517f-e4fc-46c4-9d52-435b2422b5cf` is open
for Tom's ruling: [Q-007](../knowledge/questions.md#q-007). Parent 119 stays
blocked. Frozen records and the diagnostic script are not edited.

Harness and launcher identities match the reviewed 432 inputs:

- preparation harness SHA-256
  `b09a9491b2b2d3e670c7402ec3615051f8c2afc39c480fefabf3a0781d2691d5`
- original diagnostic SHA-256
  `7c6ac68b34a8065a3c1e22b296a0325abb28976cf5779e450dbf2a16fe80ce04`
- stage-launcher source SHA-256
  `47b5d140c8deda3fd3a70a111b6aa125e25a453e53ea52b739d36cf642bbc996`
- stage-launcher binary SHA-256
  `d8cfcdb2a87f8c18f23935134c09abf5ccc6bee6d7f527b181be0da62d663e75`
- shared helper SHA-256
  `a9e489a6c143c100411236dc21f4d3ee0484394eb5bbd2b30831f8592f27d510`
- fake source SHA-256
  `ae2b93434590b820bd3e7cd823fba74b49f1a83e7e61f68917b4f6f8bb9bef5f`

Append-only records under
[`copilot-cli-acp-no-child-diagnostic/`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic/):

- [`authority-record.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic/authority-record.json)
  SHA-256 `69b08c5f06a751bd165403a9e58623d092fedb6c2448881fd9e4dcf371678cb5`,
  `execution_authorized: true`
- [`original-profile-proof-record.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic/original-profile-proof-record.json)
  SHA-256 `70e89cd67c58f44f3d737ba60169f7349acb07edf42e8fdbcd5d6654befd929d`
- [`attempt-record.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic/attempt-record.json)
  SHA-256 `422ce490b300bfb615510b0ac1beb028f06fa910cb24c838f01f6528e22f9d52`,
  fsynced exclusive before original start at `2026-10-09T22:03:34Z`
- [`execution-record.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic/execution-record.json)
  SHA-256 `13d5d017a2e3af6e9d76c0ebf5f9152cbf064e7729c72421fd48a613e26482fd`,
  completed at `2026-10-09T22:03:37Z`

The 432 preparation record and harness stay immutable. General permission
entrypoints stay closed. The three permission prompts remain unused. Both
legacy invocation and renewal guards remain consumed.

## Original-profile fake

The launch-path delta places the task-owned fake at
`artifacts/1.0.93/copilot` so the original-profile `process-exec` grants cover
the same relative target the original uses. Fake claims that passed:

- all eight child-creation attempts denied
- real-home, service, and network denials using synthetic sentinels
- same-root exec transition
- EOF without startup remains unknown
- initialize observed once; no `session/new` or `session/prompt`
- bounded secret-safe diagnostics
- exclusive record and crash/replay refusals
- timeout, cancel, root, readers, and stage channel joined

Fake success on that path proves the reviewed no-child grants and lifecycle
controls. It does not prove Copilot startup.

## Original initialize

The original started only after the original-profile fake proof and the
exclusive consumed attempt. Argv is `--acp --stdio`. Cwd is the action root.
Environment keys are the Research 432 synthetic `HOME`, `COPILOT_HOME`,
`GH_COPILOT_HOME`, XDG, and `TMPDIR` roles, with `PATH`
`/usr/bin:/bin:/usr/sbin:/sbin` and `TERM` `dumb`. Token, model, and proxy
variables are absent. The harness sent one initialize request and no
session, prompt, or permission request.

Closed-vocabulary result:

- `initialize_outcome`: `not-reached`
- `vendor_startup_status`: `unknown`
- `failure_class`: `sandbox-denial`
- `agent_version`: `unknown`
- `protocol_bytes`: 0
- `initialize_requests_sent`: 1
- `session_new_sent` / `session_prompt_sent` / `permission_request_observed` /
  `session_update_observed`: false
- `exec_boundary_eof`: true
- `timed_out`: false
- `launch_exception`: false
- `exit_observed`: true
- `exit_success`: false
- `elapsed_milliseconds`: 2626
- `stderr_category`: `sandbox-denial`
- `stderr_marker_facets`: `sandbox-marker-present`
- `stderr_captured_bytes` / `stderr_total_bytes`: 997
- raw stdout/stderr, hashes, paths, and secrets: not persisted
- root, stderr reader, and stage channel: joined

The stage launcher recorded an exec-boundary EOF, so the same-PID target exec
appeared to succeed from the launcher. The process then exited before any ACP
protocol byte. Marker facets record a sandbox stderr marker; they do not name
an operation, path, or denied service. The cause stays unknown.

[Research 430](./430-copilot-acp-renewed-1-0-93-permission-proof-stop.md)
also exited before initialize with `sandbox-denial` under a different profile
and argv. This tuple is the reviewed no-child isolation with `--acp --stdio`
only. The launcher-boundary EOF is new evidence; vendor startup remains
unknown.

## Original versus fake

On the same original-profile grants, the fake returns one ACP `initialize`
and denies the eight child-creation controls. The original does not reach
`initialize`. Fail-closed no-child policy may prevent vendor startup. That
stop is isolation evidence for this tuple, not a route narrowing.

## Limits and next decision

This result is startup evidence for the reviewed no-child tuple only. It does
not prove authentication, Auto model policy, permissions, cancel/no-effect, or
supported versions. The exact `1.0.80` claim, public contracts, and releases
are unchanged. The attempt is consumed. No retry, resend, older artifact,
profile widening, or reviewer original execution is authorized.

Any broader isolation, authentication, state, or permission proof returns
separately. Parent 119 does not continue from this diagnostic. The
archive-identity mismatch is an unratified gate exception; Q-007 owns Tom's
ruling on whether this consumed attempt stands.
