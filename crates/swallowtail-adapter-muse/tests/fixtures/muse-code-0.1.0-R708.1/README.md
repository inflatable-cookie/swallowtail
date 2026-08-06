# Muse Code 0.1.0-R708.1 corpus

Secret-free qualification evidence for the exact installed Muse Code payload
`0.1.0-R708.1` and selected route `muse-code.headless`.

`artifact.json` keeps the Bash launcher and signed Mach-O payload separate. The
launcher may update the payload before delegating. Swallowtail therefore binds
the selected route to the versioned payload, not to the mutable launcher.

`version.txt`, `help.txt`, and `exec-help.txt` are prompt-free direct-payload
captures. `echo-success.jsonl` is a complete direct-payload echo run with
session, command, and task identities replaced consistently. The source run
used `--no-session-log`, disabled write, shell, and web tools, and sent no
provider request.

`meta-success.jsonl` is the field-minimized, correlation-preserving projection
of the authenticated Meta success capture. The source capture contained 26
records and one model step. Account profile, provider request and response
identifiers, raw reasoning, host paths, and non-contract status facets were
removed. The exact provider, model, effort, output, lifecycle order, and
terminal result remain. The final bounded `session.workspace_branch.observed`
record preserves live evidence that non-authoritative observations may follow
the terminal result. It is evidence for selection and lifecycle shape, not a
byte-for-byte provider transcript.

All variable identities use `fixture-*` values. Muse's deterministic event
record ids and timestamps remain because they carry ordering evidence and do
not identify the account or host.

The corpus grants no authority for interactive TUI operation, retained session
recovery, transcript export, tools, writes, approvals, questions, subagents,
usage accounting, or a Meta Model API route.
