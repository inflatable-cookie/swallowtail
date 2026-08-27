# 226 Claude Code Headless Maximum Turns Evidence

Status: promoted
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.079 / 219

## Question

Which exact qualified `claude-code.headless` versions and positive values can
dispatch and natively enforce `--max-turns` through a closed adapter-local
selection, with explicit argv precedence over `CLAUDE_CODE_MAX_TURNS` and an
exact limit-reached stream/result disposition?

## Evidence Boundary

Research must use exact official package/native artifacts for the qualified
`2.1.220..=2.1.241` route window plus frozen official documentation. Mutable
current documentation is a lead and cannot backport support. No provider
prompt, login/account work, paid operation, host install/update, or ambient
configuration mutation is authorized.

The record must freeze support membership, parser domain, repeated-value and
environment precedence, counted-turn definition, loop enforcement,
limit-reached stream/result/usage/exit truth, omission, and current driver
mapping. It must classify every candidate row as deliver now, evidence-gated,
intentionally withheld, or not applicable.

## Promotion Gate

Cards 220-221 may run only for a non-empty exact deliver-now set whose positive
numeric domain, native enforcement, and explicit-argv precedence are proved
without live provider work. An empty set is an acceptable result.

## Method

Evidence was collected on 2026-08-27. No Claude Code installation, login,
credential capture, account inspection, provider request, prompt, or paid
operation was used. Host `claude` was not on `PATH` and was not installed,
replaced, or updated. No ambient configuration was written.

Every published official npm package in the qualified window
`2.1.220..=2.1.241` and its `@anthropic-ai/claude-code-darwin-arm64` platform
package were downloaded to disposable `/tmp` paths. Native executables were
inspected through `--version`, `--help`, deterministic local argv probes, and
extracted implementation source. All probes ran under `env -i` with only
`PATH`, `HOME`, and — where the probe required it — one disposable
`CLAUDE_CODE_MAX_TURNS`, in a throwaway `HOME` and working directory.

Two prompt-free probe terminals were used:

- `claude [flags] doctor` reaches full commander parsing and a local health
  action that sends no provider request. It separates *unknown option* from
  *accepted option* and exercises the `--max-turns` value parser.
- `claude [flags] -p` with `stdin` closed reaches the main print action, which
  resolves the effective turn limit before it rejects the empty prompt. It
  separates *argv wins* from *environment consulted* without any API call.

The route under study is `claude-code.headless`, driver
`swallowtail.claude-code.headless`, axis `claude-code.headless-stream-json`,
qualified window `2.1.220..=2.1.241`, behavior
`claude-code.headless.stream-json.v1`. This record does not amend
`claude-code.response-only` or `claude-agent.acp`.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference) | `--max-turns` print-mode description, error on limit, no limit by default | 2026-08-27 | SHA-256 `e4a827f85dcbde8fc9395d87fad9ebe1815896972e82222753ec14655f610eda` |
| [Claude agent loop](https://code.claude.com/docs/en/agent-sdk/agent-loop) | counted-turn definition, `error_max_turns`, result-field availability | 2026-08-27 | SHA-256 `6709f9077d36a01a828fa2095ea6f229f1c82095fc5c3f02ba3bebaa53563891` |
| [Claude Code environment variables](https://code.claude.com/docs/en/env-vars) | `CLAUDE_CODE_MAX_TURNS` equivalence and argv precedence | 2026-08-27 | SHA-256 `dc824141c7e6306e91df33ab9aff9ac58f58bd0f40468bda1479772d157521d3` |
| `@anthropic-ai/claude-code@2.1.220` wrapper tarball | window baseline identity | 2026-08-27 | SHA-256 `df33087481fcf5fe9b848b3f7ae7ee6bb1b893c327b0793f052987f9c5b4eee3`; npm shasum `29e7249f01f9602b78c2d5f3c2f1c8a11b2ebcb4` |
| `@anthropic-ai/claude-code@2.1.241` wrapper tarball | window ceiling identity; matches Research 202 | 2026-08-27 | SHA-256 `752252ff9a65431c356ce1ae54b7ded74a138aaa7b93148573d97ff541a2e7e6`; npm shasum `150077700180a6f915a486a34b4c34404e4aee59` |
| `@anthropic-ai/claude-code-darwin-arm64` `2.1.220..=2.1.241` | exact parser, precedence, loop, terminal, and exit evidence | 2026-08-27 | per-version digests in the support table below |
| `claude-code-2.1.241/headless-maximum-turns.json` | sanitized deterministic specimen corpus | 2026-08-27 | asserted in `claude_code_headless_identity.rs` |

Current documentation is a lead only. It states that `--max-turns` limits
agentic turns in print mode, exits with an error when the limit is reached, has
no default limit, counts tool-use round trips, produces result subtype
`error_max_turns` with no `result` field, and that `CLAUDE_CODE_MAX_TURNS` is
equivalent when argv is absent while explicit argv takes precedence. None of
that backports onto the qualified window. Every claim below rests on the exact
artifacts.

The wrapper npm packages carry no `--max-turns` string at either window
endpoint. They remain installer wrappers; all evidence is in the platform
native executable.

## Help Omission Is Deliberate, Not Absence Of Support

`--max-turns` appears in no `--help` output at any probed version. Official
`--help` digests reproduce Research 202 and Research 212 exactly:
`fcd5b45507c7c602d54d85a300eab288a8a3c6770c6def696ca19a3100725de4` at
`2.1.220` and `71ad650f59e08ae40ede14c534db4f49d8590ee5a4f92f6da2882d3a5560fea6`
at `2.1.241`. `--max-budget-usd` is advertised at both; `--max-turns` is not.

The extracted option declaration settles why. At every probed version the
option is registered and then explicitly hidden:

```js
.addOption(new wp("--max-turns <turns>",
  "Maximum number of agentic turns in non-interactive mode. This will early exit the conversation after the specified number of turns. (only works with --print)")
  .argParser(E8y).hideHelp())
```

Help omission is a `hideHelp()` call on a registered option, not missing
support. Research 212's rule still holds in the other direction: registration
and parser acceptance alone do not prove enforcement. Enforcement is proved
separately below.

The local probe confirms registration behaviourally. An unknown flag is
rejected, `--max-turns` is accepted:

| Probe | `2.1.220` | `2.1.241` |
| --- | --- | --- |
| `claude --nope 3 doctor` | exit 1, `error: unknown option '--nope'` | exit 1, `error: unknown option '--nope'` |
| `claude --max-turns 3 doctor` | exit 0, no stderr | exit 0, no stderr |

## Parser Domain

The registered `argParser` is shared with `--max-thinking-tokens` and performs
one numeric coercion with no positivity, integrality, or range check:

```js
function E8y(e){ let t = Ere(e); if (Number.isNaN(t)) throw new o6t("must be a number"); return t }
function Ere(e){
  let t = Number(e); if (!Number.isNaN(t)) return t;
  let r = String(e).trim();
  return r.length <= 32 && kIu.test(r) ? parseInt(r.replace(AIu, ""), 10) : NaN;
}
// kIu = /^[+-]?\d{1,3}([_,   ])\d{3}(?:\1\d{3})*$/
```

Observed dispositions are identical at every probed version:

| Input | Parser | Notes |
| --- | --- | --- |
| `1`, `3` | accepted | ordinary positive integers |
| `0` | accepted | **not rejected**; see enforcement |
| `-1`, `-0` | accepted | negative and signed zero pass the parser |
| `+3` | accepted | `Number("+3") === 3` |
| `" 3"`, `"3 "` | accepted | `Number` trims surrounding whitespace |
| `03` | accepted | leading zero coerces to `3` |
| `3.5` | accepted | fractional values pass the parser |
| `1e3` | accepted | exponent form coerces to `1000` |
| `0x3` | accepted | hexadecimal coerces to `3` |
| `""` (empty string) | accepted | `Number("") === 0` |
| `1,000`, `1_000`, `1 000` | accepted | grouped-digit fallback via `Ere` |
| `Infinity` | accepted | never exceeded by any turn count |
| `NaN` | rejected | `error: option '--max-turns <turns>' argument 'NaN' is invalid. must be a number` |
| `abc`, `3abc`, `doctor` | rejected | same `must be a number` diagnostic, exit 1 |
| flag with no value | rejected | `error: option '--max-turns <turns>' argument missing`, exit 1 |

`--max-turns=4` and `--max-turns 4` are both accepted; there is no alias and no
short form. The `argParser` ignores commander's previous-value argument and
returns the coerced value, so a repeated flag is last-wins rather than
accumulating.

The parser therefore admits a much wider domain than the documented "positive"
one. Any adapter-local binding must supply positivity and integrality itself;
the native parser will not.

## Environment Precedence

Resolution is one function, identical at every probed version:

```js
function UOu(e){
  if (e !== void 0) return e;                                  // explicit argv wins
  let t = process.env.CLAUDE_CODE_MAX_TURNS?.trim();
  if (!t) return;                                              // absent or blank => no cap
  let r = Ere(t);
  if (!Number.isInteger(r) || r <= 0)
    throw Error(`CLAUDE_CODE_MAX_TURNS must be a positive integer; got "${t}"`);
  return r;
}
```

The main print action calls it before any prompt handling — `c = UOu(t.maxTurns)`
inside a `try` whose `catch` renders the error and returns — and the resolved
value is threaded straight into the run as `submitMessage({ …, maxTurns: c, … })`
and then into the agent loop.

The `-p` probe confirms every branch at both window endpoints:

| `CLAUDE_CODE_MAX_TURNS` | argv | Result |
| --- | --- | --- |
| unset | absent | proceeds; no limit |
| `bogus` | absent | exit 1, `CLAUDE_CODE_MAX_TURNS must be a positive integer; got "bogus"` |
| `0` | absent | exit 1, same positive-integer error |
| `-2` | absent | exit 1, same positive-integer error |
| `3.5` | absent | exit 1, same positive-integer error |
| `3` | absent | proceeds; env cap applies |
| `"  3  "` | absent | proceeds; value is trimmed before parsing |
| `""` | absent | proceeds; blank is treated as absent |
| `bogus` | `--max-turns 3` | proceeds; **environment never read** |
| `0` | `--max-turns 3` | proceeds; environment never read |
| `bogus` | `--max-turns 0` | proceeds; environment never read |
| `bogus` | `--max-turns -1` | proceeds; environment never read |

Explicit argv precedence is unconditional: the environment branch is
unreachable whenever the flag is present, including for argv values the
environment itself would reject. Preparation therefore never needs to inspect,
scrub, or rewrite the operator-approved environment to make an explicit
selection authoritative.

The converse is equally exact and matters for omission. With no `--max-turns`
argument the ambient value is authoritative: a valid ambient positive integer
silently caps the run, and an invalid ambient value aborts the process at
startup with exit 1 before any stream is produced. That is the route's current
truth today. Omission is not "unlimited"; it is "whatever the approved
environment already says".

No settings key competes. The two `"maxTurns"` string literals in the artifact
belong to a plugin/agent manifest key list and an evals config key list. There
is no session-level settings source in the resolution path, and
`--setting-sources user,project,local` cannot introduce one.

## Counted Turn And Loop Enforcement

The agent loop initialises `turnCount: 1` and, on the path taken after tool
results have been collected, computes the next turn and the limit in one
expression:

```js
let gr = Te + 1, $r = u && gr > u ? u : void 0;
…
if ($r !== void 0)
  return yield ac({ type: "max_turns_reached", maxTurns: $r, turnCount: gr }, m),
         yot(J, l),
         { reason: "max_turns", turnCount: gr };
```

The same expression appears at every probed version, differing only in
minified identifiers. A second, equivalent guard covers the stop-hook
continuation path, and a third emits the same attachment when the loop aborts
with tools in flight.

Three facts follow.

- **A counted turn is a tool-use round trip.** The check sits after tool
  execution and tool-result attachment, on the branch that would otherwise
  recurse into another provider request. A final text-only response does not
  reach it. This matches the documentation lead: `max_turns` counts tool-use
  turns only.
- **Enforcement is native and unconditional for positive values.** Nothing
  between resolution and the guard can raise, reset, or discard the value. It
  is not clamped, not rounded, and not renegotiated by the model, the tools,
  hooks, provider configuration, or retries. On this route there is no resume
  or session state to reintroduce one, because `--no-session-persistence` is
  always sent.
- **The guard is a truthiness test, not a presence test.** `u && …` means a
  resolved `0` disables enforcement entirely. A negative value is truthy and
  fires at the first opportunity (`gr = 2 > -1`), and a fractional value fires
  at the first integer above it while reporting the fractional bound verbatim.
  These are degenerate, not useful, and they are exactly the values a closed
  positive type must exclude before preparation.

The `max_turns_reached` attachment is consumed inside the SDK message
transform (`{ turnCount, maxTurns }` is captured and the message is dropped
with `continue`). It is not emitted as a separate stream-JSON line.

## Limit-Reached Terminal Truth

The captured attachment produces the terminal result:

```js
… ? { ...H6({ startedAt: xt,
        common: { ...Jt, is_error: !0, num_turns: ot.turnCount },
        variant: { subtype: "error_max_turns",
                   errors: [`Reached maximum number of turns (${ot.maxTurns})`] } }), ...yt }
```

The result schema separates the success and error variants. The error variant
carries `type`, `subtype`, `duration_ms`, `duration_api_ms`, `is_error`,
`num_turns`, `stop_reason` (nullable), `total_cost_usd`, `usage`, `modelUsage`,
`permission_denials`, and `errors`. It has **no `result` field**; final text is
only present on `success`.

Under this route's `--output-format stream-json --verbose`, the headless print
switch takes `case "stream-json": break` and prints nothing extra: the result
message itself is the only carrier. The plain-text branch would print
`Error: Reached max turns (N)`, which this route never selects.

The process exit is one expression, identical at every probed version:

```js
Fd($e?.type === "result" && $e?.is_error || De ? 1 : 0)
```

So a native limit-reached run is **both** an `error_max_turns` result on the
stream **and** process exit `1`. It is not a successful process carrying an
error subtype, and it is not a bare nonzero exit with no result.

`error_max_turns` is one of four error subtypes alongside
`error_during_execution`, `error_max_budget_usd`, and
`error_max_structured_output_retries`. Native bound reached is distinguishable
from provider execution failure only by that subtype string; the exit code and
`is_error` flag are shared.

## Current Driver Mapping

The route already handles this terminal shape without change.

`claude_code_events.rs` requires `subtype`, `is_error`, and `usage` on every
result. Any subtype other than `success`, or `is_error: true`, records
`swallowtail.claude_code.headless.provider_failed` with a
Provider/Unknown/Unknown classification, leaves `final_output` unset, still
emits the usage observation, and marks the terminal as seen. `usage` is present
on the error variant, so decoding succeeds.

`claude_code_events/terminal.rs` then orders exit interpretation so the provider
diagnostic wins over the generic nonzero-exit mapping. Exit `1` with a recorded
provider failure yields `TerminalStatus::ProviderFailed` with
`FailureOrigin::Provider`, not the harness `process_failed` diagnostic, and not
`Completed`. Cleanup stays `CleanupOutcome::Clean` and the existing joined
task/process teardown is untouched.

Two consequences are worth stating plainly. A native limit-reached run is
already reported as a provider failure with no output, which is correct and
must not become completion. And the current diagnostic does not distinguish
`error_max_turns` from `error_during_execution`; this record does not admit a
new diagnostic code, because doing so would widen terminal mapping beyond the
lane's boundary. Requested, prepared, dispatched, parser-accepted, enforced,
reached, and observed state stay separate in docs and tests instead.

Unknown stream types remain tolerated as unknown activity, so the consumed
`max_turns_reached` attachment could not disturb decoding even if a future
version emitted it.

## Support Table

Every published version in the qualified window was probed on its own native
darwin-arm64 executable. `2.1.230` was never published to npm, so no artifact
exists for it; the semantic range still contains it but it can never be
observed on a host.

All 21 published points returned identical results on every check:

- `--help` never advertises `--max-turns`
- the hidden `--max-turns <turns>` option declaration with its shared numeric
  `argParser` and `hideHelp()` is present
- the `CLAUDE_CODE_MAX_TURNS` resolver with its unconditional
  `if (argv !== undefined) return argv` short-circuit is present, as is its
  `must be a positive integer` diagnostic
- the loop guard `next = turnCount + 1; limit = max && next > max ? max : undefined`
  is present, differing only in minified identifiers
- the `subtype: "error_max_turns"` result construction and its
  `Reached maximum number of turns (` message are present
- the exit expression `result && is_error ? 1 : 0` is present
- `--max-turns 3`, `0`, `-1`, and `3.5` are accepted by the parser (exit `0`);
  `--max-turns abc` is rejected (exit `1`); an unknown flag is rejected

| Version | darwin-arm64 native binary SHA-256 |
| --- | --- |
| `2.1.220` | `8addc857f3fe64d5a0368af9ee50321b50afb4a6918ba3ef018ab84f5dbbe081` |
| `2.1.221` | `7a181f36ed0fc4fbac6cee4ecf2b615eff93d8b434221fff5d7c878dc5ebf380` |
| `2.1.222` | `c66a6cc6fa2e8145bb1a6e77831f2caf4b83690ff04650500dfa6e2c05ca997c` |
| `2.1.223` | `fcbe0b8d47570c501302dd1ad31cc26ac2810f022c45fa253936a6961dee32bf` |
| `2.1.224` | `391df9d2ab04e4cf32199335720ac7715a582e91eaecfd4d2198a16f57ea59b3` |
| `2.1.225` | `08d6e85dd2b80883bb8da93cbeae3dc79b4704d6b84a05d614bf1ff4a5155b69` |
| `2.1.226` | `013a1cf17df5ff1dcc189d5d6fd3fdd5f097ddc3cd41aa9992e99805574febbe` |
| `2.1.227` | `7432511ba3be818e01f23f6eef8630d214a8b618451e188c3c7d61a987eef6c7` |
| `2.1.228` | `43484b1352cef03a08346f36ef0437755b1aad646ab9313ce187857b794b7247` |
| `2.1.229` | `d732f0ba0a539c58c2ffcaef06ed03b4e523726f0cb6cc27b3a5b7e7ae0a7a21` |
| `2.1.231` | `ba790279cab6ef77b713864d4bf5f764fcea87d3a3eb7591a41f741e45212b5c` |
| `2.1.232` | `7b39c1588df919d001dea3ffd5651adb682f2451b5a0e18d42d4233296b53cc7` |
| `2.1.233` | `bc466b6cde63edafc773f471a1fb98787fabb31f52240c8616ce7e1f587b212d` |
| `2.1.234` | `08d8700313697cbe730a25420c908a299ce52d56f0eb2cf4fac94cab5109bc57` |
| `2.1.235` | `83b8f806f6f2eea316cfe246628e6c23374711d868f1fd0409db551b877b7748` |
| `2.1.236` | `6bc4ba992d2786cbf0237c4453ca53c1fdf0c3b3d83ffa0025c0d8190ed27848` |
| `2.1.237` | `338901351d4ff17495738c67fc3e12a32c1b506738ac5e012eb782d3d8b5be43` |
| `2.1.238` | `1c196c456373b57818ae87df84aecee96cb659448c0d6a6bbb401ac5758431b2` |
| `2.1.239` | `2b4f7aafdaa65bcc2335f56a4b276317837203f2c5587b1f2a17ca78ad14e36f` |
| `2.1.240` | `8917e01c99ea0ce6ed887a1729a4cda693c758fe542747be71756987b145c772` |
| `2.1.241` | `1495eb7c42d3b4451f5f1cd38b6d498d22a4a38c802bc2be5c1cf1795e64820d` |

## Omission

Omission emits no `--max-turns` argument and leaves the exact current command
byte-identical:

```text
claude -p --input-format text --output-format stream-json --verbose
  --no-session-persistence --model <selected> [--effort <selected>]
  --permission-mode plan --tools Read,Glob,Grep
  --setting-sources user,project,local --mcp-config {"mcpServers":{}}
  --strict-mcp-config
```

The approved environment is passed through unchanged. No unlimited-execution
claim follows: as proved above, an ambient `CLAUDE_CODE_MAX_TURNS` remains
authoritative under omission and an invalid one aborts startup. Omission
preserves current behavior exactly, including that ambiguity.

## Claim Strength

| Claim | Strength at the exact evidence boundary |
| --- | --- |
| help advertises `--max-turns` | not observed at any probed version; `hideHelp()` in the declaration explains it |
| option is registered and parsed | observed at every published version in `2.1.220..=2.1.241` |
| parser rejects non-numeric values | observed at every probed version |
| parser accepts zero, negatives, fractions, `Infinity`, grouped digits | observed at every probed version |
| explicit argv overrides `CLAUDE_CODE_MAX_TURNS` | proved by source and by runtime probe at both endpoints |
| ambient environment applies under omission | proved by runtime probe at both endpoints |
| counted turn is a tool-use round trip | proved by loop position in exact source |
| positive value is natively enforced | proved by exact loop guard at every probed version |
| selected positive value can be ignored, clamped, or shadowed | not observed; no settings, hook, resume, or provider path reaches it |
| resolved `0` disables enforcement | proved by the truthiness guard; excluded by a positive-only type |
| limit reached emits `error_max_turns` with `is_error`, `num_turns`, `usage`, `errors`, no `result` | proved by exact result construction and schema |
| limit reached exits `1` | proved by the exact exit expression at every probed version |
| current decoder maps it to `ProviderFailed` with Provider origin | proved by route source |
| effective turn count for any given prompt | unproved; requires live provider work and is not claimed |
| `error_max_turns` distinguishable from other provider failures in Swallowtail terminal state | not admitted; terminal mapping stays unchanged |

## Deliver-Now Table

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| `--max-turns <positive integer>` at every published `2.1.220..=2.1.241` version | registered hidden option, native loop guard, argv precedence, `error_max_turns` result, exit `1` | **deliver now** |
| omission | current argv preserved byte-for-byte; ambient environment authoritative | **deliver now**, unchanged, with no unlimited claim |
| `--max-turns 0` | parser accepts; truthiness guard disables enforcement | rejected; a selected bound must never be inert |
| `--max-turns` negative, `-0`, fractional, `Infinity`, `1e3`, `0x3`, empty string, grouped digits | parser accepts; degenerate or ambiguous enforcement | rejected; closed positive-integer type only |
| raw string or raw number escape hatch | parser domain is far wider than the documented one | rejected |
| `CLAUDE_CODE_MAX_TURNS` write, scrub, or inspection | argv precedence is unconditional; no need | rejected; out of scope |
| portable maximum-turn, agent-budget, tool-call, cost, or wall-time control | turns are tool-use round trips only | rejected; not a portable generation control |
| `--max-budget-usd`, `--task-budget`, `--autocompact`, `--max-thinking-tokens` | advertised or hidden siblings, not this lane | not applicable |
| `2.1.242` and later `UnverifiedNewer` points | no artifact probed | evidence-gated; selection must reject them |
| `2.1.230` | inside the semantic window but never published to npm; no artifact exists | evidence-gated; a `Qualified` assessment is not evidence, so the selection must reject it too |
| new terminal diagnostic for `error_max_turns` | mapping already fail-closed as provider failure | intentionally withheld; do not widen terminal mapping |
| `claude-code.response-only`, `claude-agent.acp` | separate routes and axes | not applicable |

Deliver-now rows: **two** — one closed positive-integer selection across the
whole published qualified window, and unchanged omission.

## Decision

Card 219 is complete with a non-empty exact set. The promotion gate is met:
the numeric domain is closed to positive integers by the adapter because the
native parser will not close it, native loop enforcement is proved from exact
artifacts at every published qualified version, and explicit-argv precedence
over the ambient environment equivalent is proved by both source and runtime
probe without any provider work.

Cards 220 and 221 may run. The admitted binding is adapter-local and closed:
one positive-integer Claude Code selection carried immutably from prepared
input through prepared evidence into the low-level driver, and dispatched as
exactly one canonical `--max-turns <N>`. Omission keeps the existing command
and environment handoff exactly. Terminal mapping, route authority, tools,
configuration, retention, and cleanup do not widen.

The version gate must be the exact set of points probed above, not the route's
qualified window. The window is weaker evidence in two distinct ways: its claim
permits later stable points as `UnverifiedNewer`, and its segment is a semantic
range that contains `2.1.230`, a version that was never published and for which
no artifact can be obtained. A `Qualified` assessment for `2.1.230` is a
statement about the range, not about any observed binary, so it must not admit
this feature.

Prepared execution must be the only way a bound reaches a process. Neither
`PreflightPlan` nor `StructuredRunRequest` records a maximum-turn bound, so an
extracted driver that carried one could be handed another prepared run's plan
and silently dispatch a value that plan never selected — including onto a run
that deliberately omitted the selection. There is no immutable execution input
to compare against, so agreement has to come from construction: the bound and
its `(plan, request)` pair may only ever be brought together in one place.

## Sources

- npm `@anthropic-ai/claude-code` `2.1.220` through `2.1.241`
- official `@anthropic-ai/claude-code-darwin-arm64` `2.1.220` through `2.1.241`
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
- [Claude agent loop](https://code.claude.com/docs/en/agent-sdk/agent-loop)
- [Claude Code environment variables](https://code.claude.com/docs/en/env-vars)
- [Research 202 Claude Code 2.1.241 Identity](./202-claude-code-2-1-241-identity.md)
- [Research 212 Claude Code Headless Ultracode](./212-claude-code-headless-ultracode-evidence.md)
- `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-maximum-turns.json`
