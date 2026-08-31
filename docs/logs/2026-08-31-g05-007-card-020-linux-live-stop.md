# 2026-08-31 g05.007 Card 020 Linux Live Stop

Status: complete; evidence stop
Owner: Tom
Milestone: g05.007
Card: 020
Contracts: 044, 059, 060
Repair head: `adb04f17`

## Sanitized Result

The bounded per-platform digest repair landed at `adb04f17`. Credential-free
proof binds rustc `linux`/`x86_64` to Research 261's official `linux-x64`
digest and rustc `macos`/`aarch64` to the existing `darwin-arm64` digest, and
it rejects every other OS/arch pair including npm-style `darwin`/`arm64`.

Pre-contact checks on that clean head passed. The selected host was
`linux-x86_64`. Installed Claude Code was exact `2.1.251`. The native digest
was
`fd5f10ff0eb58daec04900466b143ea98aab50abf208a422bc008eaec13f61f7`.
`ANTHROPIC_API_KEY` was absent. Source was clean.

One `effigy probe:claude-code-watcher-live` turn then ran with exact model
`claude-haiku-4-5`. The ordered recorder retained one fact for turn
`claude-code-headless:live-claude-code-watcher`: `JoinedZero`. Missing, in
oracle order: MCP initialize, reserved tool discovery, watcher start, Stop-hook
start, Stop-attributed completion-gate, Stop-hook response, same-session
continuation, explicit wait or stop, and provider success. No session
correlation was observed. Terminal cleanup produced `JoinedZero`; provider
success did not.

No raw provider or HTTP body, prompt, endpoint, bearer, credential, path,
command, argument, environment, PID, watcher output, or source artifact is
retained. The attempt is consumed. No watcher capability, matrix, guide, or
version-range claim follows. No rerun, Darwin dispatch, or second provider
turn is authorized.

## Validation

Repair head `adb04f17`, before contact:

- `cargo fmt -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-adapter-claude-agent -p swallowtail-testkit -- --check`
- `effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent swallowtail-testkit` — 585 passed
- `effigy package:verify-affected swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent swallowtail-testkit` — 4 packages
- `effigy package:api` — 40 packages at v0.3.3
- `cargo test -p swallowtail-adapter-claude-agent --features live-probes --test live_watcher_probe` — 3 passed, 1 ignored
- the same test `--no-run`
- `git diff --check`

`effigy doctor` on the planning base was 390 god-file findings (341 warnings /
49 errors) plus one generated-in-source warning. That inherited baseline was
not widened by this lane.

## Current State

Card 020 is complete as an evidence attempt and stopped as a capability proof.
g05.007 is stopped after live evidence. Card 011, g05.003, and PR 127 stay
unchanged. Watcher claims remain unpublished.

## Next

Reassess g05 after this stop. Any later live attempt needs fresh operator
authority. This worker does not merge and does not run another provider turn.
