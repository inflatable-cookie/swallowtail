# 2026-08-29 Process Containment Host Composition

Status: promoted; g05.003 card 014
Owner: Tom
Lane: g05.003
Source: post-PR 117 containment checkpoint

## Fixed Boundary

Research 259 and Contract 059 keep the hard no-outliving invariant. The
default macOS process host, process groups, `launchd`, output pipes, and
process-table observation do not qualify. Card 010 cannot start against the
recording test backend or the public injection seam alone; one real host
composition must own descendant containment and prove empty scope.

## Viable First-Proof Shapes

### Opt-In Container Or VM Supervisor

Run each accepted watcher operation inside one host-owned container or VM
scope. The backend owns create, start, graceful stop, force-stop, terminal
observation, empty-scope proof, and supervisor join. Workload processes must
not receive the control socket or another path out of the owned scope.

This is the only current candidate that can serve a macOS desktop host without
claiming a native Darwin containment primitive. It adds an external runtime,
resource-mount, image, startup-cost, and availability boundary. It must remain
opt-in and exact; operating-system detection cannot enable it.

### Linux Cgroup V2

Place the workload in one delegated cgroup before public watcher identity,
deny workload migration out of the delegated scope, target stop through the
owned cgroup, and prove `populated=0` after termination. The proof must cover
concurrent forks, authority to create and manage the scope, controller
delegation, cleanup after partial start, and environments where cgroup
authority is absent.

This is the smallest kernel-backed server composition, but it does not make
the current macOS development host watcher-capable and may need a privileged
or service-manager-owned delegation boundary.

### Windows Job Objects

Create and bind a non-breakaway job before the watcher id returns, target
termination through the job handle, observe job completion, and prove the job
empty before handle release. The proof must cover nested-job behavior,
breakaway flags, partial start, assignment races, and exact supported Windows
versions.

This is a strong desktop-native candidate, but it does not serve the current
macOS host and needs Windows-specific implementation and execution evidence.

## Operator Decision Needed

Does the first production watcher proof need to work from the current macOS
desktop environment?

- If yes, compile an opt-in container/VM containment research-and-proof card.
- If no and Linux deployment is the first target, compile the cgroup-v2 card.
- Choose Job Objects only when Windows is the first target.

Recommendation: use the opt-in container/VM route when macOS usefulness is a
first-proof requirement; otherwise prefer Linux cgroup v2 as the narrowest
kernel-owned composition. Do not start card 010 until the selected composition
is implemented and proved.

## Operator Decision

On 2026-08-29 the operator selected macOS via an opt-in OCI composition for the
first proof. The current host already exposes Docker Desktop `4.87.0`, Docker
Engine `29.7.2`, API `1.55`, containerd `2.2.5`, and runc `1.3.6`; no install or
update is required. Card 014 narrows the candidate to the exact Docker Engine
API composition and may return an honest empty result. This observation is not
a compatibility range or production capability claim.

## Disposition

Promoted into g05.003 card 014 and the Research 260 assignment. Keep cards
010-011 gated until the exact composition is positively proved and a separate
implementation card lands. This note is not execution authority.
