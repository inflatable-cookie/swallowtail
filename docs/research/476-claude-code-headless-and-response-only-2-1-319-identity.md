# Research 476: Claude Code headless and response-only 2.1.319 identity

Status: response-only extends through npm official stable `2.1.319` on its
existing `stream-json.v3` claim. Headless remains qualified through `2.1.317`;
the `2.1.319` permission setting changes its selected permission flow. The two
claims remain independent from each other, `claude-agent.acp`, and
`claude-agent.sdk`.

Headless axis: `claude-code.headless-stream-json`
Headless route: `claude-code.headless`
Headless claim: `claude-code.headless.window-1`
Previous headless ceiling: `2.1.317` on maintained `claude-code.headless.stream-json.v2`

Response-only axis: `claude-code.response-only-stream-json`
Response-only route: `claude-code.response-only`
Response-only claim: `claude-code.response-only.window-1`
Previous response-only ceiling: `2.1.317` on maintained `claude-code.response-only.stream-json.v3`

## Observation

Re-probed 2026-11-09. npm `latest`, npm `next`, and GitHub latest stable all
agree on `2.1.319`; npm `stable` remains the delayed channel at `2.1.311`.
The published hop set after `2.1.317` is exactly `2.1.318` and `2.1.319`.
There is no unpublished point between the previous ceiling and official
stable. `2.1.320` is absent from npm and has no GitHub tag.

| Version | npm publish time | npm wrapper tarball SHA-256 | GitHub tag | GitHub release publish time | GitHub commit |
| --- | --- | --- | --- | --- | --- |
| `2.1.318` | `2026-11-03T21:55:17.247Z` | `d86c0189406af48b393f658ea1c0ba9a8fd10dc3c461e0ad50887b465a18b83c` | `v2.1.318` | `2026-11-04T00:01:03Z` | `646b74189c735abbccf78ef5cb7ddd38a07a36eb` |
| `2.1.319` | `2026-11-06T00:58:56.366Z` | `733a926c19df1c84f35a8e4285a637edce7bf2de13672cfe13a1397efd0bf823` | `v2.1.319` | `2026-11-06T17:04:01Z` | `0ed94698aa0aeee471465fc9c867c2e8434c4d68` |

The wrapper and Darwin arm64 and Linux x64 npm packages were frozen for
`2.1.318` and `2.1.319`; each tarball matches the registry's SHA-512
integrity. Their per-file SHA-256 inventories and the selected marker census
are in the [2.1.319 corpus](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.319/README.md).
No downloaded executable was run, and no provider prompt, login, install, or
live session was used.

The selected `--input-format` and `--output-format` help lines remain
byte-identical. Both binaries retain the selected stream, result, permission,
hook, and fork markers. The tree inventory adds and removes no files. On both
platforms the wrapper changes only `package.json` and `sdk-tools.d.ts` on the
`2.1.317` to `2.1.318` hop. The next wrapper hop also changes `cli-wrapper.cjs`
and `install.cjs`. The CLI wrapper now checks the installed platform package
identity and refuses duplicate or unreadable manifests. This is failure-only
hardening for an ambiguous or damaged install. A copy selected by the previous
wrapper would still face Swallowtail's exact executable-version check before
provider work, so the new refusal keeps the same fail-closed route outcome for
those invalid install states. It leaves the selected argv unchanged for an
exact installed package. `install.cjs` applies the corresponding check during
postinstall, outside route execution. Neither file adds a CLI operation. Each
platform package changes its executable and `package.json` on both hops. The
`sdk-tools.d.ts` change is a declaration comment and neither CLI mapping imports
it.

## Hop classification

### 2.1.318

Compatible extension on both routes. Release notes report that a `turn.step`
mod hook can no longer make a stopped headless reply finish as success, and
that `-p` no longer reports success when an API error cuts off a reply while
such a hook runs. These are failure-reporting corrections on the existing
terminal result surface. The adapter already maps the result `is_error` flag
and error subtypes to provider failure; no event, result field, argv, permission
mode, approval path, or safe-mode rule is added. Headless never changes model
after its one prepared prompt, so the queued-message cancellation fix does not
apply. Response-only has no tools or hooks.

### 2.1.319

Response-only is a compatible extension on v3. It passes `--tools ""`,
`--safe-mode`, and an empty strict MCP configuration. The newly introduced
`permissions.denyInsteadOfPrompting` setting cannot affect a tool call because
the route exposes no tools. The selected init and text-result shape remain
unchanged. Raise only this claim through `2.1.319`; `2.1.320` is the first
unpublished point and remains `UnverifiedNewer`.

Headless stops at `2.1.317`. The `2.1.319` setting changes a selected
permission-flow outcome: when a selected `Read`, `Glob`, or `Grep` call would
wait for a permission prompt, the setting denies the call and lets the turn
continue. The route keeps user, project, and local settings and does not
override the new setting. It is therefore reachable on the existing
`--permission-mode plan` route and cannot be treated as an unmapped tool
payload or an unrelated interactive feature.

**Decision needed:** should the headless route preserve the new setting and
qualify its changed ask-to-deny behavior, explicitly suppress or pin it to
preserve the current permission contract, or give that behavior a distinct
adapter revision? Keep the headless ceiling at `2.1.317` until that choice is
settled and implemented or bounded.

## Decisions

- Keep the routes separate. The new permission setting is reachable in
  headless and unreachable in response-only, so one route can advance while
  the other remains stopped.
- Keep headless on the existing v2 revision and do not extend its optional
  Research 226 maximum-turn set.
- Extend response-only on its existing v3 revision through `2.1.319`; retain
  the baseline, deny-list, and earlier v1/v2 segments.

## Sources

- [Frozen fixture corpus](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.319/README.md)
- [npm package](https://www.npmjs.com/package/@anthropic-ai/claude-code)
- [GitHub `v2.1.318`](https://github.com/anthropics/claude-code/releases/tag/v2.1.318)
- [GitHub `v2.1.319`](https://github.com/anthropics/claude-code/releases/tag/v2.1.319)
- [Research 473](./473-claude-code-headless-and-response-only-2-1-317-identity.md)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
