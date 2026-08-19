# 104 Oh My Pi 17.3.8 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: Research 159; Research 166; g03.103
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 029, 032, 036-037, 039, 044, 052
Planning state: cards 320-321 completed

## Problem

Research 159 ranked remaining AllowUnverified families after Kimi. Oh My
Pi RPC is qualified `17.2.9..=17.3.7` with AllowUnverified. This host is
already qualified `17.2.15`. Official npm `latest` is `17.3.8`. Raising
the bound is useful-newer support, not a silent `latest` bump and not a
flatten onto `pi.package`.

## Generation Runway Goal

Freeze Oh My Pi host `17.2.15` / official `17.3.8` identity against the
`17.3.7` corpus, then raise the qualified ceiling only if that evidence
names a compatible extension.

## Goals

- [x] rank remaining AllowUnverified registry-newer families and pick
      Oh My Pi `17.3.8`
- [x] freeze npm `17.3.8` identity, host `17.2.15` help, and selected RPC
      protocol
- [x] raise the package claim through exact `17.3.8` on the existing
      behavior revision
- [x] leave Antigravity and deferred Gemini for later one-family work

## Non-Goals

- mixing `pi.package` into this milestone
- mapping `providers.cacheRetention`, advisor, ACP, session switching, or
  subagent authority
- Gemini requalification
- provider prompts, install, update, or publication

## Execution Plan

### Batch 104.1 — Identity Corpus

- [x] Execute card 320.
- [x] record host `17.2.15` and npm `17.3.8` identity against `17.3.7`
- [x] name compatible-extension for card 321 without changing production
      claims

### Batch 104.2 — Claim And Acceptance

- [x] Execute card 321 after card 320 names compatible-extension.
- [x] raise `oh-my-pi.package` through exact `17.3.8`
- [x] keep baseline `17.2.9`, AllowUnverified, unpublished `17.3.6`, and
      behavior `oh-my-pi.rpc-v2-v17.2.9`
- [x] refresh matrices, guides, architecture, Contract 029 ceiling text,
      and focused Oh My Pi proof

## Acceptance Criteria

- [x] host `17.2.15` remains Qualified
- [x] official `17.3.8` classifies as Qualified Maintained
- [x] `17.3.9` remains permitted UnverifiedNewer
- [x] unpublished `17.3.6` stays unpublished
- [x] decoder and `17.3.7` specimens remain
- [x] focused Oh My Pi proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Decision Gates

- Stop if selected RPC evidence requires a new public operation.
- Stop if qualification depends on a provider prompt or harness install.
- Do not compile Antigravity or Gemini inside this milestone.

## Closeout

g03.104 is complete. Next Upgrade Workflow family is Antigravity
`1.1.15`. Gemini stays deferred.
