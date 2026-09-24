# Codex CLI 0.155.1 currentness corpus

Secret-free identity for official npm `@openai/codex` `0.155.1` before
Swallowtail raises the `codex.cli` ceiling past `0.154.0`.

Downloaded official binaries were hashed and never executed. The public
GitHub source tree was compared blob-for-blob at all three release tag
commits (`rust-v0.154.0`, `rust-v0.155.0`, `rust-v0.155.1`). The published
stable hops after the prior ceiling are exactly `0.155.0` and `0.155.1`.
The entire exec tree and the selected schema params (`ModelListParams`,
`TurnStartParams`, `ThreadStartParams`, `ThreadReadParams`,
`ThreadResumeParams`, archive/delete, turn/interrupt, initialize) are
byte-identical across the window. `ClientRequest` and `ServerNotification`
gain only unmapped thread-attachment methods and one notification.

Selected delete still enumerates spawn descendants, hard-deletes through
`thread_store.delete_threads`, and emits `thread/deleted`. Wrapper
`README.md` and `bin/codex.js` are byte-identical; `package.json` files are
version-and-pin only. Platform trees add an unmapped voice-host/GStreamer
bundle at `0.155.0`. Unpublished `0.154.1` becomes an interior gap;
unpublished `0.155.2` is the first later stable. No decoder update required.
This environment has no host `codex` on `PATH`; nothing was installed.
