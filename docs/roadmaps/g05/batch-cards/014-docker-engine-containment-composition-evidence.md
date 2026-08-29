# 014 Docker Engine Containment Composition Evidence

Status: ready
Owner: Tom
Created: 2026-08-29
Updated: 2026-08-29
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: completed card 009; Research 259; operator-selected macOS-via-OCI target
Research: 260

## Goal

Determine whether one exact Docker Engine API composition on the current
macOS host can satisfy Contract 059 process containment strongly enough to
justify a production backend card.

## Scope

1. Freeze the exact official Docker Engine API, OCI lifecycle, Docker Desktop
   bind-mount, private namespace, restart, stop, kill, wait, inspect, and
   remove semantics used by the candidate.
2. Record prompt-free local identity for the existing Docker Desktop, Engine,
   API, containerd, and runtime. Do not infer compatibility beyond the exact
   observed segment.
3. Run bounded live containment probes through the host-owned engine endpoint
   using only a pre-existing cached image and uniquely labelled temporary
   containers. Do not pull, build, push, login, install, or update.
4. Prove create-before-start binding, private process scope, no restart,
   closed-pipe `setsid` descendants, concurrent forks during stop, graceful
   stop, force-stop, wait, stopped-state inspection, resource removal, and
   partial-start cleanup. Stop and cleanup target only the owned container id;
   numeric process ids are observation evidence, never authority.
5. Trace how host-approved executable, environment, working-resource, image,
   mount, user, network, output, and engine-endpoint policy would map into the
   existing `ProcessContainmentBackend` seam without granting model or portable
   caller authority.
6. Return a positive exact composition or an honest empty result in Research
   260, including package ownership, capability discovery, failure taxonomy,
   validation needs, and the smallest follow-on implementation boundary.

## Output

Research 260, sanitized frozen evidence, a small identity fixture/test when it
materially improves reproducibility, and the assigned lane log only. Do not
add a production Docker dependency, backend, watcher route claim, public API,
or card 010 wiring.

## Acceptance Criteria

- [ ] exact Docker Desktop, Engine, API, container runtime, and cached image
      identity are recorded without widening a version claim
- [ ] create binds an opaque owned container identity before workload start
- [ ] workload cannot receive the engine socket, host PID namespace,
      privileged mode, host cgroup authority, or restart policy
- [ ] a child that calls `setsid` and closes output remains in the owned scope
      and cannot outlive stop/kill plus empty-scope proof
- [ ] concurrent forks cannot survive the owned container teardown
- [ ] wait plus stopped-state inspection and removal give an exact durable
      empty-scope/join proof, or the candidate is rejected
- [ ] partial create/start/control failures retain enough identity for bounded
      cleanup and truthful failure
- [ ] image, mounts, working resource, user, environment, network, and command
      selection remain host-approved and absent from watcher control data
- [ ] no broad container prune, image/volume deletion, provider work, or
      unrelated running container is touched
- [ ] Research 260 returns either one implementable exact composition or an
      honest empty set that keeps card 010 gated

## Validation

- `effigy validate:focused swallowtail-host-local` when a fixture/test changes
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- exact live-probe commands and sanitized results recorded in Research 260

## Stop Conditions

- the proof needs privileged mode, host PID/cgroup namespaces, workload access
  to the engine socket, ambient Docker configuration mutation, image pull,
  registry login, network fetch, install, or update
- container stop, wait, inspect, and remove cannot prove that all descendant
  work is terminal and the owned scope is destroyed
- a `setsid`, closed-output, or fork-race workload can survive the container
  lifecycle
- correctness depends on signalling numeric host PIDs, parsing human CLI text,
  or trusting a runtime/product name without exact capability evidence
- the existing host seam cannot carry the composition without provider-,
  route-, consumer-, command-, path-, or engine-specific portable vocabulary
- a usable composition requires arbitrary model-supplied commands or host paths

## Auto-Continuation

No. Return Research 260 and one reviewable PR. A positive result returns to the
orchestrator to compile a separate implementation/conformance card. Keep cards
010-011 gated.
