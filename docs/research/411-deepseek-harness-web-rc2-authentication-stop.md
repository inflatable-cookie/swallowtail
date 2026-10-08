# Research 411 DeepSeek Harness Web RC2 Authentication Stop

Status: currentness stop at the browser-authentication boundary
Owner: Swallowtail
Date: 2026-10-08

## Finding

The npm `latest` and `next` tags both point to `@deepseek-ai/dsh@0.2.0-rc.2`.
The `alpha` tag points to `0.2.1-alpha.1`. The RC channel published
`0.1.0-rc.7`, `0.1.0-rc.8`, `0.1.1-rc.1`, `0.1.1-rc.2`, and `0.1.2-rc.1`
after the existing `0.1.0-rc.6` claim. Research 411 qualifies the first four
points and preserves `0.1.0-rc.6`. The first security and authority break is
`0.1.2-rc.1`; no point at or after that release is qualified. Later alpha
versions remain outside the selected RC channel.

The exact npm tarball identities, npm SHA-512 integrities, SHA-256 tarball
digests, and sorted file SHA-256 inventories are retained in
[`dist-inventory.json`](../../crates/swallowtail-adapter-deepseek-harness/tests/fixtures/deepseek-harness-web-0.2.0rc2-stop/dist-inventory.json).
The inventory contains complete trees for 51 relevant package artifacts and
1,320 package files. It includes the six RC points through the stop and the
current `0.2.0-rc.2` route-support packages. Registry metadata contains no
`gitHead`; the source repository and package directories are recorded, but
an exact source commit cannot be bound to these published tarballs. Package
artifacts, rather than an inferred Git revision, are the identity authority.

## Selected RC hop review

The complete before-and-after file hashes and changed-path classifications for
the pre-stop route-support package hops are in the inventory. The selected
surface review found:

| Hop | Selected-surface classification |
| --- | --- |
| `0.1.0-rc.6` → `0.1.0-rc.7` | The client settings error variant and image `maxImageDimension` schema change do not enter the 11-method allowlist. The package's default profile files are outside the adapter mapping. |
| `0.1.0-rc.7` → `0.1.0-rc.8` | `session.create.reuseWorkspaceBlank` is an optional addition the adapter does not send. New image and file-reference remotes remain outside the allowlist. The web app now opens the host's default browser by default; the route passes its documented `--no-open` option from this release onward. |
| `0.1.0-rc.8` → `0.1.1-rc.1` | The credential remote-event name changes and the web server adds structured index injection. Neither adds a Swallowtail method or changes the admitted HTTP/WebSocket paths. Product prompt and static UI changes remain downstream. |
| `0.1.1-rc.1` → `0.1.1-rc.2` | The HTTP bridge body ceiling rises from 160 MiB to 300 MiB; the adapter keeps its 256 KiB request bound. The optional `reuseWorkspaceBlank` field is removed; the adapter never sends it. The selected request method set and WebSocket downlink remain available. |
| `0.1.1-rc.2` → `0.1.2-rc.1` | Stop. The connection adds persistent browser authentication and the web app changes its API composition. This changes the approved access and transport boundary. |

At `0.1.0-rc.8`, the Web app's published `--no-open` option maps
`openBrowser` to false. The route adds that option only for `0.1.0-rc.8`,
`0.1.1-rc.1`, and `0.1.1-rc.2`. It keeps the original arguments for `0.1.0-rc.6`
and `0.1.0-rc.7`, which do not publish the option. This preserves the
existing owned-process lifecycle and prevents an adapter-owned run from
launching a host browser as a side effect.

## Currentness stop and exact adaptation needed

At `@deepseek-ai/dsh-client-connection@0.1.2-rc.1`,
`package/lib/index.js` introduces `BrowserAuth`. It reads or creates
`credentialKey("client-connection", "browser-session")` through
`credentials.modifyRecord`, signs browser-session cookies, and returns HTTP
401 for `/api` requests without a valid authenticated session. The `0.2.0-rc.2`
artifact retains this credential-backed flow. The published web app also
removes `dsh-host-apiproxy` from its bundle composition and adds typed session
and workspace controller packages. The DSH client-connection artifacts remove
the dedicated `events.mux` and `events.host` path definitions; the current
Gateway uses authenticated Typert Remote streams over the `/api/remote.mux`
WebSocket. The typed controller and stream lifecycle, including cancellation,
remains unqualified. This is separate web-axis evidence; JSON-RPC evidence does
not transfer.

The existing prepared route contract declares local unauthenticated access
with `CredentialState::NotRequired` and no credential lease. Swallowtail has
no authorized path to create or retrieve the DSH browser-session credential.
Using the new API would change security authority and event transport. A
ruling must authorize either an adapter-private DSH-managed browser-session
adaptation with an explicit credential and process-authority boundary, or a
consumer-visible contract change. If authorized, qualify the typed controller
and stream paths with deterministic fakes, including authentication failure,
session lifecycle, and cleanup. Do not claim the current target until those
boundaries are approved and tested.

The Web claim keeps ID `deepseek-harness.web-rc6-1`, facade revision
`deepseek-harness.apiproxy-v1`, and `QualifiedOnly`. Its exact maintained
points are `0.1.0-rc.6`, `0.1.0-rc.7`, `0.1.0-rc.8`, `0.1.1-rc.1`, and
`0.1.1-rc.2`. Published `0.1.2-rc.1` and later RCs remain unqualified pending
adaptation. All alpha versions remain excluded. The JSON-RPC runtime-bin and
Web package axes remain separate.

## Verification boundary

The inspection used public npm registry metadata and downloaded package
tarballs. No package was installed or executed. No `dsh` version probe, web
server, provider prompt, catalogue, session, credential, or host browser was
used. The new installed-version parser and launch-argument mapping are covered
by fixture tests; they do not perform a live provider action.

## Sources

- [npm package metadata](https://registry.npmjs.org/@deepseek-ai/dsh), including
  the official package version tags and tarball references frozen in the
  inventory.
- [Upstream source repository](https://github.com/deepseek-ai/deepseek-harness).
  npm metadata has no `gitHead`, so this record does not claim a source commit
  for any published package artifact.
