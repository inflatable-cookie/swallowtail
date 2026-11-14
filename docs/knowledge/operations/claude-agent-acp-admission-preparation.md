# Guarded ACP admission preparation

[Review5904122860](https://github.com/inflatable-cookie/swallowtail/pull/546#pullrequestreview-5904122860)
accepts the source/fake milestone at5bb3579c. Its `proof`, `source_cleanup` and
`source_actor` verdicts cover owned fakes only; `outcome` remains insufficient
for qualification. Preserve that proof and the earlier failed-run limits.
No settled test or spent original grant is replayed here.

The [exact preparation record](claude-agent-acp-admission-record.json) binds
retained graph/artifact identities, selected execution-import parents, dynamic
source cautions and native identities for all eight hops. It adds a bounded
[Python input inventory](claude-agent-acp-python-inputs.json) and the finite
composition delta owned by234. These records describe admission requirements;
they supply no admission capability, original command or new qualification.

## Selected edge boundary

| Edge | Established boundary | Remaining original-admission requirement |
| --- | --- | --- |
| ESM/CommonJS/JSON | Captured static file/import/builtin identities; unknown module resolution refuses. | Static reachability is not complete call/dataflow or native closure. |
| Child processes | Production facade checks held native bytes/path and caller-specific bounded `ps`. | Four import parents per hop remain in scope. Exact argv/env/cwd/stdio/detached policy, one-shot allowance and entry release need selected original binding. |
| Accepted fake launch | The separate fake recipe uses the reviewed f902 one-shot facade and trusted entry. | Do not project that stricter recipe onto the original SDK or production path-only facade. |
| Workers/VM | Unlisted `worker_threads`/`vm` imports refuse captured resolution. | This does not prohibit opaque native workers or raw process capabilities. |
| Dynamic code/addons | Node startup disables string code generation and addons. | Retained SDK/Zod `Function` sites need selected compatibility proof. Do not relax startup controls from fake success. |
| Ambient process access | Production bootstrap disables `getBuiltinModule`. | It does not establish that `process.binding`, `_linkedBinding` or `execve` are blocked. Classify selected uses or mediate them before effects; normal builtins are not a hostile-code sandbox. |
| Native image | Each hop/platform retains exact image SHA256 and size. | Those identities and bounded permission fragments do not close descendant exec, workers, loader or permitted-executable argv bypass. This remains opaque, with no further reverse engineering inferred. |

ACP caller names change across the v1 relocation: the first three hops include
`acp-agent.js`; later hops include `auth-status-reporter.js`. Every hop also
has `index.js`, session-index live-registry and SDK child-process import
parents. Their presence is not proof of invocation, and their filenames are
not evidence that they are unreachable. SDK native resolver parents, other ACP
callers and the separately identified `ps` query must not be flattened into a
single SDK hook allowance.

## Small Python inventory

Read-only Mach-O inspection identifies the installed framework-app executable
and its non-system `Python3` framework image. The app loads it through
`@executable_path/../../../../Python3`. The framework's own `@rpath` install-name
is `LC_ID_DYLIB`, not another missing dependency. The inspector distinguishes
it with `otool -D`; it never guesses an `@rpath` search result. CoreFoundation
and libSystem remain trusted system-loader inputs under the settled host
boundary, without extracting or claiming to attest the OS shared cache.

The inventory binds17 direct import roots from the exact entry/host sources,
their available source candidates, observed direct cache candidates and the
zip candidate. No direct cache candidate or zip was found in this observation;
that does not prove no transitive bytecode/extension input exists. `-B` prevents
bytecode writes, not reads; `-I` and `-S` do not turn ordinary Python imports
into captured loading. Exact transitive imports/extensions, cache or zip
selection, prefix/path configuration and coordinated non-system runtime
custody remain host inputs. This is the smallest direct-input inventory,
not complete Python admission or a general OS cache project. System-loader
identity/update custody stays the existing trusted-host precondition, not a
new same-user-resistant or verified-exec requirement.

## Finite shared-host delta

Task234 owns the shared implementation and frozen pending-attempt packet.
This preparation changes neither. Its next ACP composition must bind:

1. A host monitor and actual Rust prepared owner; the sleep group owner and
   checked ACP Node are children of Rust, with ACP in sleep's group. The native
   entry is ACP's child. The driver-under-group-owner stand-in is different.
2. Actual PID/parent/group/start/launch identities before release, including
   every selected original caller and final invocation inputs above. A trace
   cannot mint actors or wait rights.
3. Kernel task containment before untrusted entry, counting threads, enclosing
   helpers and descendants and refusing migration/controller changes; exact
   permitted-executable bypass needs closure, not detection after effects.
4. Original-compatible transport and the combined canonical absent/disabled/
   managed-deny fixtures through actual ACP prepared paths, with separate
   software waits, adoptee rights, pidfd/cgroup settlement and fixture cleanup.
5. ACP's installed compatible Node identity and the selected Python inputs,
   independently of SDK22 evidence. Any complete original packet needs fresh
   source/plan/runtime/artifact bindings and separate exact execution authority.

The actual shared host/kernel/fixture admission and complete selected launch/
runtime bindings remain the external prerequisite. The original command stays
null and the ceiling stays0.100.0. Do not mutate234's frozen packet or infer an
activation from this source record, its earlier grants or operator request
lineage78e36a73.

## Reconstruction and mutation proof

The inspector reads files/linkage without executing the selected Python or any
original ACP/SDK/native. Source record construction reads only retained inputs.
The validator checks exact proposal content against those pinned inputs; it
is not runtime attestation and cannot grant admission. Mutation tests reject
missing/extra/reclassified callers, changed graph/native/source identities,
removed cautions/gates, forged sibling topology, weakened Python inputs and
invented execution/qualification. Owned fake metadata tests distinguish a
framework self install-name from a genuinely unresolved dependency.

```sh
effigy build:acp-admission-record --output docs/knowledge/operations/claude-agent-acp-admission-record.json
effigy check:acp-admission-record --record docs/knowledge/operations/claude-agent-acp-admission-record.json
effigy validate:acp-admission-record
```

Only newly affected Python format, selector-routing and documentation checks
cover this increment. No Rust/package/route production truth changed, and the
accepted three-case execution and older suites are not repeated.
