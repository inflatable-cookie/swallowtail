# 2026-09-24 Chatterbox Rulings: HTTP MCP Placement And No Terminal Stop

Owner: Tom
Date: 2026-09-24

## Decisions

**Consumer-supplied streamable-HTTP MCP placement admitted.** Tom accepted the
Chatterbox recommendation. Contract 063 now admits a URL-plus-header entry as a
placement shape: the consumer's server entry written verbatim into a harness's
own MCP configuration, with URL and header values redacted everywhere. It adds
no Swallowtail listener. Routes emit it only on frozen provider evidence;
honouring stays gated on a live gate. First route: `opencode.acp`, wired by
g06.019. Longhorn's stdio carrier stays the path for stdio-only harnesses.

**No terminal stop.** Tom ruled that stopping at an older version is not
acceptable. Contract 029 now treats a stop as a transient work item: the
record moves no claim, and Chatterbox compiles an adaptation task that
qualifies the current official stable. Narrowing a consumer-visible guarantee
is an operator ruling, never a reason to hold the ceiling. First owner:
g06.018 for the Claude Code `2.1.280` `--safe-mode` stop.

**Grok Bot suite pilot.** #358 (Grok Build ACP) enters review entry as the
pilot for #354–#359 (g06.020); the rest wait for a fresh base.

**Next currentness lane.** Command Code `1.54.0` → current official (g06.021).

**No rollover.** g06 holds 21 tasks, below the 30–50 range; the earlier
handoff's "well past the range" was wrong. g06 extends.

## Evidence

Research 336, 337, 338; npm `latest` at planning: Claude Code `2.1.281`,
Grok `1.0.41`, Command Code `1.65.0`.
