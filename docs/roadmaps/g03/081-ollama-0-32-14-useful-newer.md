# 081 Ollama 0.32.14 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-18
Depends on: Research 127; Research 138; g03.080
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 031, 036-039, 043
Planning state: cards 250-251 completed

## Problem

Research 127 ranked AllowUnverified host drift after Kimi. Ollama attached
is `0.14.0..=0.32.1` excluding `0.32.2` with AllowUnverified. This host is
`0.32.9`. Official GitHub latest is `v0.32.14`. Raising the bound is
useful-newer support, not a silent `latest` bump and not a flatten onto
Ollama Cloud.

## Generation Runway Goal

Freeze Ollama `0.32.9` / `0.32.14` identity against the `0.32.1` corpus,
then raise the qualified ceiling only if that evidence names a compatible
extension.

## Goals

- [x] rank remaining AllowUnverified host-drift families and pick Ollama
- [x] freeze host `0.32.9` and official GitHub `0.32.14` identity and
      selected native API source
- [x] raise the native runtime claim through exact `0.32.14`, keeping
      `0.32.2` excluded and adding `0.32.10`
- [x] leave Claude Agent ACP, Pi, Qwen, Antigravity, and Gemini for later
      one-family work

## Non-Goals

- flattening Ollama Cloud, generate, tools, or thinking into the selected
  text facade
- inferring `0.32.2` or `0.32.10` qualified because they use plain version
  strings
- bulk-bumping other AllowUnverified families
- Gemini requalification
- provider prompts, install, update, server start, or publication

## Execution Plan

### Batch 81.1 — Identity Corpus

- [x] Execute card 250.
- [x] record host `0.32.9` and official `0.32.14` against exact `0.32.1`
- [x] name compatible-extension for card 251 without changing production
      claims

### Batch 81.2 — Claim And Acceptance

- [x] Execute card 251 after card 250 names compatible-extension.
- [x] raise `0.14.0..=0.32.14` on `ollama.native-text-v1`
- [x] keep `0.32.2` excluded; add `0.32.10`
- [x] keep AllowUnverified and synthetic later `0.32.15`
- [x] refresh matrices, guides, contracts that name the ceiling, and
      focused Ollama proof

## Acceptance Criteria

- [x] host `0.32.9` and official `0.32.14` classify as Qualified
      Maintained
- [x] `0.32.2` and `0.32.10` remain incompatible
- [x] `0.32.15` remains permitted UnverifiedNewer
- [x] decoder specimen remains `ollama-native-v0.14.0-v0.32.1`
- [x] focused Ollama proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Decision Gates

- Stop if exact artifact identity cannot be corroborated from official
  sources.
- Stop if selected native protocol differs from the recorded evidence.
- Stop if qualifying the current official/host point would require a
  provider prompt or starting the attached server.

## Closeout

g03.081 is complete. Next family is Claude Agent ACP (registry newer;
host already on a qualified bound). Gemini stays deferred.
