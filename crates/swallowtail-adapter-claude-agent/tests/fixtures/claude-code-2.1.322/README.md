# Claude Code response-only 2.1.321 and 2.1.322 identity

This corpus extends Research 477 through official npm latest and GitHub stable `2.1.322`. It records complete wrapper, Darwin arm64, and Linux x64 package inventories for the published `2.1.321` and `2.1.322` hops, exact tarball integrity, extracted file hashes, and selected static marker counts. The earlier `2.1.319` files and evidence are retained by reference.

`2.1.320` was not published to npm and has no GitHub tag, so it remains an excluded gap. `2.1.323` is the first unpublished point above the new ceiling and remains `UnverifiedNewer`.

The wrapper launch shim, `cli-wrapper.cjs`, and `install.cjs` are unchanged from `2.1.319`; the SDK declaration change is unrelated. Selected stream-json help lines and marker presence remain stable. The `2.1.321` Linux NFS note corrects silent transcript-loss success to a process failure; this maps to the existing typed process-failure outcome. The hook-counter note requires Bash or PowerShell calls, which this empty-tools route cannot make. The `2.1.322` Vertex fast-mode change is outside this local CLI route. No downloaded executable was run, and no provider prompt, login, installation, or live session was used.

Response-only keeps its independent `stream-json.v3` claim through `2.1.322`. Headless remains at `2.1.317`; this task does not change its claim or implement its separate adaptation lead.
