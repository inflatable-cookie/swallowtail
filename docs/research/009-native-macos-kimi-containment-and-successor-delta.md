# 009 Native macOS Kimi Containment And Successor Delta

Status: promoted
Owner: Tom
Updated: 2026-07-20

## Supersession Notice

Research 010 supersedes this record's Kimi successor/version delta. The
existing `0.28.1` corpus already targets the maintained TypeScript Kimi Code
successor; `0.26.0` was an older documentation entry, not its replacement.
The native macOS App Sandbox selection and bookmark-handoff proof gap below
remain active only as platform evidence. Research 011 proves the handoff for a
compatible helper but finds the exact Kimi `0.28.1` artifact runtime
incompatible with the documented inherited-helper signature. Research 012
then makes containment optional and reopens the Kimi lane under explicit
ambient execution. Do not use the superseded version conclusions or the
unqualified mechanism selection for planning.

## Question

Which first deployment-owned containment mechanism best fits a seamless Kimi
experience, and does the existing Kimi ACP evidence still target the current
maintained production line?

## Operator Preference

Prefer native, seamless startup over a heavy container. The operator accepts
the recommended first platform and mechanism.

## Recommendation

Select a deployment-owned macOS App Sandbox launcher/helper as the first
Contract 017 proof. Do not select a container or VM for the first slice.

The intended user experience is:

- the app bundles or resolves one pinned Kimi executable
- the user selects a project root once
- the app persists that authority with a security-scoped bookmark
- Kimi uses an isolated app-owned `KIMI_CODE_HOME`
- later sessions start as ordinary child processes without container setup,
  images, mounts, daemons, or a second filesystem
- OAuth or other delegated login occurs once inside the isolated Kimi state;
  Swallowtail never extracts the resulting credential

## Apple Evidence

Apple documents App Sandbox as the supported macOS boundary for limiting file,
network, and other resource access. A command-line helper can be embedded in a
sandboxed application, including independently distributed Developer ID apps.
The embedded helper is signed with App Sandbox plus inheritance entitlements.
Child processes, including system shell children, inherit the parent's
sandbox.

User-selected read-only or read-write folders can be granted through standard
system interaction and persisted with security-scoped bookmarks. A folder
grant extends recursively below that selected folder. The app must stop the
security-scoped access when work finishes.

One proof gap remains material: static helper inheritance does not by itself
prove propagation of a dynamic PowerBox or bookmark grant. The launcher must
resolve and hand off the exact bookmark authority in a form that Kimi and every
descendant inherit. Card 057 must demonstrate this on the supported macOS
mechanism before Swallowtail adds a portable containment lease or claims
bounded workspace access.

Temporary exception entitlements, deprecated `sandbox-exec`, private Seatbelt
profiles, and broad home-directory access remain prohibited.

## Superseded Kimi Delta

The earlier Kimi corpus targets the legacy Python `kimi-cli` line pinned in
Research 006. Current official evidence now presents a successor Kimi Code
implementation:

- the legacy repository says it is being wound down and migrates users to Kimi
  Code
- the successor is distributed as a fast single binary and retains `kimi acp`
- official release notes list Kimi Code `0.26.0` on 2026-07-16
- the successor supports project sessions, background work, subagents, shell
  execution, plugins, MCP, and built-in file tools
- all runtime data, including OAuth state and sessions, can be redirected with
  `KIMI_CODE_HOME`

The current official `kimi acp` reference is materially more precise than the
legacy corpus:

- the subprocess prints no banner, uses JSON-RPC over stdin/stdout, and keeps
  logs on stderr
- initialization advertises image and embedded-context prompts, HTTP and SSE
  MCP forwarding, `session/load`, and session listing
- file reads and writes use ACP client callbacks
- ACP terminal reverse RPC is absent; shell commands execute locally
- only one unstable method, model selection, is documented as implemented
- the documentation identifies ACP `0.23`; SDK package, schema release, wire
  version, and Kimi artifact version must still be pinned independently

The first successor slice must not preserve legacy resume merely for API
symmetry. If current tagged evidence advertises load but no separate resume,
the driver claims load only. MCP forwarding, embedded context, images, session
listing, unstable model selection, plugins, background work, and subagents stay
excluded until their own authority and conformance work exists.

This strengthens the containment need and invalidates direct production use of
the old `0.28.1` provider pin. Shared ACP framing and the load/resume/write
contract may remain reusable, but the exact successor schema, capabilities,
session semantics, source hashes, access route, and exclusions require a new
fixture delta before card 058.

`0.26.0` is an evidence snapshot, not a floating dependency. Swallowtail will
not run `kimi upgrade`, accept an arbitrary executable found on `PATH`, or
silently treat a later release as compatible. Each supported upgrade needs a
new exact artifact digest, source and schema comparison, fixture pass, adapter
version or configured-instance revision, and explicit support evidence.

## Selected Proof Boundary

Card 057 may resume as a native macOS mechanism proof with:

- app-sandbox and helper-inheritance signature checks
- one exact user-selected root and `Read` or `ReadWrite` bookmark grant
- an isolated app-container Kimi state root
- descendant probes for direct file access, shell access, links, renames,
  inherited descriptors, background work, and cleanup
- network authority modeled separately
- unsupported platform, signature, entitlement, bookmark, root, or access
  mismatch rejected before child launch
- process and reader join before bookmark, resource, or containment release

Container, VM, remote-host, Linux, and Windows implementations remain outside
this first mechanism proof.

## Promotion

- durable qualification and lifecycle boundary: Contract 017, unchanged
- selected mechanism and delivery state: roadmap 018 and card 057
- successor provider revalidation: card 065 before production mapping
- active repository task: card 063 remains first, per the earlier operator
  sequencing decision

## Primary Sources

- [Apple: Protecting user data with App Sandbox](https://developer.apple.com/documentation/security/protecting-user-data-with-app-sandbox)
- [Apple: Embedding a command-line tool in a sandboxed app](https://developer.apple.com/documentation/xcode/embedding-a-helper-tool-in-a-sandboxed-app)
- [Apple: Accessing files from the macOS App Sandbox](https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox)
- [Apple: Enabling Security-Scoped Bookmark and URL Access](https://developer.apple.com/documentation/professional-video-applications/enabling-security-scoped-bookmark-and-url-access)
- [Kimi Code CLI command reference](https://www.kimi.com/code/docs/en/kimi-code-cli/reference/kimi-command)
- [Kimi Code ACP reference](https://www.kimi.com/code/docs/en/kimi-code-cli/reference/kimi-acp)
- [Kimi Code data locations](https://www.kimi.com/code/docs/en/kimi-code-cli/configuration/data-locations.html)
- [Kimi Code built-in tools](https://www.kimi.com/code/docs/en/kimi-code-cli/reference/tools.html)
- [Kimi Code changelog](https://www.kimi.com/code/docs/en/kimi-code-cli/release-notes/changelog.html)
- [MoonshotAI Kimi Code repository](https://github.com/MoonshotAI/kimi-code)
- [MoonshotAI legacy Kimi CLI repository](https://github.com/MoonshotAI/kimi-cli)
