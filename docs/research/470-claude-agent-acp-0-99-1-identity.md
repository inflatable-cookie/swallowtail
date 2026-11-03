# Research 470: Claude Agent ACP 0.99.1 Identity and Currentness

Evidence checked 2026-11-03 for `claude-agent.acp` only. Claude Code and
`claude-agent.sdk` are separate families and are unchanged here. This record
covers the middle hop between `0.99.0` (Research 469) and official `0.100.0`
(Research 471).

## Official current point

Official npm `latest` is `0.100.0`, re-probed at the start of the run. This
hop's own official point was current when published: `0.99.1` reached npm at
2026-10-29T19:37:04Z. The dispatch named hops `0.99.0` and `0.100.0`;
`0.99.1` is also a published stable between them, so the run walks it rather
than leaving an unqualified interior point inside the extended segment.
GitHub release `v0.99.1` is not a draft or prerelease, published
2026-10-29T19:34:40Z at `0f78dbbdd089e5c64fc908173e8656aa9cc757fb`. npm
`gitHead` matches the tag. The exact npm integrity is
`sha512-tK5lNtoI85IS0K8PVKYK86t/ARgCS1AjEw+u6QSNmdu0tbhyeB6FTNzFnGNIgsL/3xYwrQUMFDC6osT1aIPReQ==`;
the downloaded tarball SHA-256 is
`18629662c6940585ee2344329a7f46faf9ab1419de998a6a4f94c5224ae65f5c`.
The package pins ACP SDK `2.1.0` and Claude Agent SDK `0.3.312` — identical
to `0.99.0` — requires Node `>=22`, and exposes `dist/index.js` as its binary
entry. Host `claude-agent-acp` was absent from `PATH` in the observation
environment; that observation belongs to that environment and is not a claim
of universal absence.

## Published stable hop

| Version | Published | GitHub tag commit | ACP SDK | Claude Agent SDK |
| --- | --- | --- | --- | --- |
| `0.99.1` | 2026-10-29 19:37:04Z | `0f78dbbdd089e5c64fc908173e8656aa9cc757fb` | `2.1.0` | `0.3.312` |

The retained fixture records every integrity value, tarball SHA-256, package
dependency identity, and complete file tree in
[`claude-agent-acp-0.100.0`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.100.0/README.md).
`0.99.0` to `0.99.1` adds no files, removes none, and changes 12. 276 files
are unchanged, including `dist/index.js`, `dist/serve.js`, `dist/tools.js`,
and the whole `session-index` directory. `package.json` changes version
metadata only.

## Selected ACP v1 review

- Login-required text detection broadens: the synthetic login assistant
  message, goal-command failures, and turn failures now match a `run /login`
  pattern instead of one sentence, and usage-limit banners are excluded. This
  is the provider auth surface, which Swallowtail does not exercise; where it
  changes a turn failure into `authRequired`, the adapter still maps the JSON-RPC
  error to its one rejected-request failure class. Replay filtering of those
  synthetic messages changes which provider-authored text a resume replays,
  not any wire shape.
- Skill-file refresh (`getContextUsage().skills.skillFrontmatter`) feeds
  `available_commands_update` skill paths and an AIR-only `_meta` kind marker.
  Every entry point is gated on the AIR capability Swallowtail does not
  advertise: `refreshSkillFiles` returns before any read, `skillFiles` stays
  undefined, and the command list resolves through the unchanged non-AIR
  fallback. The v1 `available_commands_update` content is unchanged for this
  client.
- Approved-plan `plan_update` ids gain an AIR review suffix, gated on the plan
  client capability Swallowtail does not advertise.
- Fork (`dir` scoping) and native-subagent label internals sit behind the AIR
  fork `_meta` surface and the native-subagent capability Swallowtail does not
  select.

## Decision and limits

This is a compatible extension of `claude-agent.acp.initialize-meta-extensions-v7`,
recorded per hop and applied together with Research 469 and 471 in one claim
revision that extends the maintained segment to `0.66.0..=0.100.0` through
`0.100.0`, keeping baseline `0.53.0`, claim id `claude-agent.acp.window-2`,
the seven behavior milestones, exclusions `0.52.0` and `0.58.0`, and
`AllowUnverified`. Until that claim revision lands, `0.98.0` stays the
qualified ceiling and `0.99.1` stays `UnverifiedNewer` under
`AllowUnverified`. `0.100.1` is the synthetic `UnverifiedNewer` point after
the claim revision. Prerelease points, including the `0.99.1-preview.*` and
`0.99.2-preview.*` lines, remain incompatible without their own exact
segment.

No new public mapped operation was added. HTTP MCP live honouring remains
exact to `0.79.0` and `0.81.2` (Research 361 and 364). The root package pin
stays `0.81.2`. The pinned ACP SDK `2.1.0` and Agent SDK `0.3.312` are
recorded dependency identity on separate axes. No tag, publication, host
install, package install, provider prompt, live initialize, credential use, or
host mutation occurred.

Sources: [official npm package 0.99.1](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.99.1),
[official GitHub tag v0.99.1](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.99.1),
and the [ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json).
