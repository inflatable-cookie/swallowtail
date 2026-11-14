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

The trace records actual phase calls, child PIDs and exit codes; no active or
qualified marker supplies proof. It is an ACP-local rehearsal trace, not an
instance of task234's shared supervision record. The private shared seam will
bind its exact source/manifest before composition. Native-entry release,
parent/adoptee supervision, pidfd/cgroup settlement, canonical administrator
fixtures and kernel enforcement remain independent prerequisites owned by234.
This harness adds no public runtime API. Ambient local fake execution does not
prove the original native can use the eventual confined transport.

Run `effigy validate:acp-guard-installed-child` for these two cases and the18 captured-host regressions on the
installed Darwin arm64 runtime; other platforms report not applicable. The
existing installed-child selector includes these cases and the captured-host
regressions. All temporary fixtures belong to the current run's `TMPDIR`.
