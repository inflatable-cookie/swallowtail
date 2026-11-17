# Guarded ACP route rehearsal

Repository-owned ACP, SDK-hook, native-child, endpoint and settings fakes run
through `prepare_claude_agent`, checked captured loading and real prepared
session operations. This is software integration proof, not kernel confinement
or evidence about original ACP/SDK/native behavior.

`entry.mjs` handles actual initialize/new/load/resume/config/prompt/permission/
cancel wire traffic. `sdk.mjs` starts a real `ChildProcess` under the registered
native facade and waits its exit. The captured `native.cjs` program runs on the
host-admitted Node; it reads only owned fixture settings/tool data and sends an
ordinary TCP request to the owned ephemeral loopback endpoint. No provider is
contacted. User settings, absent/disabled managed policy and managed denial are
separate inputs. Managed denial skips the permission callback and tool read,
but receives continued output. Allowed reads happen only after a real callback
response. The tests check activity ordering, persisted model identity across
load/resume, unchanged settings, cancellation, native exit and resource cleanup.

The direct owned trusted-runner self-test checks the actual Rust sibling route,
startup-shaped SDK fallbacks and joined cleanup. The older shared Python actor/
kernel proposal is archived at its accepted historical head. No shared host
admission is required for the disposable original qualification procedure.

Run `effigy validate:claude-agent-acp-trusted-native-self-test` for the direct
owned route, or `effigy validate:acp-guard-captured-child` for captured-loader
and focused refusal/lifecycle regressions. Original qualification runs separately
on disposable CI and receives no credit from these fakes.
