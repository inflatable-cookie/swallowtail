# 342 Claude Code 2.1.281 Narrowed Response-Only Claim

Status: promoted; g06.022 claim evidence.

Date: 2026-09-24

## Question

Can both Claude Code stream-JSON axes move through official `2.1.281` under
the Contract 039 narrowed built-in-hook ruling?

## Official latest

Re-probed npm `@anthropic-ai/claude-code` `latest` / `next` and GitHub latest
release. All three still name `2.1.281`. npm `stable` is `2.1.273` and is not
this family's channel. `2.1.279` and `2.1.282` remain unpublished. Downloaded
Research 341 artifacts under `/tmp/g06-018` were hashed and read; none were
executed. Host `claude` was not invoked.

## Switch candidates

Frozen `2.1.281` `agents-md` USER_CONFIG proves four `instructionFiles`
values: `claude-md`, `claude-md-or-agents-md` (default),
`claude-md-and-agents-md`, and `managed-only`. `claude-md` returns before
registering AGENTS.md hooks. `managed-only` filters `project` / `local` /
`user` instruction kinds. Those values are not pinned: the selected
`--settings` JSON path into plugin userConfig is not proven.

Frozen `telemetry` module reads `DISABLE_TELEMETRY` and
`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` as any non-empty string, and
`DO_NOT_TRACK` as `1` / `true` / `yes` / `on`. No settings-key opt-out is in
that module. Those env vars are not pinned: response-only
`EnvironmentRef` is opaque host-approved subscription state.

Results are frozen in
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.281/protocol.json`.

## Launch directory

Instruction files resolve from the process cwd. A consumer `Read` working
resource is that cwd. Without one, v1 (`2.1.227..=2.1.278`) keeps the
inherited process cwd. v2 (`2.1.280` onward, including `UnverifiedNewer`)
creates an adapter-owned empty temporary working resource and launches
there, so `AGENTS.md` / `CLAUDE.md` are not supplied from the host cwd.
That uses the existing `WorkingResourceService::create_temporary` host
service. It does not advertise `Capability::WorkingResource`.

## Claim

- Headless: compatible extension of `claude-code.headless.stream-json.v1` from
  `2.1.220..=2.1.278` to `2.1.220..=2.1.281`. Unpublished `2.1.279` joins the
  interior gap list. Watcher stays exact `2.1.251`. Feature-specific exact
  sets stay on their probed points.
- Response-only: private milestone. Same claim id
  `claude-code.response-only.window-1`. v1 `2.1.227..=2.1.278` keeps
  `stream-json.v1`. v2 `2.1.280..=2.1.281` is `stream-json.v2`. `2.1.279` is
  an explicit unpublished exclusion. Selected argv is unchanged across
  segments.
- `AllowUnverified` stays. Synthetic later stable is `2.1.282`.

## Sources

- [Research 341](./341-claude-code-2-1-281-builtin-hooks-and-qualification-stop.md)
- [Research 338](./338-claude-code-2-1-280-identity-stop.md)
- Frozen corpus: `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.281/`
- npm `latest` and GitHub latest re-probe 2026-09-24
