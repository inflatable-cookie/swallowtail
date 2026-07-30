# 2026-07-30 Realtime And Non-Applicable Activity Closeout

## Change

- added one reusable prepared-evidence assertion for non-agent operation roles
- proved six route-local catalogue or inventory operations, four auxiliary
  catalogue operations, two realtime-media sessions, and one serving
  lifecycle as `NotApplicable`
- exposed immutable prepared evidence on the four auxiliary catalogue facades
- retained dedicated OpenAI Realtime and Gemini Live media event models
- retained external ownership for attached Ollama and llama.cpp inference
- retained separate owned llama.cpp serving lifecycle

## Boundary

`NotApplicable` is operation truth, not missing support. Model listings,
installed or resident model observations, media transcripts, audio, commit,
interruption, response status, rollover, readiness, and stop evidence do not
become ordinary agent activity.

The selected OpenAI Realtime and Gemini Live profiles have no qualified tool
activity. Current richer upstream surfaces do not widen those exact routes.
Direct inference still reports only the assistant, readable reasoning summary,
and exact provider or consumer tool activity selected by Research 067.

## Evidence

- all 13 negative classifications use actual public prepared-operation
  evidence
- the machine corpus still accounts for 13 non-harness routes, 14 positive
  text profiles, and 13 non-applicable operations
- realtime production tests retain their native audio, transcript, response,
  cancellation, interruption, and rollover assertions
- attached and owned runtime tests retain residency, reachability, readiness,
  and cleanup truth

## Validation

- 46 focused catalogue, prepared-facade, realtime, attached, serving, and
  applicability-corpus tests pass
- `effigy check:rust`
- `effigy lint:rust`
- `effigy qa:docs`
- `effigy qa:routes`
- `effigy package:api`
- `effigy format:check`

The first Alibaba catalogue test correctly rejected the workspace inference
access profile. The fixture now uses the separate international control-plane
audience and credential profile required by the prepared catalogue.

## Current State

Roadmap g02.039 and cards 132-134 are complete. Roadmap g02.040 is active.
Cards 135-137 remain in bounds. Card 135 is the sole ready task: publish the
provider-solution activity matrix and consumer guidance.
