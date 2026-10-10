# Copilot CLI ACP artifact hops through 1.0.95

This ledger freezes the official npm wrapper and Darwin ARM64 package for the
qualified baseline `1.0.80` and every published stable through the current
`1.0.95`. It records registry publication metadata, tarball SHA-256/SRI,
complete sorted file trees, per-hop file deltas, and a conservative selected
surface classification. The metadata observation is
`2026-10-10T13:59:23Z`; `1.0.96-2` was the prerelease at that observation.

The previous frozen `1.0.80`, `1.0.81`, and `1.0.93` trees are reused without
re-downloading their tarballs. Every other stable package was downloaded from
its exact `registry.npmjs.org` tarball URL. Archive SHA-256, npm SHA-1 and
SHA-512 integrity, package entry count, unpacked byte count, and every member's
path, mode, size, and SHA-256 were checked. No package was installed or run.

The wrapper has four files at every point. `npm-loader.js` is byte-identical
throughout (`0ea824a86be5757533fdb092eff7050871bd7a711a46babde0ffe0e44ac5ad88`);
the root `package.json` changes on each hop. The native package has 241 files
at `1.0.80`, 225 at `1.0.81`, 224 at `1.0.82`, 228 at `1.0.83`, 232 at
`1.0.84`, then four files from `1.0.85` onward. The `1.0.84` to `1.0.85`
repackaging removes 228 files and leaves a changed opaque `copilot` executable.
That executable changes on every later stable hop through `1.0.95`.

Each hop's exact added, removed, changed, and identical path sets are in
`artifact-hop-ledger.json`. The wrapper loader remains identical, but each
native hop changes runtime files. Changed application code and native
executables are classified as unresolved for selected ACP behavior. The
post-`1.0.84` package exposes no source file for the changed monolithic
executable. These are evidence stops, not compatible-extension findings.

The production claim remains exactly `1.0.80 QualifiedOnly`. Research 439's
accepted cancellation evidence remains limited to its exact `1.0.93` attempt;
the direct-native launch used `--model auto --acp --stdio`, unlike production
`--acp --stdio`, and its attributed action did not match the sentinel edit.
Nothing here transfers permission, edit, or lifecycle behavior to another
version or closes that launch-alignment gate.

The private test build captures the exact approved execution host, executable
reference, and environment reference before discovery. It hashes the approved
executable bytes before discovery and checks the same path again at the prepared
process-service boundary before forwarding the unchanged `--acp --stdio`
request. Exact safe host, executable, and environment reference identifiers
and their hashes are written to the private mode-0600 consumed records before
their respective effects. A mode-0400 copy in a mode-0700 scratch directory
has no planned writers and is hash-checked after cleanup; the runner launches
the approved path, not that copy. The host still opens the executable by path
after the process-service check, so a replacement in the check-to-open
interval remains an accepted trusted-principal boundary. The private binding
payload carries only the host home path; the launcher forwards that as `HOME`
so the installed CLI uses its existing host-owned login and configured
default. It does not read or serialize credential or configuration values.

The actual prepared-facade fake runner uses one absolute monotonic budget: it
stops work at 57 seconds, requests prompt cancellation, then joins the prompt
and protocol tasks, stops the owned child, and releases the scratch resource
inside the remaining three seconds. A hanging fake process exercises that
path. The driver's public turn-deadline rejection is unchanged. The same
exclusive, fsynced record writer is exercised for the attempt-4 and prompt-3
records in fresh scratch before prepared open and prompt. The original effect
gate remains disabled, writes no host user-data records, and starts no original
in this preparation revision. The Python self-check record is generated on
each self-test run in scratch, not stored as a checked-in pass attestation. The
Python self-check reports only plan validation, its permanently disabled
launcher gate, and its own record round trip; it makes no prepared-driver
behavior claim.

The Rust dispatch checks the persisted plan and exact binding digest before it
resolves host inputs or opens a record. The private fake branch uses the same
budgeted runner with an in-memory continuation and a fresh `LocalProcessHost`
that launches only the task-owned Python ACP fake. The fake child confirms the
attempt record exists before version discovery and the prompt record exists
before the prompt is sent. Tests inspect its stop and exit trace, joined task
and released-resource outcomes, measured task-tree hashes, and the final
secret-free result. They reject missing or mismatched action metadata and
measure an injected sentinel write as an effect. No continuation or host values
are persisted by the tests, and the committed plan remains disabled.

The frozen command template is
`effigy observe:copilot-acp-private-assessment <binding-payload.json>`. The
payload is a private mode-0600 JSON file no larger than 8 KiB with exactly the
keys in `private-original-binding.schema.json`: reviewed head, reviewed plan
SHA-256, command template, safe execution-host/executable/environment/resource
references, and the current host home path. `binding_sha256` in the reviewed
plan is SHA-256 over compact sorted-key JSON containing only the five safe
host-binding fields, excluding the reviewed head, plan digest, and command.
The Rust entry validates those
bindings before building `LocalHostServices`, then supplies the production
60-second/3-second budget and frozen native executable digest to
`run_original_task_once` through the assessment prepared facade. Its ignored
test entry is the only selector that can reach that code. The separate
`validate:copilot-acp-private-assessment` selector excludes ignored tests and
cannot launch an original even after the plan is later enabled.

At this head the plan remains disabled and its host-binding digest is null, so
the observe entry exits before resolving host services, creating task scratch,
or writing records. `frozen_enable_entry_dispatches_payload_through_the_one_shot_runner`
dispatches a synthetic payload through the same entry with a real local fake
host, verifies executable drift fails before process start, and verifies
payload identity drift fails before host resolution. It also checks the fixed
production deadline and executable policy. Only the planner may later bind a
reviewed host payload and enable the exact reviewed plan; the Python validator
cannot enable or launch it.
