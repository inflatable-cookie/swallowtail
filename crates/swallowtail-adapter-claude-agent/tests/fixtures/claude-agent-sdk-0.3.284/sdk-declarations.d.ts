/**
 * Reproducible excerpts copied from the frozen
 * @anthropic-ai/claude-agent-sdk 0.3.284 sdk.d.ts.
 */

// query — 0.3.284 package/sdk.d.ts:3232
export declare function query(_params: {
    prompt: string | AsyncIterable<SDKUserMessage>;
    options?: Options;
}): Query;
// AccountInfo — 0.3.284 package/sdk.d.ts:23
export declare type AccountInfo = {
    email?: string;
    organization?: string;
    subscriptionType?: string;
    tokenSource?: string;
    apiKeySource?: string;
    /**
     * Active API backend. Anthropic OAuth login only applies when "firstParty"; for 3P providers the other fields are absent and auth is external (AWS creds, gcloud ADC, etc.). "gateway" means the CLI is authenticated against an enterprise gateway.
     */
    apiProvider?: 'firstParty' | 'bedrock' | 'vertex' | 'foundry' | 'anthropicAws' | 'anthropicGoogleCloud' | 'mantle' | 'gateway';
};
// SpawnedProcess — 0.3.284 package/sdk.d.ts:9328
export declare interface SpawnedProcess {
    /** Writable stream for sending data to the process stdin */
    stdin: Writable;
    /** Readable stream for receiving data from the process stdout */
    stdout: Readable;
    /** Whether the process has been killed */
    readonly killed: boolean;
    /** Exit code if the process has exited, null otherwise */
    readonly exitCode: number | null;
    /**
     * Signal that terminated the process, if any. Optional: ChildProcess
     * provides it; custom spawners may omit it (signal exits then read as
     * still-running until their 'exit' event delivers the signal).
     */
    readonly signalCode?: NodeJS.Signals | null;
    /**
     * Kill the process with the given signal
     * @param signal - The signal to send (e.g., 'SIGTERM', 'SIGKILL')
     */
    kill(signal: NodeJS.Signals): boolean;
    /**
     * Register a callback for when the process exits
     * @param event - Must be 'exit'
     * @param listener - Callback receiving exit code and signal
     *
     * ProcessTransport's built-in local spawn delivers this only after the
     * child's stderr has also closed (bounded by a short grace), so exit
     * consumers see a complete stderr tail in exit errors. Custom
     * `spawnClaudeCodeProcess` implementations emit plain process exit.
     */
    on(event: 'exit', listener: (code: number | null, signal: NodeJS.Signals | null) => void): void;
    /**
     * Register a callback for process errors
     * @param event - Must be 'error'
     * @param listener - Callback receiving the error
     */
    on(event: 'error', listener: (error: Error) => void): void;
    /**
     * Register a one-time callback for when the process exits
     */
    once(event: 'exit', listener: (code: number | null, signal: NodeJS.Signals | null) => void): void;
    once(event: 'error', listener: (error: Error) => void): void;
    /**
     * Remove an event listener
     */
    off(event: 'exit', listener: (code: number | null, signal: NodeJS.Signals | null) => void): void;
    off(event: 'error', listener: (error: Error) => void): void;
}
// SpawnOptions — 0.3.284 package/sdk.d.ts:9380
export declare interface SpawnOptions {
    /** Command to execute */
    command: string;
    /** Arguments to pass to the command */
    args: string[];
    /** Working directory */
    cwd?: string;
    /** Environment variables */
    env: {
        [envVar: string]: string | undefined;
    };
    /**
     * Abort signal for cancellation.
     *
     * This is a **forwarded** signal owned by `ProcessTransport`, not the
     * caller's `Options.abortController.signal` directly. It aborts only
     * after the SDK's graceful-close path has run: stdin EOF →
     * `GRACEFUL_EXIT_TIMEOUT_MS` (~2 s) grace window. Anything you hang on
     * it (Node `spawn({signal})` → `child.kill()`, VM/container teardown,
     * fetch cancellation) fires **after** the child has had a chance to
     * shut down cleanly via stdin close.
     *
     * Why: passing the caller's raw signal to Node `spawn()` registers
     * Node's own abort listener that calls `child.kill()` — on Windows
     * that's `TerminateProcess` (instant, uncatchable), and AbortSignal
     * listeners fire synchronously in registration order, so it would race
     * ahead of the SDK's stdin-EOF + grace path and the CLI's
     * `gracefulShutdown` would never run.
     *
     * If you need the caller's *immediate* signal (no grace), it's the
     * `AbortController` you passed to `Options.abortController` — capture
     * it in closure.
     */
    signal: AbortSignal;
}
// CanUseTool — 0.3.284 package/sdk.d.ts:213
export declare type CanUseTool = (toolName: string, input: Record<string, unknown>, options: {
    /** Signaled if the operation should be aborted. */
    signal: AbortSignal;
    /**
     * Suggestions for updating permissions so that the user will not be
     * prompted again for this tool during this session.
     *
     * Typically if presenting the user an option 'always allow' or similar,
     * then this full set of suggestions should be returned as the
     * `updatedPermissions` in the PermissionResult.
     */
    suggestions?: PermissionUpdate[];
    /**
     * The file path that triggered the permission request, if applicable.
     * For example, when a Bash command tries to access a path outside allowed directories.
     */
    blockedPath?: string;
    /**
     * For `mcp__*` tools: the MCP server serving the tool and where its
     * definition came from. `source: 'sdk'` means one of the in-process
     * servers this SDK host registered (its `name` is the key you registered;
     * only the host can register one); any other value (`plugin`, `user`,
     * `project`, `local`, `dynamic`, `managed`, …) is a server from
     * configuration, whose `name` is the key as authored there — untrusted
     * text, escape it before display. Key trust decisions on `source`, not on
     * the name or the tool-name prefix. Absent for non-MCP tools and on CLIs
     * that predate the field.
     */
    mcpServer?: {
        name: string;
        source: string;
    };
    /** Explains why this permission request was triggered. */
    decisionReason?: string;
    /**
     * Full permission prompt sentence rendered by the bridge (e.g.
     * "Claude wants to read foo.txt"). Use this as the primary prompt
     * text when present instead of reconstructing from toolName+input.
     */
    title?: string;
    /**
     * Short noun phrase for the tool action (e.g. "Read file"), suitable
     * for button labels or compact UI.
     */
    displayName?: string;
    /**
     * Human-readable subtitle from the bridge (e.g. "Claude will have
     * read and write access to files in ~/Downloads").
     */
    description?: string;
    /**
     * The ask must not be approvable by a single stray keystroke: open the
     * prompt on its decline option and offer no one-key approve shortcut.
     */
    defaultToNo?: boolean;
    /**
     * The ask must not offer a persistent "don't ask again" choice: the
     * rule it would write grants more than this ask's own action.
     */
    suppressAlwaysAllowRule?: boolean;
    /**
     * Unique identifier for this specific tool call within the assistant message.
     * Multiple tool calls in the same assistant message will have different toolUseIDs.
     */
    toolUseID: string;
    /** If running within the context of a sub-agent, the sub-agent's ID. */
    agentID?: string;
    /**
     * The control_request envelope's `request_id`. A control_response sent
     * out-of-band (e.g. a signed HTTP POST instead of the SDK's WS write)
     * must echo this value for the worker to match it.
     */
    requestId: string;
    /**
     * Set when a user-configured ask RULE (permissions.ask) forced this
     * prompt while the ask carries the tool's own decisionReason. Hosts
     * making policy on the reason (e.g. auto-deny a safetyCheck) or
     * running host-side auto-approval should treat asks carrying this
     * field as rule-forced: the user's stated intent is a human prompt.
     */
    matchedAskRule?: {
        source: string;
        toolName: string;
        ruleContent?: string;
    };



}) => Promise<PermissionResult | null>;
// PermissionResult — 0.3.284 package/sdk.d.ts:2494
export declare type PermissionResult = {
    behavior: 'allow';
    updatedInput?: Record<string, unknown>;
    updatedPermissions?: PermissionUpdate[];
    toolUseID?: string;
    decisionClassification?: PermissionDecisionClassification;
} | {
    behavior: 'deny';
    message: string;
    interrupt?: boolean;
    toolUseID?: string;
    decisionClassification?: PermissionDecisionClassification;
};
// McpStdioServerConfig — 0.3.284 package/sdk.d.ts:1327
export declare type McpStdioServerConfig = {
    type?: 'stdio';
    command: string;
    args?: string[];
    env?: Record<string, string>;
    /**
     * Per-server tool-call timeout in milliseconds. Overrides the MCP_TOOL_TIMEOUT environment variable for this server. Hard wall-clock limit per call; progress notifications do not extend it. Values below 1000ms are ignored (falls through to MCP_TOOL_TIMEOUT or the default).
     */
    timeout?: number;
    /**
     * When true, all tools from this server are always included in the prompt and never deferred behind tool search. Equivalent to setting defer_loading: false on the API. Default: tools are deferred when tool search is enabled. As a side effect this also blocks startup until the server is connected (capped at the standard 5s connect timeout) even though MCP startup is otherwise non-blocking by default, since the tools must be present when the turn-1 prompt is built.
     */
    alwaysLoad?: boolean;

};
// McpServerStatus — 0.3.284 package/sdk.d.ts:1226
export declare type McpServerStatus = {
    /**
     * Server name as configured
     */
    name: string;
    /**
     * Current connection status
     */
    status: 'connected' | 'failed' | 'needs-auth' | 'pending' | 'disabled';
    /**
     * Server information (available when connected)
     */
    serverInfo?: {
        name: string;
        version: string;
    };
    /**
     * Error message (available when status is 'failed')
     */
    error?: string;

    /**
     * Server configuration (includes URL for HTTP/SSE servers)
     */
    config?: McpServerStatusConfig;
    /**
     * Configuration scope (e.g., project, user, local, claudeai, managed)
     */
    scope?: string;
    /**
     * Where the server definition came from: sdk (an in-process server the SDK host registered — only the host can register one), plugin (a server a plugin ships or registers at runtime), or the config scope (user, project, local, dynamic, managed, enterprise, claudeai, agent). Key trust on this, not on the name. Absent on CLIs that predate the field.
     */
    source?: string;
    /**
     * Tools provided by this server (available when connected)
     */
    tools?: {
        name: string;
        description?: string;
        annotations?: {
            readOnly?: boolean;
            destructive?: boolean;
            openWorld?: boolean;
        };
        /**
         * The MCP Apps (SEP-1865) members of the tool's `_meta`, for a host that renders the tool's `ui://` resource, under the keys the server used: `ui` (an object: `resourceUri`, a `ui://` string; `visibility`, an array of 'model' | 'app'; and any other member the server sent) and the deprecated flat `ui/resourceUri` (a `ui://` string). Validated and size-bounded; every other `_meta` key is withheld. Present only on a tool that declares one, from CLIs that advertise `mcp_tool_ui_meta_v1`.
         */
        _meta?: Record<string, unknown>;
    }[];

};
// Options.mcpServers — 0.3.284 package/sdk.d.ts:1950
    mcpServers?: Record<string, McpServerConfig>;
// Options.strictMcpConfig — 0.3.284 package/sdk.d.ts:2288
    strictMcpConfig?: boolean;
// Query.mcpServerStatus — 0.3.284 package/sdk.d.ts:3011
    mcpServerStatus(): Promise<McpServerStatus[]>;
// Query.close — 0.3.284 package/sdk.d.ts:3226-3229
    /**
     * Use this when you need to abort a query that is still running.
     * After calling close(), no further messages will be received.
     */
    close(): void;

/**
 * Usage and context excerpts copied from official npm
 * @anthropic-ai/claude-agent-sdk 0.3.284 package/sdk.d.ts. The source
 * tarball SHA-256 is 4550e830246026133fc1802a2208dd0f3a785cae1eec83f261d114c33d797771;
 * package/sdk.d.ts SHA-256 is 048ae2e6c796cc2aa3c423afaad59a08972cb48c271ffcc9847d910ff65f61b2.
 */

// package/sdk.d.ts:1-3 — BetaUsage comes from the declared peer package.
import type { BetaUsage } from '@anthropic-ai/sdk/resources/beta/messages/messages.mjs';

// ModelUsage — package/sdk.d.ts:1425-1450
export declare type ModelUsage = {
    inputTokens: number;
    outputTokens: number;
    /**
     * Thinking tokens, already counted inside outputTokens. Counts only turns run on CLI versions that record this field: absent when none did, and partial for a resumed session that began on an older version.
     */
    thinkingTokens?: number;
    cacheReadInputTokens: number;
    cacheCreationInputTokens: number;
    webSearchRequests: number;
    costUSD: number;
    contextWindow: number;
    maxOutputTokens: number;
    /**
     * Canonical model id used for the pricing lookup (e.g. 'claude-opus-4-7'). May differ from the raw model string this entry is keyed by (provider-specific ids, aliases).
     */
    canonicalModel?: string;
    /**
     * API provider that served this model (e.g. 'firstParty', 'bedrock', 'vertex', 'foundry', 'anthropicAws', 'mantle', 'gateway').
     */
    provider?: string;
    /**
     * Which price table the most recent request for this model was priced at: Claude Code's built-in list prices ('list'), the organization's managed-settings modelPricing rates or multiplier ('managed'), or neither ('unknown' — no pricing row and no built-in price matched the model ID, so costUSD is a guess at the default model's rate). Overwritten per request like canonicalModel, so a consumer that differences the cumulative costUSD per turn gets that turn's basis. Absent until this process has priced a request for the model (e.g. right after --resume) and on builds that predate the field; treat as 'list'.
     */
    costBasis?: 'list' | 'managed' | 'unknown';
};

// NonNullableUsage — package/sdk.d.ts:1452-1454
export declare type NonNullableUsage = {
    [K in keyof BetaUsage]: NonNullable<BetaUsage[K]>;
};

// SDKResultError — package/sdk.d.ts:5620-5626
/**
 * MAIN AGENT LOOP ONLY — excludes Task subagent, sidechain, and auxiliary model calls, and is per-turn in streaming-input sessions. Prefer modelUsage for token/cost accounting.
 */
usage: NonNullableUsage;
/**
 * Per-model totals for every model call made through the query pipeline during this query() call — main loop, Task subagents, sidechains, and internal calls such as compaction and Workflow agents. Cumulative across turns in streaming-input sessions: each result carries the running total so far, so read the latest result rather than summing across results. Internal helper calls outside the query pipeline (e.g. the permission classifier, token-count probes) are excluded; crash/startup-error results may carry zeroed usage, a resumed or forked session continues from the totals its transcript saved, when it has them (so the first result already carries the earlier turns), and a mid-session /clear resets the running total. The correct field for token/cost accounting; treat it as an estimate, not a billing statement.
 */
modelUsage: Record<string, ModelUsage>;

// SDKResultSuccess — package/sdk.d.ts:5707-5713; same field contracts as SDKResultError.
/**
 * MAIN AGENT LOOP ONLY — excludes Task subagent, sidechain, and auxiliary model calls, and is per-turn in streaming-input sessions. Prefer modelUsage for token/cost accounting.
 */
usage: NonNullableUsage;
/**
 * Per-model totals for every model call made through the query pipeline during this query() call — main loop, Task subagents, sidechains, and internal calls such as compaction and Workflow agents. Cumulative across turns in streaming-input sessions: each result carries the running total so far, so read the latest result rather than summing across results. Internal helper calls outside the query pipeline (e.g. the permission classifier, token-count probes) are excluded; crash/startup-error results may carry zeroed usage, a resumed or forked session continues from the totals its transcript saved, when it has them (so the first result already carries the earlier turns), and a mid-session /clear resets the running total. The correct field for token/cost accounting; treat it as an estimate, not a billing statement.
 */
modelUsage: Record<string, ModelUsage>;

// Query.supportedModels and Query.getContextUsage — package/sdk.d.ts:2995-2999, 3012-3024
supportedModels(): Promise<ModelInfo[]>;
/**
 * Get a breakdown of current context window usage by category
 * (system prompt, tools, messages, MCP tools, memory files, etc.).
 *
 * `detail: 'full'` counts each category with the token-count API;
 * `'summary'` answers from the last response's usage and local estimates
 * without the per-category token-count calls. Defaults to `'full'`.
 *
 * @returns Context usage breakdown including token counts per category and total usage
 */
getContextUsage(opts?: {
    detail?: 'summary' | 'full';
}): Promise<SDKControlGetContextUsageResponse>;

// SDKControlGetContextUsageResponse — package/sdk.d.ts:3931-3947, 3956 (selected fields)
export declare type SDKControlGetContextUsageResponse = {
    categories: {
        name: string;
        tokens: number;
        color: string;
        isDeferred?: boolean;
        /**
         * What the row is, the same classification the /context result's context_usage rows carry: 'used' content occupies the window; 'free' is the remaining window; 'buffer' is the compaction reserve; 'deferred' rows are out-of-window tool schemas. Classify on this, never on the English name.
         */
        kind: 'used' | 'free' | 'buffer' | 'deferred';
    }[];
    totalTokens: number;
    maxTokens: number;
    rawMaxTokens: number;
    percentage: number;
    // The source declaration also includes grid, tool, memory, agent and other breakdown fields.
    model: string;
};

// ModelInfo — package/sdk.d.ts:1384-1423; no context-window field is declared.
export declare type ModelInfo = {
    /**
     * Model identifier to use in API calls
     */
    value: string;
    /**
     * Canonical wire model id this row's `value` resolves to (e.g. 'sonnet' → 'claude-sonnet-5'). Lets hosts match a persisted explicit id against the alias row that covers it.
     */
    resolvedModel?: string;
    /**
     * Human-readable display name
     */
    displayName: string;
    /**
     * Description of the model's capabilities
     */
    description: string;
    /**
     * Whether this model supports effort levels
     */
    supportsEffort?: boolean;
    /**
     * Available effort levels for this model
     */
    supportedEffortLevels?: ('low' | 'medium' | 'high' | 'xhigh' | 'max')[];
    /**
     * Whether this model supports adaptive thinking (Claude decides when and how much to think)
     */
    supportsAdaptiveThinking?: boolean;
    /**
     * Whether this model supports fast mode
     */
    supportsFastMode?: boolean;
    /**
     * Whether this model supports auto mode
     */
    supportsAutoMode?: boolean;
};

// SDKAssistantMessage — package/sdk.d.ts:3654-3656
/**
 * Structured twin of the /usage report, carried on the synthetic assistant message that delivers its text: the session totals, the plan's usage rows and extra-usage spend, for remote clients that render a card from data. Present only on /usage results from CLIs new enough to attach it and from claude.ai-subscriber sessions; the text in message.content remains the canonical fallback. Wrapper-level sibling — never inside `message.content` — so it is not replayed to the model.
 */
usage_report?: SDKUsageReport;

// SDKUsageReport — package/sdk.d.ts:6071-6085 (session excerpt)
/**
 * Structured twin of a /usage result, carried beside its text: the session's totals, the plan's usage rows as the server sent them and the extra-usage spend, and nothing else from the usage body (the get_usage control reply carries the rest). Experimental — the shape may change.
 */
export declare type SDKUsageReport = {
    /**
     * Cost and usage accumulated by the current session.
     */
    session: {
        total_cost_usd: number;
        total_api_duration_ms: number;
        total_duration_ms: number;
        total_lines_added: number;
        total_lines_removed: number;
        model_usage: Record<string, ModelUsage>;
    };
    // Plan rate-limit rows and extra-usage spend are omitted from this excerpt.
};
