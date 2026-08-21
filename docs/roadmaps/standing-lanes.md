# Standing Lanes

Status: active
Owner: Tom
Updated: 2026-08-22

These lanes outlive a generation. They are not generation runway goals
and they do not keep a generation open.

## Contract 029 Currentness

Authority: Contract 029 Recurring Currentness Checkpoint and Upgrade
Workflow. Runbook:
[Version Currentness Checkpoint](../guides/version-currentness-checkpoint.md).
Execute through the repo skill `version-currentness`.

Cadence is operator-triggered: after a consumer defect on an
unverified-newer point, after a cluster of stables move, or when the
operator asks. It is not CI and not a calendar job.

A checkpoint writes research. It does not change claims and does not
need an active generation.

One-family qualification that changes a claim compiles into the
then-active generation. If none is active, stop and ask. Do not invent a
generation to house currentness.

Last one-family claim: Gemini CLI ACP and headless `0.56.0` (g04.034,
cards 093-094). The immediately preceding claim is Kimi Code ACP, headless,
and local-server `0.38.0` (g04.032, PR 30). Neither changes the generation
pointer.

### Completed Family

Gemini CLI requalification is complete. Host `0.53.0` and official stable
`0.56.0` were compared on separate ACP and headless axes under the enterprise
API-key posture. Both axes are compatible extensions through `0.56.0`; Code
Assist browser login and individual-account service remain outside the route.
The first unpublished stable, `0.56.1`, remains permitted `UnverifiedNewer`.

g04.034 and cards 093-094 compile that work. No live prompt, authentication,
host update, or contract decision was required.

### Do Not

- treat currentness as a reason to keep a generation active
- bulk-bump from registry `latest`
- leave the current host or official stable `UnverifiedNewer` without a
  named incompatible reason

## Next Task

Implement the active roadmap card named by the
[roadmaps front door](README.md#next-task). This file records standing
lanes; the batch pointer stays in the front door.
