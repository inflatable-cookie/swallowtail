# 2026-07-22 Pi RPC Records, Version Qualification, And Corpus

Card 099 is complete.

Contract 029 now treats exact interface versions and compatibility claims as
different records. Adapter, package, SDK, wire, service, facade, instance,
route, and model revisions cannot substitute for one another. A driver owns a
maintained support window with a baseline, latest-qualified boundary, private
behavior milestones, deprecation states, and exact exclusions. The initial Pi
window remains one exact point. Later evidence can retain older installed
versions while extending or segmenting the window; unknown versions fail
preflight. This lets routine provider updates reuse a driver without making
`latest` or an untested semver interval safe.

Core now binds exact interface points to configured instances, requirements,
driver claims, and immutable plans. Pure preflight rejects an unqualified point
or a mismatched harness RPC policy. The first restrictive policy fixes one
active operation, two completed prompts, one pending steering message, one
pending follow-up, and empty ambient customization, update, telemetry, package,
and automatic-retry allow-lists.

Runtime records keep prompt, steering, and follow-up classes separate. A
correlated command response reports only acceptance or rejection. Bounded UI
dialogs use the callback exchange; display-only UI remains semantic observation
without a response. Redacted command, dialog, display, and content records keep
raw provider material out of stable diagnostics.

The existing eleven profiles do not change. A separate assertion pack over the
long-lived RPC profile proves version and policy preflight, scheduling order,
busy and overflow rejection, callback timeout, late-response rejection, UI
observation, exact provider/model binding, and ambient read intent without a
filesystem boundary.

The new `swallowtail-adapter-pi` scaffold contains adapter-private decoding and
a frozen `@earendil-works/pi-coding-agent@0.80.10` corpus. It fixes strict LF
JSONL, offline and ephemeral startup, exact provider/model placeholders,
read-intent tools, disabled customization, disabled retry/compaction, command
acknowledgements, events, UI, malformed, unknown, and disconnect evidence. It
does not launch Pi or access authentication or a model provider.

Focused core, runtime, testkit, and Pi validation passes 129 tests. Focused
warnings-denied clippy, workspace all-target check, docs QA, explicit index
links, and diff checks pass. Doctor returns the inherited 19 oversized-file
findings: 7 errors and 12 warnings, with no batch-added finding.

Card 100 is ready for the production process driver. Card 101 remains the
bounded cross-topology conformance and roadmap closeout continuation.
