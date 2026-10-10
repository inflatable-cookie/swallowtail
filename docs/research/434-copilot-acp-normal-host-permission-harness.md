# Research 434: Copilot ACP Normal-Host Permission Harness Preparation

This is a fake-only preparation for one exact Darwin ARM64 Copilot CLI ACP
permission observation at `1.0.93`. It follows Tom's normal-host direction in
[Contract 023](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#copilot-normal-host-proof-direction).
The preparation ran no vendor artifact, original ACP session, prompt, login,
or provider request. The original-enabled and preparation-execution flags in
the reviewed plan are both `false`; no prompt or invocation allowance was
consumed here. The existing `1.0.80` claim and released consumers are
unchanged.

## Exact disabled plan

The plan is
[`plan.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof/plan.json)
with SHA-256
`469cacfe46125141c6142b7b8b9418805da00ae550fb6d9c672b994f439dd664`. The
runner is
[`copilot-acp-host-permission-proof.py`](../../scripts/copilot-acp-host-permission-proof.py)
with SHA-256
`a7b1218d8ffc99e57c5d771be8ba022b69886afc453523115444432e1f323bdc`. Its
artifact inventory is the retained Research 404 inventory with SHA-256
`2d122117ccbb52dd547a783117ea3b1699df8e15357bca86a4d27a65416c8b0f`.

The independently identified wrapper is `@github/copilot@1.0.93`: archive
SHA-256 `a8e704fb6874364af1b268aed2170bb597e0ca8086f3182b8fe5cb86ca3e43e1`,
manifest SHA-256 `5f29c2061d18be20a8cb1677161359253a85bb49a7b4ea1e6c11db8ae1d9b64b`,
and `package/npm-loader.js` SHA-256
`0ea824a86be5757533fdb092eff7050871bd7a711a46babde0ffe0e44ac5ad88`. It is
an identity cross-check and will not be executed by this plan. The selected
native payload is `@github/copilot-darwin-arm64@1.0.93`: archive SHA-256
`f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb`, package
manifest SHA-256
`44de35dc12ce1b582678cf565d9f8ddddb786b00dd083373948d6e453e1eb729`, and
extracted `package/copilot` SHA-256
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`. The
validator cross-checks package/version labels, official tarball URLs and SRI,
file inventories, executable mode and digest, Darwin/ARM64 labels, and the
direct launch arguments `--model auto --acp --stdio`. Preparation did not
stage or run these artifacts.

An unauthenticated read of official npm latest metadata on 2026-10-10
`06:28:17Z` observed `1.0.95` for both the wrapper and native package ([wrapper
metadata](https://registry.npmjs.org/@github/copilot), [native package
metadata](https://registry.npmjs.org/@github/copilot-darwin-arm64)). This is
currentness movement only. It does not add a target, invocation or prompt
allowance to this exact `1.0.93` plan.

## Normal-host boundary and operation

The later proof trusts the exact reviewed CLI under the normal macOS account,
including its ordinary access to the existing `betterthanclay` login, vendor
children, managed logs/state and networking. The harness does not read or copy
credentials, Keychain/config data or auth responses; it does not query account
identity or create a login. Auto is passed explicitly. If ACP exposes a
structured model identity it may be recorded as a bounded observation;
otherwise the effective model remains unobserved. Vendor request counts and
all internal state access are not mediated. The scratch directory is not an
OS sandbox, default-deny network boundary or guarantee against ambient host
access.

The proposed single prompt asks the CLI to overwrite only
`permission-sentinel.txt` in a fresh task-owned directory, from the exact
before bytes to the fixed after bytes. The runner requires the matching write
permission request, replies with ACP `cancelled`, and requires the prompt to
end cancelled while the sentinel remains byte-identical. It never approves,
resends or falls back. Missing permission is not proof. A changed sentinel,
wrong path/content/session, duplicate request or tool update without
permission fails the observation. The later attempt is limited to one
invocation, one prompt within the existing three-prompt shared ceiling, and a
60-second operation ceiling with bounded cleanup. All historical consumed
records remain untouched.

Before any later original start, the plan specifies exclusive mode-0600
attempt and prompt-slot files beneath the private mode-0700 task record root.
The attempt binds plan and runner digests, wrapper/native archive and
executable digests, argv, environment policy, account reference, action and
budgets. `O_EXCL`/`O_NOFOLLOW` creation and file/directory `fsync` consume the
invocation before launch and prompt slot before send. Existing records refuse
replay. Preparation created no such original record; fake ledgers are inside
automatically retired task scratch only.

## Fake results

The successful fake path exercised ACP initialize, session creation, one
prompt, matching permission request, cancel response and cancelled prompt
result. It created and read a synthetic Copilot config, made home/XDG/temp and
Copilot state directories, wrote a runtime state file and log, and left the
sentinel unchanged without an unapproved effect marker. Fake fork, ordinary
spawn and `setsid` children were waited by exact child handles and completed.
The fake record reports root exit, process-group observation and stdio EOF
separately. Arbitrary vendor descendant cleanup remains
`unknown-for-arbitrary-vendor-descendants`; a root or process-group exit alone
does not establish that every descendant joined.

The negative fixtures classify spontaneous effect, missing permission, wrong
action, duplicate permission, malformed and oversized frames, early EOF,
initialize/session errors, wrong version, unpermitted tool update, two timeout
positions, and crashes before or after the prompt send. EOF remains
`unknown-eof`; the consumed prompt-slot record plus a fake-only receipt marker
distinguishes crash-before-send from crash-after-send without a causal claim.
The identity-query helper's deterministic fixtures returned success, bounded
timeout and failure with injected short deadlines; the production default
caps the combined host query at two seconds. No host identity query or
quiet-host wait was needed. Attempt and prompt replay, a disabled original
ledger write, false permission-action records, and unreviewed raw fields were
all refused. Only closed failure/marker categories, byte counts and protocol
outcomes are persisted; raw ACP frames, stderr, secrets, auth responses, paths
and diagnostic hashes are not.

These fakes do not prove Copilot `1.0.93` startup, existing login acceptance,
Auto resolution, provider access, actual permission behavior, cancellation
semantics or route compatibility. A later original result must be reviewed
against this exact plan and its limits. No original or reviewer spend is part
of this preparation.

The `check:copilot-acp-host-permission-proof` selector passed plan, schema,
inventory and runner identity validation. The
`validate:copilot-acp-host-permission-proof` selector passed the fake lifecycle,
protocol, ledger, diagnostics and identity fixture checks above. Neither
selector invokes the original CLI.
