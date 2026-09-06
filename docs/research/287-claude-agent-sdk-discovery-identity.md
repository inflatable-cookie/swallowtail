# 287 Claude Agent SDK Discovery Identity

Status: evidence stop; no Card 086 range admitted
Date: 2026-09-06
Card: g05.029 Card 086
Authority: Research 280, Contract 029, and the promoted Card 086 manifest

## Question

Can the SDK sidecar identify compatible triples of wrapper 0.3.259, Claude
native, and Node without a prompt or live turn? The required evidence is
system/init, supportedModels, harnessSchema, and any refusal for each triple.
This document records only evidence that exists; it does not change a claim,
pin, source file, guide, matrix, or baseline.

## Method

No new SDK/native compatibility probe or provider session was run for Card 086.
The provider-free inventory did run the installed CLI's version command, but
that command only reports the executable version; it is not a provider session
or an SDK/native sidecar compatibility probe. The repository already records
the permitted open-only probes and their stop point. Card 100 records that the
initialize exchange and bounded supportedModels() and accountInfo()
controls can complete without awaiting query.next(), while system/init is the
first message of the first query and a missing message is init_missing (Card
100, lines 191-197). The same record says not to run another live probe before
code review after the open-only diagnostic account_not_subscription (Card 100,
lines 253-260). That is the evidence boundary used here.

The provider-free host inventory was captured without ambient provider
variables or a provider session:

    E1  /Users/tom/.local/bin/claude --version
        2.1.258 (Claude Code)
    E2  /opt/homebrew/bin/node --version
        v26.7.0
    E3  /Users/tom/.local/bin/node --version
        v22.23.2

The SDK artifact facts are from Research 280: package 0.3.259 declares
engines.node >=18.0.0 (Research 280, lines 61-65), and the artifact keeps
sdkCompat.harnessSchema at 1 while its tested wrapper list still ends at
0.3.227 (Research 280, lines 126-136). The latter is artifact metadata, not
per-triple runtime evidence.

## Inventory

| Axis | Observed inventory | Evidence |
| --- | --- | --- |
| Wrapper | Official @anthropic-ai/claude-agent-sdk 0.3.259 | Research 280, lines 39-49 |
| Bundled native | Coupled native 2.1.259 | Research 280, lines 126-131 |
| Host native | Installed claude 2.1.258 | E1 |
| Node | 22.23.2 at ~/.local/bin/node; 26.7.0 at /opt/homebrew/bin/node | E2-E3 |
| Harness schema | Artifact value 1 | Research 280, lines 133-134 |

No older native point was installed by this worker. No package, native
artifact, or Node version was installed or changed.

## Triple evidence

The following are the complete triples that have probe evidence available to
this card. The references are exact repository evidence lines; no result is
inferred from a changelog or from the artifact version alone.

| Wrapper | Native | Node | Classification at this boundary | Exact evidence |
| --- | --- | --- | --- | --- |
| 0.3.259 | bundled 2.1.259 | 22.23.2 (~/.local/bin/node, also the pinned point) | surface-changed: the retired repo-side account_not_subscription diagnostic was followed by successful open/initialize controls; Card 086 still admits no range because its permitted transcript lacks system/init | Card 100, lines 253-272 and 287-300 |
| 0.3.259 | bundled 2.1.259 | 26.7.0 (/opt/homebrew/bin/node) | refused-with-code was observed as initialization_failed; a later bounded open-only run timed out without system/init, so not compatible | Card 100, lines 237-250 |

For the 22.23.2 triple, account_not_subscription is a retired repo-side
diagnostic defect, not a current provider or native-identity refusal. The
stronger prior live evidence records open success, system/init, canonical cwd
match, requested-model confirmation, effective model, capability labels, and
the later provider-failed terminal/cleanup posture (Card 100, lines 287-300).
That live turn is outside Card 086's initialize-only acceptance boundary, so it
does not admit the triple here. The later open-only evidence independently
recorded requested-with-supported-list, first-party gate passage, five
supported-model rows, and presence-only account-source labels, but no
system/init and no model or account values (Card 100, lines 261-272). For the
26.7.0 triple, the bounded run reached the native child but recorded no SDK
first message, system/init, model, account projection, or rejection code (Card
100, lines 244-252). It remains refused-with-code/unresolved, not compatible.

The artifact-level harnessSchema: 1 is recorded for both rows by Research 280,
lines 133-134. It is not a runtime field observed in either open-only
transcript, so it does not satisfy the per-triple acceptance evidence.

## Withheld triples

These triples were not probed and therefore have no compatibility
classification:

| Triple set | Why withheld |
| --- | --- |
| 0.3.259 + host claude 2.1.258 + 22.23.2 | A host-native crossing would require a real credential-dependent SDK/native probe. Card 100's account_not_subscription design-review stop forbids another live probe before code review (lines 253-260); no new credential or probe is authorized |
| 0.3.259 + host claude 2.1.258 + 26.7.0 | Same credential-dependent Card 100 design-review stop (lines 253-260); no new credential or probe is authorized |
| 0.3.259 + each native point 2.1.227..=2.1.258 + 22.23.2 | No such native artifact was installed or made available for a permitted probe |
| 0.3.259 + each native point 2.1.227..=2.1.258 + 26.7.0 | No such native artifact was installed or made available for a permitted probe |
| 0.3.259 + native 2.1.259 + any other Node point | Only the two observed Node installations were in scope; no install is permitted |

The repository does contain prior live-turn evidence in which Node 22.23.2
yielded system/init (Card 100, lines 273-286). That operation was explicitly a
live turn, not an initialize-only Card 086 probe, and is excluded from the
compatibility table. Reusing it would violate this card's initialize-only
boundary.

## Invariants

1. A successful initialize-control response is not system/init; the latter
   belongs to the first query message (Card 100, lines 191-197).
2. An artifact harnessSchema value is not a per-triple runtime observation.
3. account_not_subscription in Card 100, lines 253-260, is a retired repo-side
   diagnostic defect; the stronger Node 22.23.2 evidence is recorded at lines
   287-300.
4. A refusal followed by a later open-only surface that still lacks system/init
   does not prove compatibility.
5. Node engines.node >=18.0.0 is package admission metadata, not evidence that
   this sidecar/native triple is compatible.
6. No claim, pin, source, guide, matrix, or baseline may move from these
   observations.

## Decision

Card 086 admits no new candidate range. The candidate ranges are explicitly
empty at this evidence boundary:

| Axis | Candidate admitted range from Card 086 | Reason |
| --- | --- | --- |
| SDK wrapper | none | No initialize-only triple has the required system/init evidence |
| Bundled native | none | No initialize-only triple has the required system/init evidence |
| Host native discovery | none | The installed 2.1.258 executable is inventory-only; its credential-dependent crossing is stopped by Card 100's design-review boundary |
| Node 22.23.2 | none | Open-only initialize controls were observed, but system/init was not |
| Node 26.7.0 | none | Refusal/timeout evidence exists, but no successful system/init |
| Harness schema | none as a range axis | 1 is unchanged artifact metadata, not a runtime range result |

Existing production claims and exact pins remain unchanged. Card 087 must not
consume this document as admitted ranges.

## Falsification

| Statement | Smallest falsifier | Result |
| --- | --- | --- |
| A listed triple is compatible | One permitted initialize-only transcript containing system/init, supportedModels, and the runtime harnessSchema for that same triple | Not captured; no compatibility claim made |
| A candidate range can be widened | Evidence at the proposed endpoint and every required boundary triple | Not available; no range widened |
| Host claude 2.1.258 is interchangeable with bundled native 2.1.259 | A permitted sidecar transcript identifying the host executable and recording the required fields | Not run; no interchangeability claim made |

## Withheld

- No new system/init capture under the initialize-only rule.
- No live prompt, live turn, or credential-dependent probe.
- No host-native sidecar crossing for claude 2.1.258; Card 100's credential-dependent design-review stop remains in force.
- No probes for native points 2.1.227..=2.1.258.
- No claim, pin, source, guide, matrix, or baseline changes.

The precise blocker is evidence, not planning: the required runtime system/init
evidence is only available after the first query message, and Card 086 forbids
that live turn. The card's stop condition therefore applies.
