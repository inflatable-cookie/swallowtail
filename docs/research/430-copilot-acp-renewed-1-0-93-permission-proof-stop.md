# Research 430: Copilot ACP Renewed 1.0.93 Permission Proof Stop

Date: 2026-10-09

## Scope

Run the one renewed original `1.0.93` invocation authorized by Tom's board
decision `411be8ce-77a0-4a50-930f-d6aeacdffce9` (“Approve one corrected 1.0.93
attempt”), using the reviewed [Research 429](./429-copilot-acp-proof-startup-diagnostics-correction.md)
correction. Contract 023 records the authority. This work does not qualify a
route, change a claim, or touch an older artifact.

## Authority and admission

The renewal is a separate record set beside the consumed
[Research 428](./428-copilot-acp-host-login-permission-proof-stop.md) record,
which is unchanged. Fixture files:

- `permission-proof-renewal-authority.json`: the grant, operation ID
  `1c00be41-c012-4f12-bac8-87526f72d46b`, the frozen executable SHA-256
  `df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`, the
  permission and correction plan digests, the frozen inventory digest, and the
  harness digest
  `f11f44d43e8baf432ed3627cd75c9630d65f0fd8fb95535cc1437fd1dbc40418`.
- `permission-proof-renewal-attempt.json`: created exclusively and fsynced
  before any staging or launch. SHA-256
  `2516a765988e124329b126dbeaa9740a14a17830a3dcbe2bd5c9d568b14a2222`.
- `permission-proof-renewal-execution-record.json`: created exclusively after
  the run. SHA-256
  `d011ad9363405a1790c372616927394a6d8d0ca9c9b271eab9e261dffed3106f`.

Admission is read-only and refuses missing, symlinked, tampered or mismatched
authority; a consumed or existing attempt or result; and a missing committed
`1.0.93` consumption record. The harness also refuses a digest mismatch on the
staged executable. The fake controls cover each refusal, the CLI refusal before
staging, the single fake launch, and post-consumption refusal. The renewal
authority, the consumed-record guard and the legacy `--execute` and
`--permission-proof` refusals are unchanged.

The fresh preflight was generated from the committed harness in the task
directory. Its SHA-256 is
`affb2dc5c4c89ed3727f434e60b1edde76eaa259e3b24227df684cd9efc7b281`. The
profile generator is the same one described in Research 429. The per-run
profile digest for this invocation is
`c00ccaadfae0c55fceffa1455952f40583a5ece13eddde64571e389828b3611a`.

## Result

The renewed invocation started at `2026-10-09T18:15:23Z` and ended at
`18:15:25Z`. The record shows:

- one `1.0.93` invocation consumed; the attempt was bound before the launcher;
- launcher exit code 1 after 1.289 seconds, before any ACP `initialize`;
- stream failed; vendor startup `unknown`;
- stderr category `sandbox-denial`, 427 bytes, raw text not retained or displayed;
- no egress destination reached the proxy; no authentication request, session,
  prompt, permission request, cancel, or effect;
- process, process group, proxy threads and stderr reader all joined;
- zero prompts used; the shared prompt allowance remains 3 of 3 because this
  invocation sent none.

The harness stopped as designed. No further `1.0.93` invocation and no
`1.0.81` or `1.0.80` invocation was started. The record validates through
`check:copilot-acp-permission-record`.

The host unified log retained no sandbox denial line for the invocation window,
so the denied operation is unobserved. The category does not identify a path,
a runtime root, a home read, or a write target. The cause is unknown.

## Remaining gate

This is a finite safe diagnosed stop, not positive permission evidence or a
compatibility change. The renewal is consumed. Route qualification, the
exact-`1.0.80` control and every route claim are unchanged.

Any further attempt needs new exact authority after the planner decides how
to diagnose the denial without widening. Candidate causes are unproven: an
unmapped runtime or cache path, a home write, a read outside the approved
exception, or the approved `config.json` exception being insufficient for
this binary. Widening the profile during an invocation is out of scope. A new
profile needs a reviewed correction and separate authority.
