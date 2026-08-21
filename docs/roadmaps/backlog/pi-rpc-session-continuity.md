# Pi RPC Session Continuity

Status: promoted to active decision

The operator reopened Pi continuity on 2026-08-21. Current `0.84.2` still lacks
a resource-bound RPC attachment: the public runtime accepts `cwdOverride`, but
RPC `switch_session` cannot carry it and `get_state` does not report effective
cwd. The post-g04.024 checkpoint owns the interface decision. Contracts 017 and
038 remain unchanged.
