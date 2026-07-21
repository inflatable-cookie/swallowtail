# 007 Harness Process Filesystem Containment Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-20

## Question

Can Swallowtail's current local process host enforce Contract 017's complete
filesystem boundary for a harness and its child processes without ambient
privilege, container, VM, application-signing, or deployment authority?

## Method

Current Linux kernel Landlock documentation, the maintained Rust Landlock
binding, current Apple App Sandbox documentation, the installed macOS 26.5.2
`sandbox-exec(1)` manual, and current Microsoft process-sandbox documentation
were checked on 2026-07-20.

No sandbox profile was applied. No process was launched under a candidate
mechanism. No provider binary, credential, login, or external inference route
was used.

## Qualification Boundary

A mechanism can satisfy Contract 017 only when authoritative evidence and
deterministic tests prove all of these properties:

- the exact resource root, `Read` or `ReadWrite` access, operation scope, and
  execution host are fixed before process start
- every filesystem read or mutation path outside that authority is denied,
  including inherited descriptors and alternate syscall families
- restrictions survive `exec` and apply to every descendant
- symlink, hard-link, rename, mount, special-filesystem, and pre-opened-handle
  behavior is explicit
- unsupported platform, kernel, mechanism version, or configuration fails
  before provider work; best-effort degradation is not acceptable
- process join finishes before containment or resource authority is released
- network containment remains a separate policy and proof

Working directories, callback mediation, provider permission posture, and
provider conventions do not meet this boundary.

## Linux Landlock

Landlock is a stable kernel interface designed for unprivileged self-
restriction. A ruleset applies to the enforcing thread and its future
children. Path-beneath rules, stacked restrictions, runtime ABI discovery, and
deny-by-default handled rights make it useful defense in depth.

Landlock alone does not satisfy Contract 017:

- descriptors opened before enforcement are outside its path-open checks
- current documentation lists `stat`, `chmod`, `chown`, `setxattr`, `utime`,
  `fcntl`, `access`, and other syscall families that cannot be restricted
- special files reachable through `/proc/<pid>/fd` are not completely covered
- older ABIs cannot deny all required operations, including truncation before
  ABI 3
- the kernel may omit or disable Landlock, and upstream recommends best-effort
  compatibility while Swallowtail requires hard failure

Landlock may strengthen a deployment-owned sandbox. It is not complete
one-root filesystem containment for an arbitrary coding harness.

## macOS

Apple's supported App Sandbox uses entitlements sealed into code signatures.
Embedded command-line tools inherit the containing sandbox's static rights;
different helper rights require an XPC service, helper app, or another
deployment-owned component.

The current host still ships `/usr/bin/sandbox-exec`, but its installed manual
marks it deprecated and directs developers to App Sandbox. A private or
deprecated Seatbelt profile is not a supportable Swallowtail production
contract.

The current unsandboxed Rust CLI therefore cannot attach a supported dynamic
one-root sandbox to Kimi. App Sandbox would require application packaging,
signing, entitlements, helper layout, and resource-grant policy owned by the
deploying host application.

## Windows

Microsoft now documents composable process-sandbox APIs with AppContainer and
Bound File System read-only/read-write paths. The APIs are explicitly
experimental and subject to change. Existing AppContainer profiles are
per-user application identities with lifecycle and deployment implications.

This is promising future evidence, not a stable cross-platform contract or a
mechanism that can be claimed by the current macOS local host.

## Topology Result

- current local macOS host: unsupported
- Linux Landlock alone: incomplete, defense in depth only
- Windows experimental process sandbox: not stable authority
- remote or deployment-owned contained host: potentially supportable only
  after that host proves its exact mechanism and returns matching authority
- container, VM, signed App Sandbox helper, or equivalent launcher: outside
  current Swallowtail authority unless the operator expands execution-host
  scope

A local mechanism cannot represent remote containment. A deterministic remote
fixture may test identity and failure semantics, but it cannot prove an
unimplemented remote sandbox.

## Recommendation

Do not add speculative containment records or a Kimi production driver.
Landlock-only support would overstate the existing filesystem contract, and
`sandbox-exec` would rely on a deprecated surface.

Pause roadmap 018 after its validated protocol corpus. The operator must
choose one of two product directions:

1. authorize a deployment-owned contained execution host, select its first
   target platform, and compile a mechanism-specific proof before Kimi
2. defer Kimi production work and resume the next independent coverage lane,
   currently host-owned ephemeral llama.cpp

## Risks

- a defense-in-depth mechanism may be mistaken for complete containment
- a broad read allowlist for runtimes, libraries, configuration, or delegated
  auth can expose data to provider tools
- inherited descriptors can bypass path restrictions
- platform-specific sandbox availability can drift independently of Rust code
- a remote host claim without mechanism evidence would turn topology identity
  into false authority

## Primary Sources

- [Linux Landlock userspace API](https://docs.kernel.org/userspace-api/landlock.html)
- [Rust Landlock `0.4.4` changelog](https://docs.rs/crate/landlock/0.4.4/source/CHANGELOG.md)
- [Apple: Protecting user data with App Sandbox](https://developer.apple.com/documentation/security/protecting-user-data-with-app-sandbox)
- [Apple: Discovering and diagnosing App Sandbox violations](https://developer.apple.com/documentation/security/discovering-and-diagnosing-app-sandbox-violations)
- [Apple: Embedding a command-line tool in a sandboxed app](https://developer.apple.com/documentation/xcode/embedding-a-helper-tool-in-a-sandboxed-app)
- [Apple: Understanding the code signature](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/AboutCS/AboutCS.html)
- [Microsoft: Create Process in Sandbox](https://learn.microsoft.com/en-us/windows/win32/secauthz/createprocessinsandbox)
- [Microsoft: CreateAppContainerProfile](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-createappcontainerprofile)

## Promotion

- durable qualification and failure rules: Contract 017
- realized unsupported-host boundary: system architecture
- delivery state: roadmap 018 and card 057
- selected mechanism and successor delta: Research 009

Research 012 later limits this qualification boundary to routes that claim
provider- or host-enforced isolation. The platform findings remain valid but
do not block an explicit ambient harness route.

## Decision Resolution

On 2026-07-20 the operator selected the recommended deployment-owned native
macOS App Sandbox helper and preferred it over a container for seamless use.
Research 009 records the exact candidate shape and the remaining dynamic
bookmark-handoff proof. This record's rejection of a local-only dynamic
mechanism remains unchanged.
