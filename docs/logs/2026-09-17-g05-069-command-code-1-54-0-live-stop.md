# 2026-09-17 g05.069 Command Code 1.54.0 Live Stop

Status: stopped; typed live evidence failure
Owner: Tom
Milestone: g05.069
Contracts: 029, 043

## Result

The newly authorized exact-`1.54.0` gate ran once with the canonical free
model `meituan/LongCat-2.0:free` and the stored local Command Code account.
The executable was checked immediately before provider use: `command-code`
reported `1.54.0`, and `dist/index.mjs` retained the frozen SHA-256
`157feefa0140e78f060ef2c1f9c50d10de702196ea229dc73db6bbcc39a0bcbb`.

The structured route reached the provider but returned the typed adapter
failure `swallowtail.command_code.headless.malformed_stream` with the message
`Command Code emitted malformed headless stream output`. The accepted
structured completion, tool lifecycle, and usage evidence therefore remain
unproven on `1.54.0`; the required two-turn private exact-id continuation was
not started. The exact model string is part of this observation and no other
model was tried.

The prior exact-`1.54.0` credit-failure observation remains comparison evidence
only. Research 116 and 118 remain immutable exact-`1.15.1` records. The
`command-code.npm` claim stays one exact `QualifiedOnly` point at `1.54.0`,
and the `1.54.2` availability remains an observation rather than a claim
change.

No retry, model substitution, executable update, login, catalogue operation,
or second provider attempt ran. No raw provider stream, account identifier,
prompt, session id, tool body, or private path is retained.

## Current State

The live-derived feature and activity cells remain version-bound and unchanged.
The provider-free selected route evidence and private exact-id boundary remain
valid. This gate does not qualify authenticated completion, tool lifecycle,
usage, or interactive continuation for `1.54.0`.

## Validation

- exact-payload confirmation: `effigy probe:command-code-installed` passed
- pre-gate executable check: exact `1.54.0` and frozen entrypoint digest
- authorized live gate: stopped with the typed malformed-stream failure
- branch: `ns-35815e75-5cc9-4698-92dc-5b70bf54b4c0`
- worktree: clean before documentation closeout

## Next

Chatterbox decides whether a separately authorized future live lane is needed;
this task does not retry or widen the exact point.
