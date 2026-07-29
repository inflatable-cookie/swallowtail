# 2026-07-28 Pi RPC Session Continuity Resource Gate

## Result

Card 096 reached its stop condition. Pi RPC load and resume are not
contract-ready at `0.80.10` through current `0.82.1`.

## Evidence

The protocol has usable switch, state, ordered-message, append-entry, prompt,
and settled boundaries. The selected extension-free profile also removes
switch redirection.

The public attachment path still accepts the cwd stored in the session file.
It checks only that the directory exists, recreates runtime services under that
directory, and exposes no effective cwd in `get_state`. The non-interactive
CLI session path has the same behavior.

Contract 017 requires the working resource to come from the host lease. An
ambient harness may access the host, but provider state cannot silently replace
the preflight-bound resource.

## Decision

- keep Pi load and resume `No`
- pause cards 097 and 098
- keep the existing ephemeral Pi profile unchanged
- make no shared-contract or runtime change
- revalidate only after Pi exposes a maintained cwd-bound attachment surface

Research 053 carries the exact release, source, replay, binding, and unpause
evidence. Research 051's two Pi-ready classifications are superseded.

## Next

Return to the provider feature matrix. Card 099 audits provider archive,
restore, delete, and operation-owned remote cleanup without borrowing
capabilities across routes.
