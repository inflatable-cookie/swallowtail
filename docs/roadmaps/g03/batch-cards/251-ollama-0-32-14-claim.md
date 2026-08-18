# 251 Ollama 0.32.14 Claim And Acceptance

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../081-ollama-0-32-14-useful-newer.md`
Depends on: card 250; Research 138

## Goal

Raise the `ollama.runtime` qualified ceiling from `0.32.1` to `0.32.14`.
Reuse `ollama.native-text-v1`. Keep `0.32.2` excluded. Add `0.32.10`.

## Scope

1. Raise `0.14.0..=0.32.14` on existing native-text behavior. Keep
   AllowUnverified.
2. Keep exact `0.32.2` excluded. Add exact `0.32.10` because GitHub still
   marks that plain version prerelease.
3. Move synthetic later-stable UnverifiedNewer to `0.32.15`.
4. Refresh focused tests, matrices, Ollama guides, architecture, and
   contracts that name the ceiling.

## Out Of Scope

- Ollama Cloud, generate, tools, or thinking as selected operations
- renaming the decoder specimen directory
- Gemini or other Research 127 families
- capturing a live prompt or starting the attached server
- install, update, or publication

## Acceptance Criteria

- [x] published `0.32.3` through `0.32.14` except `0.32.10` classify as
      Qualified Maintained
- [x] `0.32.2` and `0.32.10` remain incompatible
- [x] `0.32.15` remains permitted UnverifiedNewer
- [x] decoder specimen remains `ollama-native-v0.14.0-v0.32.1`
- [x] focused Ollama proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Validation

- `effigy validate:focused swallowtail-adapter-ollama`
- `effigy package:verify-affected swallowtail-adapter-ollama`
- `effigy qa:routes`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if card 250 did not name compatible-extension
- stop if live provider work or attached-server start would be required to
  close the claim
- stop if `0.32.14` is no longer the official stable point

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a
time and qualify useful-newer support; do not leave the current
host/official stable unqualified. Gemini stays deferred.

## Evidence

- Research 138
- `crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.32.14/`
- Decoder specimen remains `ollama-native-v0.14.0-v0.32.1`
- latest qualified = `0.32.14`
- exclusions = `0.32.2`, `0.32.10`
- synthetic later-stable UnverifiedNewer is `0.32.15`
