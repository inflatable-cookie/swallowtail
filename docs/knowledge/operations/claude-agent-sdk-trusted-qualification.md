# SDK disposable trusted-runner qualification

Status: Linux x64 and Darwin arm64 each passed 100 original cases at
9e5ee425dd7ec53a65402dc42a1d24595d988bc7. A final deadline-recipient race correction awaits changed-input proof. The proposed qualification through
0.3.322/2.1.322 now has actual route proof; normal independent review, current
CI and the planner's qualification hold remain merge gates.

Tom's decision1ec44a26-3d20-45ab-84e9-1523c2dfbbe4 v2 on2026-11-17
replaces the bespoke test-host admission ladder with trusted disposable
GitHub-hosted Linux x64/macOS arm64 runners and ordinary OS/runtime libraries.
Rolling image builds are observed metadata, not admission pins. Recursive
ELF/OpenSSL/Python custody, cache chmod, ptrace/pidfd/adoptee/cgroup attestation
and caller-provenance catalogues are retired prerequisites for this test
procedure. The three consumed manual failures and exact source history remain
in [retirement provenance](claude-agent-sdk-prototype-retirement.md).
No original execution or installation is authorized on the persistent Mac.

## Original route proof

[Run44092489812](https://github.com/inflatable-cookie/swallowtail/actions/runs/44092489812)
checks out9e5ee425dd7ec53a65402dc42a1d24595d988bc7 explicitly. The
[receipt](claude-agent-sdk-trusted-qualification.json) binds tested source,
artifacts, results and exact Node22.23.3 bytes. GitHub's reported SHA is the
PR merge context; it is recorded separately from the actual checkout.

Each platform's matrix has 100 cases across the317 control and318/319/321/322:
35 direct SDK cases,30 shipped-sidecar cases and 35 through the actual Rust
prepared facade, LocalProcessHost, permission exchange, event projection and
session lifecycle. The original SDK modules and matching native binaries are
verified against the unchanged frozen plan; no fake receives qualification
credit. Owned-fake self-tests first exercise the same runner and assertion
function, including mutations for missing, misordered and incorrect outcomes.

Allow and deny cases require Write activity, permission before effect, exact
callback counts and continued output. Shipped/prepared user settings remain
excluded by settingSources:[] and denial stays callback-mediated. A separate
direct SDK control selects user settings: from319, both selected-user and
managed denyInsteadOfPrompting suppress the callback, deny the tool and
continue the turn.317/318 retain their observed callback behaviour. Each newer
pair is executed; the318-to319 source delta agrees with these observations.
Package/native behaviour v1 stays through318 and v2 starts319;320 is excluded.

Prepared cancellation requires Cancelled after an explicit request; the caller's
real deadline API requires TimedOut without that request. A real 400 API error
requires ProviderFailed. Strict assertions require ordered activity/results,
denial continuation, actual native exit and driver wait, and a successful
cooperative close receipt. Two minimal product fixes classify the native's
requested aborted_streaming result as cancellation and retain a timed-out turn
to drain exactly one expected native abort before close. Other late events and
duplicate terminals still refuse; no new activity or permission is projected.

The ordinary host reports root-only completion as Degraded with the exact
close_root_only_degraded diagnostic. Proof retains that limit: positive native
and driver waits do not attest an empty descendant tree or hostile same-user
containment. Every case uses isolated HOME/settings/workspace, explicit dummy
credentials, owned loopback peers, finite process limits and controlled effects.
Linux uses a standard network-none Trixie container; Darwin uses the existing
loopback-only OS sandbox. Bounds remain 45s/case plus 5s stop/join, 1MiB output
per case, 16MiB artifacts and 20min jobs. No live provider or paid turn runs.

## Retained scopes and failures

The earlier 140 observations at61ac73331ad94ed35c449b075bea8811c1d65213
([44072626492](https://github.com/inflatable-cookie/swallowtail/actions/runs/44072626492))
remain direct SDK/sidecar components. Their timer-labelled deadline was an abort
control, not typed Rust deadline proof. They are not transferred as prepared
route evidence. Earlier44070325578 retained incorrect user-setting assumptions
and a Darwin limit failure; corrected direct44071249908 passed its component
scope. Run44087907051 then exposed a Linux container ABI mismatch and incomplete
prepared deadline waits. The Trixie correction at392b09faf68da7cb4be0e59b61c9dac812557a5a
([44089153111](https://github.com/inflatable-cookie/swallowtail/actions/runs/44089153111))
left five Linux and one Darwin deadline-wait failures. None is relabelled a pass.
The new deadline regression also retained a failed virtual-clock fixture setup
before its correction; 52 focused adapter tests and nine owned harness tests pass.

The Node22.23.2..22.23.3 axis, older pairs/default, live evidence and exclusions
remain separate. No Node22.23.4 widening, Windows or ACP transfer is claimed.
This evidence qualifies only the tested trusted-machine selected flows; normal
review/CI and planner hold release remain the merge gates.

The private reuse invariant is stricter than public completion: a TimedOut
recipient remains bound until its exact native terminal has drained and its
correlated interrupt command has settled. Expiry, event dispatch and replacement share the
connection lock; a pending or unknown drain returns typed turn_active without
sending another query. Bounded session close still owns stop and actual waits.
Normal completion/reuse and public permission/projection semantics stay intact.

GitHub could not acquire a hosted arm64 runner for the macos-latest job in
[44114321537](https://github.com/inflatable-cookie/swallowtail/actions/runs/44114321537/job/134148083380).
No command ran, so that cancelled job supplies no test evidence. The existing
qualification job now selects macos-15, a standard arm64 label in
[GitHub's runner table](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).
This stays within the trusted disposable OS ruling; the actual image is
observed metadata, not an admission pin. SDK/native/Node identities and bounds
stay unchanged.
