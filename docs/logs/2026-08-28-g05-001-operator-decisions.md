# 2026-08-28 g05.001 Operator Decisions

## Decisions

1. Skill discovery reports the effective skill set visible to the selected
   harness session. It includes distribution-bundled, operator-installed
   global, and project-local skills when the harness admits them. Exact harness
   evidence is required; ambient filesystem scans and file-presence inference
   remain out.
2. The model and operator both receive watcher controls through separate typed
   operations against one host-owned registry.
3. Consumers receive lifecycle, status, and bounded redacted output summaries.
   Raw or continuous logs remain out.
4. Explicit watcher wait pauses the agent turn. Successful completion fails
   closed while a watcher remains active. Cancellation and deadline stop and
   join every owned watcher before the turn fails.

## Effect

The decisions close the card 002 gate. Card 003 is ready to promote the two
independent boundaries into architecture and contracts and select proof-route
dispositions. No implementation or public API is selected by this record.
