# 2026-09-02 Claude Agent Dual-Route Compilation

The operator broke the `v0.4.0` feature freeze after a consumer raised a
Claude interactive-parity blocker. Current official Anthropic guidance says,
for now, third-party Agent SDK applications may draw from the user's Claude
subscription limits. Swallowtail will pursue a native `claude-agent.sdk` route
and independently widen `claude-agent.acp`.

Research 277 records the route, authentication, and release decisions.
g05.022 compiles two parallel evidence gates: card 053 owns native SDK policy,
artifact, credential, sidecar, and lifecycle evidence; card 054 owns the ACP
bridge parity census and delivery selection. Shared contract promotion and
implementation sequencing remain an orchestrator integration step after both
reviews.

The g05.021 release lane is paused. Card 050's partial semantic API generation
is retained but is not accepted release evidence. A fresh exact-head audit is
required after the Claude work reaches its release boundary.

