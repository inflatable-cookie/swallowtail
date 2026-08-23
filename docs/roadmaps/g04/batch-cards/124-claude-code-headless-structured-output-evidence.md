# 124 Claude Code Headless Structured Output Evidence

Status: planned
Owner: Tom
Created: 2026-08-23
Milestone: [g04.045 Claude Code Headless Structured Output](../045-claude-code-headless-structured-output.md)
Depends on: Research 121 and 175

## Goal

Freeze exact `2.1.238` Claude Code headless JSON Schema behavior, then define
the smallest schema, command, model, and result subset that can satisfy
Contracts 039 and 040 on `claude-code.headless`.

## Method

1. Freeze current official CLI/headless documentation and the exact
   `@anthropic-ai/claude-code@2.1.238` package identity, help declaration, parse
   path, schema handling, retry loop, stream renderer, result encoder, and exit
   path. Record retrieval dates and SHA-256 digests.
2. Prove whether `--json-schema` composes with the selected command:
   `-p`, text input, stream-JSON output, verbose, no session persistence,
   caller model, optional effort, Plan mode, `Read,Glob,Grep`, selected setting
   sources, and empty strict MCP config.
3. Identify the exact accepted JSON Schema dialect and keyword subset. Freeze
   valid, invalid, unsupported-keyword, malformed, and oversized-schema
   behavior without claiming an unproved draft.
4. Classify enforcement as exactly `ProviderNative` or `HarnessValidated`.
   Trace model-visible schema tools, validation ownership, structured-result
   production, and any post-validation. Prompt instructions alone do not
   qualify.
5. Freeze attempt and turn semantics for valid, initially invalid then valid,
   and unsatisfiable outputs. Determine whether any exact flag or package rule
   imposes a preflight-bindable maximum. Do not infer that `--max-turns` bounds
   validation attempts.
6. Freeze terminal `structured_output`, ordinary result text, `num_turns`,
   model, session, usage, exit status, malformed/duplicate result, null/missing
   structured output, and post-terminal event behavior.
7. Classify every currently qualified reasoning mode and the schema-absent
   path independently. Preserve activity, usage, deadline, cancellation,
   process termination, and joined cleanup truth.
8. Decide the exact facade point/private behavior revision and whether support
   is restricted to `2.1.238`. Earlier qualified and later unverified versions
   receive no capability by inference.
9. Replace Research 192's reservation with exact route/schema dispositions.
   Do not edit shared architecture, matrices, programme, indexes, changelog,
   or roadmap front doors.

No installation, login, credential/account inspection, paid prompt, provider
request, or live Claude operation is authorized. Exact package download and
secret-free local source/CLI inspection are allowed; do not install or replace
the host `claude` executable.

## Acceptance Criteria

- [ ] official and exact `2.1.238` sources and specimens are frozen with dates
      and digests
- [ ] selected-command composition and schema dialect/subset are explicit
- [ ] enforcement source, model-visible tool behavior, attempts, turns,
      retries, exit, null result, and terminal shape are explicit
- [ ] reasoning, schema-absent, activity, usage, cancellation, and cleanup
      compatibility are explicit
- [ ] facade revision and exact-version disposition are explicit
- [ ] Research 192 is promoted with a route/schema deliver-now table
- [ ] no production code, capability, guide, matrix, or compatibility claim
      changes during evidence
- [ ] `effigy validate:focused swallowtail-adapter-claude-agent` passes
- [ ] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [ ] `git diff --check` passes

Auto-continue to card 125 only when Research 192 admits at least one exact
schema subset with qualifying enforcement, an exact bounded attempt policy,
unambiguous non-null terminal structured output, and no contract gap.

## Stop Conditions

- the exact package contradicts current official docs or the selected command
- schema dialect/subset or validation ownership is ambiguous
- retries are non-zero without a preflight-bindable maximum
- failure can exit zero with null/missing structured output and cannot be
  converted into explicit run failure without inference
- existing route controls or cleanup weaken
- delivery needs a contract change, live provider proof, or compatibility
  inference

## Out Of Scope

- production binding, guide/matrix claims, or shared closeout surfaces
- response-only, ACP, another version range, another Claude flag, or live work

