# 008 Owned llama.cpp Serving Lifecycle Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-20

## Question

What is the smallest current llama.cpp lifecycle that can prove host-owned
ephemeral model serving without giving Swallowtail model acquisition, model
selection, persistent serving, or Monkey authority?

## Method

The current llama.cpp release, tagged server documentation, server bind source,
health and model-catalogue behavior, and release artifact metadata were checked
on 2026-07-20. The existing Swallowtail attached proof at build `b9910` was
reviewed separately; it remains an external-service attachment and is not
silently upgraded by this research.

No release artifact was installed. No model was loaded. No server process,
credential, download, or live inference route was used.

## Current Release

The current upstream release is `b10069`, commit
`178a6c44937154dc4c4eff0d166f4a044c4fceba`. Its macOS arm64 archive is
`llama-b10069-bin-macos-arm64.tar.gz`, published with SHA-256 digest
`022469e0b22f4b84dcd0a323867d7f5a31dae21894931ee6a24a35abd2a60359`.

Executable provenance and model provenance remain independent. A host may
approve this exact executable build without approving any model artifact, and
vice versa.

## Minimal Launch Shape

The first owned proof can use one exact host-approved `llama-server`
executable and one exact operator-supplied GGUF artifact:

- `--model` selects the already-present artifact
- `--alias` prevents the artifact path from becoming the public model id
- `--host 127.0.0.1` prevents an ambient external bind
- `--port 0` asks the server to bind an available port
- `--offline` prevents model-network access
- `--no-ui` disables the Web UI
- `--no-agent` keeps the agent proxy and built-in filesystem or shell tools off

Tagged source shows that port zero calls `bind_to_any_port`, stores the selected
port, and emits the resulting listening address. This is materially safer than
reserving a port, releasing it, and racing the child for the bind. The driver
may parse one bounded startup record into a host-scoped endpoint reference; raw
stderr never becomes a public diagnostic.

The first proof excludes model URLs, Hugging Face repositories, router mode,
dynamic model load or unload, server sleep, mutable properties, MCP proxying,
built-in tools, UI, media paths, public binds, TLS termination, and API-key
management.

## Readiness And Route Binding

`GET /health` returns a loading response until the model is ready and then a
successful `{"status":"ok"}` response. `GET /v1/models` exposes the loaded
route; an explicit alias keeps the model file path out of its public id.
`GET /props` provides the build and loaded-server observations already used by
the attached proof.

Owned start is therefore complete only after all of these hold within one
host-monotonic deadline:

1. the child reports one loopback listening address
2. health reports ready
3. observed build matches `b10069`
4. the exact configured alias is present
5. the observed deployment remains single-model and excluded features remain
   absent

The returned owned handle must retain the child, endpoint, and artifact leases.
It exposes a safe endpoint binding for subsequent catalogue or inference work;
it does not expose a host path, child pid, or raw startup output.

## Artifact Boundary

An operator-supplied model is not an attachment. It has durable identity,
format, revision, quantization, digest, provenance, and license posture outside
one inference request. The execution host resolves an opaque model-artifact
reference into a read-only lease for the serving scope.

Swallowtail may pass the host-resolved artifact value to the approved serving
executable. It does not download, convert, accept a license for, relocate,
delete, evict, or select the artifact. Artifact lease release occurs only after
the child is stopped and joined. Release never deletes consumer-owned model
material.

## Failure And Cleanup

Startup failure, malformed or duplicate listening records, non-loopback bind,
deadline expiry, early exit, health failure, build mismatch, route mismatch,
or unavailable artifact all fail the start operation. After process creation,
every failure requests graceful stop, escalates only under child ownership,
waits for exit, then releases endpoint and artifact authority.

Normal stop follows the same joined ordering. Dropping a public handle is not a
detached lifecycle and cannot leave an unobserved child task.

## Monkey Boundary

Monkey owns its model loading, warm-server, PID, readiness, and persistent
serving behavior. A later Monkey integration attaches to Monkey's service.
This proof launches only a narrowly configured upstream `llama-server` child
whose lifetime is exactly the Swallowtail owned-serving scope.

## Recommendation

Promote an owned-serving contract with three additions:

1. model-artifact references and read-only host leases distinct from
   attachments
2. an owned start result carrying a safe dynamically observed endpoint binding
3. joined startup failure and stop ordering across process, endpoint, and
   artifact authority

Then extend the provider-neutral owned conformance profile before implementing
the b10069 driver. Keep the existing b9910 attached driver unchanged.

## Risks

- upstream startup log shape can drift even when the HTTP facade remains
  compatible
- a model alias mismatch can expose an artifact path through catalogue output
- `--offline` constrains llama.cpp model retrieval but is not a general process
  network sandbox
- process limits and model memory requirements remain host-specific
- a host may approve an executable and artifact that are individually valid
  but incompatible
- a future router or persistent-server proof would need a separate lifecycle
  contract

## Primary Sources

- [llama.cpp release `b10069`](https://github.com/ggml-org/llama.cpp/releases/tag/b10069)
- [tagged `llama-server` documentation](https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/README.md)
- [tagged HTTP bind implementation](https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/server-http.cpp)
- [tagged server lifecycle implementation](https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/server.cpp)

## Promotion

- durable artifact and owned-start rules: Contract 018
- implementation sequence: roadmap 019
- operator decision: Kimi roadmap 018 remains paused while owned llama.cpp
  advances; deployment-owned containment is authorized for a later,
  platform-specific lane
