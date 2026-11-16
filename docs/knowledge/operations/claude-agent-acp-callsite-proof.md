# Guarded ACP source callsites

The [callsite record](claude-agent-acp-source-flow.json) binds actual invocation
expressions, argument expressions, imported and promisified aliases, enclosing
definitions, direct callers and dataflow excerpts to UTF-8 byte spans and file
SHA256s. It covers the selected ACP, Agent SDK and Zod source in all eight
captured graphs. It replaces import-parent-only reasoning for these callers.
It is source evidence, not execution or native qualification.

## Launch boundaries

| Caller | Source-derived invocation | Current boundary |
| --- | --- | --- |
| ACP index | Native resolver; forwarded CLI args; inherited cwd/env; inherited stdio; detached omitted. | Only under `argv.includes("--cli")`. Unreachable when the admitted original argv excludes that flag. The launch must establish that premise. |
| Auth status | Native resolver; optional `--settings JSON` before `auth status --json`; session cwd; process env overlaid by session env; buffered pipes; timeout5000; detached omitted. | Reached by initialize/session/prompt auth probing unless its conditional gateway guard returns. The promisified facade still checks native custody and startup env before spawn. This is a separate native invocation. |
| Auth logout | Native resolver; `auth logout`; inherited cwd/env; buffered pipes; no explicit timeout; detached omitted. | Explicit logout dispatch. Unselected for permission canonical proof, but not globally unreachable. No credentials mutation is authorized here. |
| Session registry | `ps -o pid=,lstart= -p PID_CSV`; inherited cwd; env plus `LC_ALL=C,TZ=UTC`; buffered pipes; timeout1000; detached omitted. | Nonempty PID query on non-Windows. Existing caller-specific facade checks exact argument shape, decimal CSV length<=4096, timeout, held ps identity and startup env before effect. |
| SDK transport | Native path, or interpreter for script extension; executableArgs plus stream-json base flags and option-derived flags; ACP session cwd/env; three pipes; windowsHide; detached omitted. | Default spawn reaches native path/hash/env mediation. ACP options, SDK option branches and extraArgs remain in the byte-bound excerpts. Actual values and entry release need original admission. Standalone SDK sidecar hooks are not ACP hooks. |
| SDK utilities | Git worktree enumeration; managed `plutil`/registry reads; Darwin `security` credential staging. | Actual callsites and callers are retained. These commands match neither held native nor caller-specific ps, so the facade refuses before spawn. Their caught-failure/fallback semantics still need compatibility proof. |
| Older SDK Windows resolver | `SYSTEMROOT/System32/where.exe`, requested name, inherited env/cwd, pipe, timeout5000. | Present in SDK0.3.318/319; Windows platform branch, outside selected Darwin/Linux. It would also fail the current facade command check. |

The record retains source expressions where a value depends on session input,
settings, environment, platform or SDK options. It does not invent a final
argv/env/cwd from those expressions. Omitted detached means the Node API default,
not a native-descendant containment claim. Native resolution remains distinct:
ACP checks the executable override/bundled path before SDK-rooted platform
resolution; the captured module facade only resolves admitted parent/edge pairs.

## A concrete startup incompatibility

Every selected SDK version has **two module-scope calls** to
`globalThis.process?.getBuiltinModule?.("async_hooks")`. Neither has a surrounding
catch. The production bootstrap in `captured_source.rs` defines a present,
nonconfigurable `getBuiltinModule` function that throws before installing the
captured loader. Optional chaining calls a present function: it does not take
the SDK's missing-API fallback. Therefore SDK evaluation fails under this exact
bootstrap before a query can start.

`guardStartup` and the SDK `raw` entries bind both sides of this mismatch.
The owned fixture demonstrates the optional-call semantics with a miniature
first-party object; it never imports SDK code. A compatible bootstrap adaptation
needs the shared bootstrap owner's planning disposition. This record changes
no startup policy and supplies no original execution authority.

## Dynamic and ambient edges

- The SDK's Lodash util detector contains real process-alias `binding("util")`
  sites. Its CommonJS detector requires an undeclared module alias to be an
  object (`n_` in318, `o_` in319, `b_` in321/322), plus an exports identity check. Under
  the standard captured Node ESM global premise, that branch is false. This is
  conditional source unreachability, not a general ban on process bindings.
- No explicit `_linkedBinding`, `execve` or `dlopen` member appears in the
  restored selected subset. Arbitrary computed properties, injected globals,
  constructor paths and opaque native internals are not ruled out by a named
  member census. ACP protocol SDK/diff retain their frozen static inventories;
  this record makes no new semantic-closure claim for those auxiliary packages.
- SDK/external Zod eval probes catch `Function` denial. Object-schema fastpass
  generation depends on their probe succeeding. `Doc.compile`, explicit Zod
  compilation and SDK Ajv schema compilation still generate strings. The Ajv
  requested-schema/elicitation path has no general interpreter fallback proved
  here. Selected compatibility remains a named runtime observation; keep
  `--disallow-code-generation-from-strings`.
- `Function.prototype` call/bind/toString references are not constructors.
  Lodash's `Function("return this")` fallback short-circuits under normal Node
  `global.Object===Object`; that startup premise is explicit. Constructor
  aliases and their actual calls remain in the syntax record.
- Captured resolution has no worker_threads/vm allowance. Missing import edges
  refuse before loading. `--no-addons` remains required. None of this closes
  native workers, descendant exec, dynamic images or permitted-image argv bypass.

The genuine original gates remain: compatible startup, exact selected invocation
and pre-entry admission, opaque native closure, Python/non-system runtime custody,
actual sibling host/kernel/transport/canonical fixtures, and separate original
execution authority. The [shared composition](claude-agent-acp-admission-preparation.md#finite-shared-host-delta)
stays with the SDK continuation; its driver-under-group-owner stand-in earns no
ACP sibling proof. `originalCommand` stays null and the ceiling stays0.100.0.

## Restored inputs and reproduction

The [restoration receipt](claude-agent-acp-restored-inputs.json) records13 exact
frozen credential-free source tarballs, both accepted digest algorithms, observed
sizes, fetch origin, limits and selected-file counts. Registry/artifact records
already bound their URLs and hashes; no fresh metadata or latest was accepted.
No source tarball had a separately frozen byte-size field: both verified hashes
bind the restored bytes, and observed sizes are recorded without inventing an
older size assertion. Unsafe paths, links, duplicates, special members, identity
drift or a missing selected file refuse before analysis.

Three deterministic compressed input bundles retain the original verified
tarballs. Each is below8MiB; their total is below32MiB. They are delivered through
Nucleus proofFiles, not committed as vendor trees. The source record reads those
bundles directly, validates their exact entry set and re-verifies every archive
and selected file. It needs no extraction or vendor execution. Use the three
retained bundle files as repeated `--bundle FILE` arguments, or an owned directory
of the13 original `.tgz` files as `--inputs DIR`:

```sh
effigy check:acp-source-flow --record docs/knowledge/operations/claude-agent-acp-source-flow.json --inputs DIR
effigy compile:acp-source-flow
effigy validate:acp-source-flow
effigy check:python-format scripts/claude-acp-source-flow.py scripts/tests/claude-acp-source-flow.py
effigy validate:oxide-routing
effigy qa:docs
effigy skill run northstar/retired-concepts
```

The parser uses installed Node's embedded Acorn to parse strings. Its own
bootstrap is tooling, not vendor execution or evidence that vendor raw bindings
are safe. It records parser identity and refuses unclassified child-process
namespace imports or malformed syntax. Exact-source reconstruction plus owned
archive, alias, mutation and startup counterexamples cover this increment.
No production Rust, shared snapshots, consumed runner/plan bytes or public route
claims change. Prior execution results remain scoped to their published heads.

Tom's preserved Paseo handoff on2026-11-16 retains the old branch/PR and evidence
as history. The continuation grants source/fake work; spent component, namespace,
fixture and diagnostic grants stay spent. Source restoration is data recovery,
not an install, live attempt or qualification.
