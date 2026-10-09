# Research 432: Copilot ACP No-Child Initialize Diagnostic Preparation

Date: 2026-10-09

## Scope and authority

Task 166 implements Tom's approval for fake-only diagnostic preparation under
decision `4d499c96-4046-4189-b14e-39b1a572e035`. It does not stage or execute
the frozen Copilot `1.0.93` artifact, change a route claim, or consume either
existing permission-proof record. The operation ID is
`d67e7bb3-8314-48f3-92d1-fd73c73f1b77`.

## Bound host and profile

The fake proof ran on macOS `26.6.2`, build `25G83`, arm64. The recorded
`dyld-support.sb` closure contains one file, size 2,655 bytes, SHA-256
`06215a5d32689aefe395c29710e182eb54ba22162f50df8b4842290f8a19bf1c`. The
fixture record binds the exact profile template, relative synthetic layout,
symbolic deny-only host-home and checkout roles, fake and proposed target
executable roles, and the action-root working directory. It binds the
imported closure, the source and compiled identities for the stage
launcher and fake, Apple clang `21.0.0`, and the harness sources.

The fake profile imports that system closure, permits runtime reads and maps
under `/System/Library` and `/usr/lib`, and grants `process-exec` only to the
task-owned stage launcher and target. It denies `process-fork` after imported
rules, all Mach lookup, all network access, host-home and repository access,
and record-root access. Writes are limited to the task's action root. The
current host home and checkout appear only as symbolic deny roles in the
retained template; private paths are not persisted.

## Fake result

The target received only `--acp --stdio` and returned one fake ACP
`initialize` response. It made no `session/new` or `session/prompt` request.
The stage-to-target path kept the same root PID. All eight child-creation
controls returned `EPERM` or `EACCES` before any child effect marker:

- `fork`: self-exec, system helper, and a child session-escape attempt;
- `vfork`: self-exec and system helper;
- `posix_spawn`: self-exec, system helper, and the stage launcher.

A direct self-exec replacement kept the same PID. A direct exec of an
unlisted system helper was denied. The fake's at-exit spawn was denied and
left no marker. These controls exercise the exact executable grants alongside
the final no-fork rule, including the imported profile.

Synthetic config, keychain and repository sentinels had denied reads and
writes. Lookups for `com.apple.securityd` and `com.apple.SecurityServer` were
denied. A connection to reserved test address `203.0.113.1:443` and a local
bind were denied. No credential contents were read; authentication-service
lookups were denied before a port was returned. The network control used a
reserved documentation address.

An early fake exit left startup `unknown` despite stage-channel EOF. A
cancel-ready fake exited and joined after direct-PID `SIGTERM`; an unresponsive
fake reached the timeout, was stopped by direct PID, and its process, stage
channel and stderr reader joined. A durable exclusive record was fsynced
before fake start. A simulated crash after that record refused replay without
starting a root. A secret-bearing stderr control persisted fixed marker facets
and byte counts only; the raw text, synthetic path, secret sentinel and test
hash sentinel were absent from its record.

## Data-only diagnostic request

The preparation record binds the Darwin ARM64 `1.0.93` archive SHA-256
`98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71` and
executable SHA-256
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`. Neither
was staged or executed. It contains a data-only candidate profile and
environment with synthetic `HOME`, `COPILOT_HOME`, XDG and temporary roles,
no token or proxy variables, no `--model auto`, at most one initialize
request, a 60-second ceiling, and a three-second cleanup bound. The request
sets `execution_authorized: false`; it is not an original-launch entrypoint.

The replayable profile and result ledger are in
[`preparation-record.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic/preparation-record.json).
The source harness is
[`copilot-acp-no-child-diagnostic.py`](../../scripts/copilot-acp-no-child-diagnostic.py)
and the task-owned native target is
[`copilot-acp-no-child-fake.c`](../../scripts/copilot-acp-no-child-fake.c).

## Limits

The result proves the fake boundary only for the recorded macOS build and
profile. It does not establish that Copilot starts, needs no child, reaches
`initialize`, or works without host login, Auto model policy, authentication
services or provider access. No permission-proof route is contained by this
claim. An original attempt requires separate exact authority and independent
review. The exact `1.0.80` claim, both consumed records and all existing
replay guards remain unchanged.
