# 2026-08-21 Post-Kimi Priority Reset

The operator reset the post-g04.024 priorities:

- hosted interactive OAuth, OpenHands, Aider, and Kiro headless return to one
  compact triage note and leave active roadmap summaries
- Gemini CLI requalification is reopened for enterprise API-key access;
  individual-account service is not a supported access posture and browser
  login remains outside the selected route
- Pi RPC session continuity becomes the immediate planning decision
- per-route feature completion becomes a delivery programme; new route-family
  research is not a priority

Current Pi `0.84.2` still does not expose the required caller-bound cwd through
RPC. Its public runtime has a `cwdOverride`, but RPC `switch_session` does not
carry it and `get_state` does not report effective cwd. Contract 017 remains
unchanged. The next Pi decision is whether to pursue the small upstream RPC
addition or authorize a separate Swallowtail-owned sidecar boundary.

Gemini host `0.53.0` and official stable `0.56.0` require the normal Contract
029 identity-before-claim workflow across the separate ACP and headless axes.
No authenticated prompt is required for the identity and deterministic claim
work. If the current mapped protocol cannot be qualified without live provider
work, the family stops for an explicit keep-or-remove decision.

g04.024 remains the active worker runway until its PR lands. No overlapping
worker was dispatched from this planning change.
