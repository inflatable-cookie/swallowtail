# Kimi Code local-server 0.31.0 status corpus

This secret-free delta corpus qualifies the selected local-server change in
Kimi Code `0.31.0`.

Exact source:

- signed release commit:
  `bc28e9d802fbec29395a7aed85e880679a050145`
- tree: `44634aa54e11f6d67e7807edf77bdfe19b3b99aa`
- repository: `https://github.com/MoonshotAI/kimi-code`

Selected source identity:

| Surface | `0.30.0` blob | `0.31.0` blob | Disposition |
| --- | --- | --- | --- |
| WebSocket event schema | `79de1337cab4346c399aa3dc098e0f8849a21678` | same | event type and fields unchanged |
| WebSocket control | `4fef9de57a8467d7c492d1546ca3c7efeb58515b` | same | protocol v2 unchanged |
| session event broadcaster | `2d968c8e92473f92404b6a7c7e05b8360c0ddd71` | `c1d6ebe8c7c00feeed031a322cf8258aad83ab17` | full status snapshot now folded for every agent |
| legacy status projection | `c70057c145d0214348571816d60bfc8d97e361aa` | `8d1771db07347c3a8b9216f1911d02fdcc81e464` | derived secondary model resolves to a display alias |

The upstream broadcaster test freezes a subagent status payload carrying
`agentId`, usage, context tokens, maximum context tokens, and model. The
synthetic `subagent-status.jsonl` retains that shape without provider,
account, or model observations.

Swallowtail maps `agent.status.updated` to non-rendered progress. Dedicated
subagent lifecycle events remain the portable activity authority. The richer
payload therefore adds no public model, usage, context, or raw status content.

One bounded installed `0.31.0` foreground smoke separately proved health,
bearer rejection, authenticated metadata, authenticated catalogue, and joined
process cleanup. No live payload or credential is retained here.

