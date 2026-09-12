# 306 Grok 1.0.25 Authenticated Catalogue Evidence

Status: complete
Verdict: admitted; exact `1.0.25` `--no-auto-update models` as an authenticated
non-inference metadata operation under `HarnessConfigurationPosture::Ambient`
Owner: Tom
Date: 2026-09-12
Card: g05.053

## Question

What is the honest boundary for the exact installed Grok `1.0.25` pre-session
model catalogue command, and does the accepted observation prove a no-prompt,
no-session, no-tool, no-inference operation?

## Method

Static inspection of the shipped exact-`1.0.25` command grammar and the
installed executable's format pieces, argv-grammar probes, the Swallowtail
host-local launch path, PR #315 review history, the two earlier observations,
and the accepted final redacted capsule. No new Grok command was run for this
record.

## Identity

The exact installed executable is unchanged from Research 305:
`grok 1.0.25 (f7e67d6988e2) [stable]`, local executable SHA-256
`9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c`,
138080336 bytes, reached through `/Users/tom/.local/bin/grok` →
`/Users/tom/.grok/bin/grok`. The shipped user guide the CLI extracts on launch
is the exact-version document set under `$GROK_HOME/docs/user-guide/`.

## Command grammar

Only `["--no-auto-update", "models"]` qualifies. The root flag is accepted in
root position (`grok --no-auto-update models --help` exits 0) and rejected
after the subcommand (`grok models --no-auto-update --help` fails with
`unexpected argument '--no-auto-update'`). `grok models` takes no `PROMPT`
argument and no session, persistence, tool, extension, project, update, retry,
or provider-invocation flag. The embedded default-model document stays
supplemental; the live listing remains authoritative for membership, order,
and default.

## Shipped output grammar

A prior observation exited zero with 113 stdout bytes and zero stderr bytes
that the Research 305 bare-row grammar rejected. Static recovery from the
installed binary's `.rodata` table next to the shipped path
`…/xai-grok-pager/src/models.rs` proves the exact-`1.0.25` format pieces:

- authentication preamble, one of `You are logged in with <account>.\n`,
  `You are using XAI_API_KEY.\n`, `Model '<name>' is using its own API
  key.\n`, `You are authenticated via deployment key.\n`, or `You are not
  authenticated.\n`;
- `Default model: ` (15), `\n` (1), `Available models:\n` (18), `  - ` (4),
  `\n` (1), `  * ` (4), ` (default)\n` (11).

Each row therefore carries a bullet: `  * <id> (default)` for the source
default and `  - <id>` for every other model. The reconstructed 113-byte
document is the 35-byte preamble `You are logged in with <account>.\n` plus
the 78-byte body:

```
Default model: grok-4.6
Available models:
  * grok-4.6 (default)
  - grok-4.5
```

The catalogue parser requires the bullet prefix and the exact default marker,
and tolerates the authentication preamble. Research 305's bare-row grammar is
retained only as historical evidence with a correction note.

## Corrected boundary

Contract 020 (updated 2026-09-12) admits one ephemeral **authenticated
metadata** process for a dedicated harness catalogue when the source command
is independent of a model session. The operation may perform bounded
authentication or catalogue-metadata traffic that is part of the source
command. It must not send a prompt, open a model session, invoke inference,
dispatch a tool, enable an extension, update the harness, retry, or retain
provider state beyond its operation scope. `ProviderSuppressed` is required
only when a consumer or route contract actually requires provider
configuration suppression; it is not the default meaning of a prompt-free
catalogue.

The catalogue therefore carries `HarnessConfigurationPosture::Ambient`, binds
the prepared ambient environment and exact `--no-auto-update models` argv, and
does not materialize an operation-private suppression file or gate on
enterprise precedence. The removed suppression machinery is not replaced with
an operating-system network sandbox. Grok's auth-refresh watcher and bounded
catalogue metadata traffic are permitted; absence of such traffic is not an
acceptance condition, and no-prompt or zero-credit success does not prove that
metadata traffic was absent.

## Observations

- The first observation used a `GROK_CONFIG` overlay. It exited zero but
  started the auth-refresh watcher and wrote a fresh remote-origin
  `models_cache.json`. That overlay route was disproved as a suppression
  mechanism and is frozen, not retried.
- The replacement observation used an operation-private `$GROK_HOME/config.toml`
  and exited zero with 113 stdout bytes and zero stderr bytes. The frozen
  bare-row parser rejected the document before the probe retained the home
  proof; static recovery above reconstructs the exact bullet document.
- The final observation is the accepted evidence. The redacted capsule
  (`live-capsule.json`, schema v2) records exact argv, the isolated authorized
  home, the private-config digest used at the time, `outcome = "success"`,
  `stdout_bytes = 113`, `stderr_bytes = 0`, `model_ids = ["grok-4.6",
  "grok-4.5"]`, `default_model = "grok-4.6"`, `process_joined = true`, and no
  prompt, session, or inference. Its `runtime_proof` fields (auth-refresh
  watcher started, no remote-fetch-disabled log line, no fresh remote-origin
  cache) are descriptive metadata-path evidence, not acceptance gates under
  the corrected boundary.

The accepted capsule's zero-credit success supports the observed
non-inference classification; it does not prove entitlement, future billing
state, or the absence of metadata traffic.

## Admitted implementation

The preserved admitted tree from PR #315 (`06ba3176`) was recovered. The
admitted correction keeps:

- exact argv `["--no-auto-update", "models"]` and the exact approved
  executable/environment binding;
- `HarnessConfigurationPosture::Ambient` in the configured instance, the
  operation requirements, the immutable plan, and driver validation;
- the shipped `*`/`-` bullet grammar with the authentication preamble;
- bounded stdout and stderr capture before parsing, retaining only byte counts
  and SHA-256 digests (`GrokCatalogueOutputEvidence`) and the ordered entries
  (`GrokCatalogueListing`);
- joined success, failure, deadline, and cleanup lifecycle;
- live listing authority for membership, order, and default; the frozen
  exact-`1.0.25` embedded document supplements matching ids only, and unknown
  valid ids pass through with empty metadata.

`grok_build_acp_claim`, `grok_build_model_for_version`, released `v0.5.0`, and
every tagged or historical ledger and release note are unchanged. Route 50
(`grok-build.catalogue`) is additive post-`v0.5.0`; tagged `v0.5.0` and
Research 281 stay frozen at 49.

## Sources

- `$GROK_HOME/docs/user-guide/` shipped exact-`1.0.25` documents
- installed `/Users/tom/.grok/bin/grok` argv-grammar probes and read-only
  `.rodata` format-piece recovery
- `crates/swallowtail-host-local/src/process/launch.rs`
- `crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.25-model-catalogue/live-capsule.json`
- Contract 020 (updated 2026-09-12), PR #315 review history, Research 305
