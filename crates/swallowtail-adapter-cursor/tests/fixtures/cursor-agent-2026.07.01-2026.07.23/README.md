# Cursor Agent 2026.07.01 And 2026.07.23 Compatibility Corpus

This secret-free corpus freezes two exact Cursor Agent calendar releases and
their opaque build revisions. It does not infer support for dates between the
two milestones.

The current official `2026.07.23-e383d2b` macOS arm64 archive matched the ACP
registry digest and reported its exact version. One prompt-free ACP initialize
exchange retained wire version 1, `cursor_login`, and the selected capability
shape with no stderr. No session was created and no provider prompt was sent.

The headless output-format and prompt-builder modules are byte-identical. The
ACP implementation adds an internal disabled-web-search guard, while the CLI
adds auto-review, worker, and empty-chat commands. Those additions are not
selected by Swallowtail and grant no new authority.

No fixture contains credentials, account identity, model observations, host
paths, session identifiers, prompts, or provider payloads.
