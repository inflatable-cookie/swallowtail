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
- g03 returns to its evidence gate and currentness is the question

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

Compile one-family range work only for material candidates, using Contract
029's Upgrade Workflow:

1. observe the exact interface versions and capability surface
2. add or update a frozen corpus for changed behavior
3. run the existing provider-neutral profile and adapter assertions
4. extend the latest segment when behavior is unchanged, add a milestone
   when adapter-private mapping changes, or create a new driver/facade
   revision when the public lifecycle changed materially

One family per card. Exact-pin and qualified-only claims stay rejected
above the pin until that family has its own corpus. A major-line reset on
the same package is an identity investigation, not an unverified-newer
default.

Gemini requalification stays deferred until the operator lifts that gate.

## Sources

Use the family's documented official channel. The 127 checkpoint used:

- npm `latest` for published CLIs
- GitHub releases or tags for Antigravity, Ollama, llama.cpp, Claude Agent
  ACP, and ACP schema
- `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
  as discovery metadata, not as a Swallowtail claim
- crates.io max stable for Bedrock SDK pins
- local `--version` only as observation

Do not send a provider prompt. Do not authenticate. Do not install or
update a harness to complete the checkpoint. Do not run workspace `qa`.

## Validation

Docs-only checkpoints run the docs index gates named by the card. Range
cards name focused package proof themselves.
