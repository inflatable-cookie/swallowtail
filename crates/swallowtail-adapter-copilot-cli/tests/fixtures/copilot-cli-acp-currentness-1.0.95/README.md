# Copilot CLI ACP artifact hops through 1.0.95

This ledger freezes the official npm wrapper and Darwin ARM64 package for the
qualified baseline `1.0.80` and every published stable through the current
`1.0.95`. It records registry publication metadata, tarball SHA-256/SRI,
complete sorted file trees, per-hop file deltas, and a conservative selected
surface classification. The metadata observation is
`2026-10-10T13:59:23Z`; `1.0.96-2` was the prerelease at that observation.

The previous frozen `1.0.80`, `1.0.81`, and `1.0.93` trees are reused without
re-downloading their tarballs. Every other stable package was downloaded from
its exact `registry.npmjs.org` tarball URL. Archive SHA-256, npm SHA-1 and
SHA-512 integrity, package entry count, unpacked byte count, and every member's
path, mode, size, and SHA-256 were checked. No package was installed or run.

The wrapper has four files at every point. `npm-loader.js` is byte-identical
throughout (`0ea824a86be5757533fdb092eff7050871bd7a711a46babde0ffe0e44ac5ad88`);
the root `package.json` changes on each hop. The native package has 241 files
at `1.0.80`, 225 at `1.0.81`, 224 at `1.0.82`, 228 at `1.0.83`, 232 at
`1.0.84`, then four files from `1.0.85` onward. The `1.0.84` to `1.0.85`
repackaging removes 228 files and leaves a changed opaque `copilot` executable.
That executable changes on every later stable hop through `1.0.95`.

Each hop's exact added, removed, changed, and identical path sets are in
`artifact-hop-ledger.json`. The wrapper loader remains identical, but each
native hop changes runtime files. Changed application code and native
executables are classified as unresolved for selected ACP behavior. The
post-`1.0.84` package exposes no source file for the changed monolithic
executable. These are evidence stops, not compatible-extension findings.

The production claim remains exactly `1.0.80 QualifiedOnly`. Research 439's
accepted cancellation evidence remains limited to its exact `1.0.93` attempt;
the direct-native launch used `--model auto --acp --stdio`, unlike production
`--acp --stdio`, and its attributed action did not match the sentinel edit.
Nothing here transfers permission, edit, or lifecycle behavior to another
version or closes that launch-alignment gate.
