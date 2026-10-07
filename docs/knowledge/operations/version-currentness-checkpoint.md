# Version Currentness Checkpoint

Revalidate every production Swallowtail route against official stable points
without turning `latest` into a compatibility claim. Contract 029 owns the
rules. Research 091 and 127 are the method specimens.

This guide does not override contracts. It is the operator and agent runbook
for the named checkpoint.

## When To Run

Run when any of these is true:

- the operator asks for a version sweep
- a consumer hits a defect on an unverified-newer point
- a cluster of stables has moved since the last research currentness record

This is a standing Contract 029 Queue lane (`version-currentness`), never
finished.

Do not run it as CI, as a calendar cron, or as an install/update/login
session.

## Scope

Every production route family:

- installed harnesses
- attached harnesses and local runtimes
- owned serving
- hosted API facades
- embedded SDK pins
- shared ACP schema

Ignore preview, nightly, alpha, and development channels unless the
Swallowtail pin is itself that prerelease. Ignore hosted "latest model".
Do not flatten packaging, desktop About, or unofficial launchers onto the
named compatibility axis.

### Pre-v0.5.2 Sweep Authority

Tom ruled on the Queue board on 2026-10-07: "Approve remaining one-family
qualifications with the stated boundaries" (decision
`a641caad-d963-4277-b62c-9fa6ac0247e9`). For the sweep inventoried in
[Research 369](../../research/369-all-route-version-currentness-checkpoint.md),
this reopens exact pins and major-version, schema and version-scheme transitions
for separate family qualification and adaptation tasks. Each task re-probes its
official target and proves exact artifact identity before changing a claim.

Preserve released consumer contracts and existing qualified points. A
consumer-visible narrowing, public API or lifecycle change, missing artifact
identity, or need for live proof returns for a separate ruling. Muse and ZCode
source gaps permit metadata discovery only until an exact target is established.
Node sidecar work stays on the Node 22 patch line. Antigravity's deferred
interior-point backfill stays out of scope. This ruling authorizes no live
provider work, credentials, installation, host mutation, workflow change, tag
or publication.

## Method

1. Read the current adapter `selection.rs` claims and the production
   feature-matrix version columns.
2. Record safe local `--version` for tools on `PATH`. Missing local install
   is not a gap.
3. Record official stable points: npm `latest`, GitHub latest stable
   release or tag, crates.io max stable, ACP registry metadata, vendor
   channels where those are the axis.
4. Fill one row per family:

   | Surface | Local observation | Current external point | Swallowtail boundary | Result |

5. Classify with this vocabulary:
   - `unchanged` — local and official points still sit on the qualified bound
   - `visible unverified-newer` — a later stable exists; AllowUnverified
     already classifies it; no bound change
   - `record only; future range work deferred` — newer point exists, but
     extension needs a dedicated family card or an existing deferral still
     holds
   - `material candidate` — enough evidence to ask before compiling one
     family range card
6. Write the next research currentness record. Index it. Optionally write
   one log.
7. Stop. The checkpoint does not edit claims, matrices, or fixtures.

## After The Record

A checkpoint row of `visible unverified-newer` is research, not permission
to skip the family. Do not leave the current host or official stable
UnverifiedNewer without a named incompatible reason. g03.068
provisional-newer is only for stables above the latest qualified point.

Compile one-family range work using Contract 029's Upgrade Workflow:

1. observe the exact interface versions and capability surface
2. add or update a frozen corpus for changed behavior
3. run the existing provider-neutral profile and adapter assertions
4. extend the latest segment when behavior is unchanged, add a milestone
   when adapter-private mapping changes, or create a new driver/facade
   revision when the public lifecycle changed materially

One family per card. When official latest moves during a run and before
the identity commit lands, extend the hop set under Contract 029's In-Run
Latest Movement rule instead of stopping; stop only on a changed selected
surface, authority change, major-line reset, or channel disagreement. Exact-pin and qualified-only claims stay rejected
above the pin until that family has its own corpus. A major-line reset on
the same package is an identity investigation, not an unverified-newer
default.

Execute that upgrade loop through the repo skill `version-currentness`
(`.cursor/skills/version-currentness/`). Gemini requalification stays
deferred until the operator lifts that gate.

## Sources

Use the family's documented official channel. The 127 checkpoint used:

- npm `latest` for published CLIs
- GitHub releases or tags for Antigravity, Ollama, llama.cpp, Claude Agent
  ACP, and ACP schema
- `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
  as discovery metadata, not as a Swallowtail claim
- crates.io max stable for Bedrock SDK pins
- local `--version` only as observation

When a package has no trustworthy public-source correlation, bind evidence to
the complete published artifact tree and its digest. Do not substitute a
public tag or changelog for shipped behavior. Record the missing correlation,
keep source-parity claims out of the result, and recheck the official artifact
immediately before qualification. The Claude Agent SDK is the current example:
its npm tarball is authoritative, while its public repository does not contain
the shipped SDK implementation.

Do not send a provider prompt. Do not authenticate. Do not install or
update a harness to complete the checkpoint. Do not run workspace `qa`.

## Validation

Docs-only checkpoints run the docs index gates named by the card. Range
cards name focused package proof themselves.
