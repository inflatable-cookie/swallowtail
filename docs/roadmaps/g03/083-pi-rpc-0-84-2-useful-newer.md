# 083 Pi RPC 0.84.2 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-18
Depends on: Research 127; Research 140; g03.082
Vision tags: maintained compatibility, exact interfaces, currentness
Contract refs: 001, 011, 023, 029, 032-033, 037, 039, 041, 044
Planning state: cards 254-255 completed

## Problem

Research 127 ranked remaining AllowUnverified families after Claude Agent
ACP. Pi RPC is exact published points `0.80.10` through `0.83.0` with
AllowUnverified. This host is already qualified `0.83.0`. Official npm
`latest` is `0.84.2`. Raising the bound is useful-newer support, not a
silent `latest` bump and not a flatten onto Oh My Pi.

## Generation Runway Goal

Freeze Pi RPC `0.83.0` / `0.84.2` identity against the `0.83.0` corpus,
then raise the qualified ceiling only if that evidence names a compatible
extension.

## Goals

- [x] rank remaining AllowUnverified registry-newer families and pick Pi RPC
- [x] freeze host `0.83.0` and official `0.84.2` identity and selected RPC
      source
- [x] raise the adapter claim through exact `0.84.2`, keeping unpublished
      `0.83.1` incompatible and adding private `0.84.0` message-update-delta
- [x] leave Qwen, Antigravity, and Gemini for later one-family work

## Non-Goals

- flattening Oh My Pi onto this axis
- mapping bash, switch_session, fork, clone, extensions, or load/resume
- mapping streaming `message_update.usage`
- bulk-bumping other AllowUnverified families
- Gemini requalification
- provider prompts, live RPC sessions, install, update, or publication

## Execution Plan

### Batch 83.1 — Identity Corpus

- [x] Execute card 254.
- [x] record host `0.83.0` and official `0.84.2` against exact `0.83.0`
- [x] name compatible-extension plus private `0.84.0` milestone for card
      255 without changing production claims

### Batch 83.2 — Claim And Acceptance

- [x] Execute card 255 after card 254 names compatible-extension.
- [x] keep exact `0.83.0` Deprecated and add Maintained `0.84.0..=0.84.2`
      message-update-delta
- [x] keep baseline, unpublished gaps including `0.83.1`, AllowUnverified,
      and synthetic later `0.84.3`
- [x] refresh matrices, guides, contracts that name the ceiling, and
      focused Pi proof

## Acceptance Criteria

- [x] host `0.83.0` remains Qualified
- [x] official `0.84.2` classifies as Qualified Maintained
- [x] unpublished `0.83.1` remains incompatible
- [x] `0.84.3` remains permitted UnverifiedNewer
- [x] decoder specimen remains `pi-rpc-0.80.10`
- [x] focused Pi proof and package verify pass
- [x] matrices and guides name the new package ceiling

## Decision Gates

- Stop if exact artifact identity cannot be corroborated from official
  sources.
- Stop if selected mapped RPC protocol differs from the recorded evidence.
- Stop if qualifying the current official point would require a provider
  prompt or live RPC session.

## Closeout

g03.083 is complete. Next family is Qwen headless (registry newer; host
already on a qualified bound). Gemini stays deferred.
