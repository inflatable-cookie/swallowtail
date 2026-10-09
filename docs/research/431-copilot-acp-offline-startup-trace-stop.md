# Research 431: Copilot ACP Offline Startup Trace Stop

Date: 2026-10-09

## Scope

Audit the retained exact Darwin ARM64 `1.0.93` executable, compare the
successful Research 404 initialization with the failed Research 430 start, and
record what startup requirements remain recoverable without executing vendor
code. This work does not launch an original artifact or change qualification.

## Frozen artifact and static reading

The retained `platform/1.0.93/copilot` SHA-256 is
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`, matching
the frozen Research 404 inventory and Research 430 record. Static tools reported
an arm64 Mach-O, minimum macOS 13.5, and `LC_MAIN` entry offset `21981796`. Its
linked system libraries are CoreFoundation, Security, libc++, and libSystem.

The Mach-O has a `NODE_SEA_BLOB` section in segment `NODE_SEA`, file offset
`92831744`, size `64346793`. The recoverable `sea-loader.js` text starts at
offset `92831760`; the rest does not expose the Copilot ACP implementation as
readable source. Searches of the retained bytes found no literal `--acp`,
`session/new`, `session/prompt`, `GITHUB_TOKEN`, `GH_TOKEN`, `config.json`, or
`settings.json`. This is not evidence that the program lacks those paths. It
means this static pass cannot trace the bundled startup code to ACP
`initialize`. Node's [SEA documentation](https://nodejs.org/api/single-executable-applications.html)
describes the Mach-O `NODE_SEA_BLOB` as the injected single-executable resource;
that identifies the container, not the vendor program's behavior.

The retained package metadata identifies `@github/copilot-darwin-arm64`
`1.0.93`. Its README describes an embedded JavaScript bundle and native
add-ons. No bytes were executed, evaluated, or loaded as code.

## Research 404 and Research 430 launch tuples

Both records bind the same `1.0.93` executable digest. Their launch tuples and
effective policy differ:

| Evidence | Research 404 | Research 430 |
| --- | --- | --- |
| argv | `copilot --acp --stdio` | `copilot --model auto --acp --stdio` |
| environment names | `PATH`, `HOME`, `COPILOT_HOME`, `GH_COPILOT_HOME`, `XDG_CONFIG_HOME`, `XDG_CACHE_HOME`, `TMPDIR`, `GITHUB_TOKEN`, `GH_TOKEN`, `CI`, `TERM` | `HOME`, `PATH`, `TMPDIR`, `TERM`, `COPILOT_AUTO_UPDATE`, `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`, `http_proxy`, `https_proxy`, `all_proxy` |
| path relationships | `HOME`, both Copilot homes, XDG directories, and temp were under task scratch; token variables held synthetic placeholders | `HOME` was the host home; temp was task scratch; proxy variables pointed to the local proxy; token and Copilot-home variables were absent |
| effective policy | Network denied, host-home and repository reads denied, writes limited to scratch, broad process allowance, and `com.apple.securityd` lookup denied | `process-fork` allowed; exact executable and system runtime map roots; host-home reads denied except literal `.copilot/config.json`; host-home, repository, and record writes/reads denied; outbound traffic limited to the local CONNECT proxy; `com.apple.securityd` lookup allowed |

The Research 404 source digest `65e8357d2942c8dd554b8973d35e065ed7a4e7a9596bf1c103298396338acb2d`
matches the retained historical harness source. Research 430 binds harness
digest `f11f44d43e8baf432ed3627cd75c9630d65f0fd8fb95535cc1437fd1dbc40418`,
which matches the source used for this offline audit.

Research 404 observed `initialize` success for `1.0.93`, then
`-32000 Authentication required` from `session/new`. Research 430 exited 1
after 1.289 seconds before an ACP `initialize` response. Its 427-byte stderr
was classified `sandbox-denial`; no proxy destination, authentication request,
session, prompt, permission request, or effect was observed. The vendor startup
state remains unknown.

The argv, environment, home relationship, and effective profile all changed.
The records do not isolate one cause. The Research 430 category shows only that
the bounded stderr prefix contained a sandbox-related marker. It does not name
a denied operation, path, errno, or startup stage.

## Profile and host identity limits

Research 430 binds generated-profile digest
`c00ccaadfae0c55fceffa1455952f40583a5ece13eddde64571e389828b3611a`. Its
record stores the digest and path roles, but not the rendered profile or every
absolute input used to generate it. In particular, the historical repository
root is absent. Re-rendering from the retained source and current checkout did
not reproduce the digest. Treat it as record-bound; exact byte reproduction
was not established.

The Research 404 record names Darwin `25.6.0`, arm64. Research 430 does not
record an OS build. This audit observed macOS `26.6.2`, build `25G83`, arm64.
On that OS, `dyld-support.sb` is 2,655 bytes, SHA-256
`06215a5d32689aefe395c29710e182eb54ba22162f50df8b4842290f8a19bf1c`, and has
no nested imports. This current snapshot cannot be bound to the historical
Research 430 profile run.

The current `com.apple.securityd` launch-daemon declaration uses label
`com.apple.securityd` and advertises Mach service `com.apple.SecurityServer`.
This corrects the service name in the evidence. It does not show which service
the frozen executable uses and grants no service access. The existing
`com.apple.securityd` profile entry is preserved; no Mach grant was added.

## Startup requirements ledger

| Area | Finding for exact `1.0.93` |
| --- | --- |
| Argument validation | The recorded argv is known for each run. Internal validation and startup branching are not recoverable from the static SEA payload. |
| Home and config | Research 404 used synthetic home paths. Research 430's profile allowed only the literal read-only `.copilot/config.json` exception. Whether the executable read it is unknown. |
| Logs, cache, and state | Exact startup reads and writes are unknown. The failed run's profile and result do not identify them. |
| Helpers and subprocesses | Mach-O imports CoreFoundation, Security, libc++, and libSystem. Exact bundled helper launches are unknown. |
| Authentication and services | Research 404 reached `session/new` with a synthetic token placeholder and returned authentication required. Research 430 did not reach `initialize`; its exact credential mechanism and service use are unknown. |
| ACP startup | Research 404 observed `initialize`; Research 430 did not. Static reading cannot recover the intervening vendor control flow. |

The lack of an original denial-log line remains inconclusive. No reproducible
query and known-denial control were retained with Research 430. The committed
records and both consumed-attempt guards remain unchanged.

## Stop and next ruling

The exact historical profile cannot be reproduced, the Research 430 OS build
is not bound, and the retained SEA does not expose the exact startup path. The
existing process-group cleanup also does not account for a descendant that
escapes the group. This audit therefore does not establish safe original
admission.

The harness now blocks original-artifact entrypoints while escaped-descendant
containment is unproved. Fake-only controls remain available. The stderr
classifier, independent stage channel, proxy completeness accounting, and
synthetic config-parent and symlink cases still need a separate reviewed
implementation and fake proof.

Recommended next ruling: keep original Copilot execution blocked. If further
evidence is wanted, first authorize a separate fake-only task to prove
descendant containment, closed-vocabulary diagnostics, the independent stage
channel, proxy accounting, and config-path cases, and to make profile replay
possible without persisting private paths. Only after that review should the
planner ask whether to authorize one new, exact initialization-only `1.0.93`
attempt. The prior one-shot authority is consumed; this report grants no new
invocation, service, home, or network access.
