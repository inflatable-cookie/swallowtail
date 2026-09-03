# Claude SDK macOS degraded cleanup posture

Status: operator decision recorded; g05.023 complete; card 055 ready on preserved PR 188
Date: 2026-09-03

## Decision

Ordinary macOS may expose `claude-agent.sdk` without claiming descendant-tree
emptiness. The route must bound interruption, disposal, escalation, and
sidecar/root join by the caller's cleanup deadline. A confirmed root exit after
the declared descendant termination attempt is `Degraded` because descendants
remain unconfirmed. An unconfirmed root exit or observed survivor is `Failed`.

`ProcessTreeCompletion::OwnedTreeEmpty` remains the only evidence that may
support `Clean`. Root-only evidence is never cached or widened to another route
or platform. Applications may reject the degraded platform posture at route
selection.

## Rationale

Card 059 found no sound owned-tree-empty observation within current ordinary
host-local macOS authority. Requiring an entitlement or system extension would
make the subscription-backed route disproportionate for ordinary desktop-app
use, while keeping the route unavailable would defeat the selected Claude
feature goal. Honest degradation preserves the evidence boundary without
pretending the process tree is joined.

## Continuation

Card 055 and PR 188 retain their existing worker, workspace, branch, and PR
identity. The worker must restack onto current `main`, preserve the already
accepted SDK identity/model/tool/credential/wire work, repair every lifecycle
path against the caller cleanup deadline, and prove the macOS `Degraded` versus
`Failed` split. No provider turn, login, token read, release operation, or tag
action is authorized.
