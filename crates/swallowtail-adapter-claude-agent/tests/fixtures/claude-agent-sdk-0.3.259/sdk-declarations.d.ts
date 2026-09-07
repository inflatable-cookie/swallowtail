/**
 * Reproducible excerpts copied from the frozen
 * @anthropic-ai/claude-agent-sdk 0.3.259 sdk.d.ts.
 */

import type { MessageParam } from '@anthropic-ai/sdk/resources';

export declare type SDKUserMessage = {
    type: 'user';
    message: MessageParam;
    parent_tool_use_id: string | null;
    session_id?: string;
};

export declare function query(_params: {
    prompt: string | AsyncIterable<SDKUserMessage>;
    options?: Options;
}): Query;

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

/**
 * Query control methods used by the sidecar during the initialize handshake.
 */
export declare interface Query extends AsyncGenerator<SDKMessage, void> {
    initializationResult(): Promise<SDKControlInitializeResponse>;
    supportedModels(): Promise<ModelInfo[]>;
    accountInfo(): Promise<AccountInfo>;
}

/**
 * Represents a spawned process with stdin/stdout streams and lifecycle management.
 * Implementers provide this interface to abstract the process spawning mechanism.
 * ChildProcess already satisfies this interface.
 */
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

/**
 * Options passed to the spawn function.
 */
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

/*
 * === Card 116 extension ==================================================
 *
 * Verbatim declarations copied from the same frozen
 * @anthropic-ai/claude-agent-sdk 0.3.259 `package/sdk.d.ts` (tarball sha256
 * 0c5740e44a536ab6fd32f2a7de0d508b75d34782ebc219b87aa8d834449a3f7e, npm
 * shasum daf465f8231392ab99e1c7fc7f1e14c3d25ea012).
 *
 * These are the exact shapes the Card 116 registered-tool gap capsule cites
 * for REG-03, CB-01, MCP-02, and MCP-03. Before this extension the capsule's
 * field lists were not reproducible from the repository; they are now.
 *
 * Reproduce with:
 *   npm pack @anthropic-ai/claude-agent-sdk@0.3.259
 *   shasum -a 256 anthropic-ai-claude-agent-sdk-0.3.259.tgz
 *   tar xzf anthropic-ai-claude-agent-sdk-0.3.259.tgz
 *   sed -n '<start>,<end>p' package/sdk.d.ts
 *
 * Exact 0.3.259 line ranges, in the order the excerpts appear below:
 *   CanUseTool                       205-269
 *   PermissionResult                 2324-2336
 *   McpStdioServerConfig             1208-1224
 *   McpSSEServerConfig               1192-1207
 *   McpHttpServerConfig              1073-1088
 *   McpSdkServerConfig               1090-1097
 *   McpSdkServerConfigWithInstance   1099-1106
 *   McpServerConfig                  1108-1112
 *   McpServerStatus                  1114-1158
 *   McpServerStatusConfig            1160-1160
 *   Options.mcpServers               1788-1802
 *   Options.strictMcpConfig          2107-2119
 *   Query.mcpServerStatus            2745-2750
 *   SdkMcpToolDefinition             4574-4586
 *   tool                             8508-8512
 *   createSdkMcpServer               505-511
 *
 * `Options.mcpServers`, `Options.strictMcpConfig`, and
 * `Query.mcpServerStatus` are members of the larger `Options` and `Query`
 * declarations. Only the member is reproduced, so each appears below as a
 * bare fragment rather than a complete declaration.
 */
// CanUseTool — 0.3.259 package/sdk.d.ts:205-269
 * `requestId`); the SDK will skip its own transport write. Fail-closed: an
 * accidental null means no control_response is sent and the tool stays
 * blocked indefinitely — permission prompts have no park deadline.
 */
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
// PermissionResult — 0.3.259 package/sdk.d.ts:2324-2336
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
// McpStdioServerConfig — 0.3.259 package/sdk.d.ts:1208-1224

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

// McpSSEServerConfig — 0.3.259 package/sdk.d.ts:1192-1207
export declare type McpSSEServerConfig = {
    type: 'sse';
    url: string;
    headers?: Record<string, string>;
    tools?: McpServerToolPolicy[];
    /**
     * Per-server tool-call timeout in milliseconds. Overrides the MCP_TOOL_TIMEOUT environment variable for this server. Hard wall-clock limit per call; progress notifications do not extend it. Values below 1000ms are ignored (falls through to MCP_TOOL_TIMEOUT or the default).
     */
    timeout?: number;

    /**
     * When true, all tools from this server are always included in the prompt and never deferred behind tool search. Equivalent to setting defer_loading: false on the API. Default: tools are deferred when tool search is enabled. As a side effect this also blocks startup until the server is connected (capped at the standard 5s connect timeout) even though MCP startup is otherwise non-blocking by default, since the tools must be present when the turn-1 prompt is built.
     */
    alwaysLoad?: boolean;

};
// McpHttpServerConfig — 0.3.259 package/sdk.d.ts:1073-1088
export declare type McpHttpServerConfig = {
    type: 'http';
    url: string;
    headers?: Record<string, string>;
    tools?: McpServerToolPolicy[];
    /**
     * Per-server tool-call timeout in milliseconds. Overrides the MCP_TOOL_TIMEOUT environment variable for this server. Hard wall-clock limit per call; progress notifications do not extend it. Values below 1000ms are ignored (falls through to MCP_TOOL_TIMEOUT or the default).
     */
    timeout?: number;

    /**
     * When true, all tools from this server are always included in the prompt and never deferred behind tool search. Equivalent to setting defer_loading: false on the API. Default: tools are deferred when tool search is enabled. As a side effect this also blocks startup until the server is connected (capped at the standard 5s connect timeout) even though MCP startup is otherwise non-blocking by default, since the tools must be present when the turn-1 prompt is built.
     */
    alwaysLoad?: boolean;

};
// McpSdkServerConfig — 0.3.259 package/sdk.d.ts:1090-1097
export declare type McpSdkServerConfig = {
    type: 'sdk';
    name: string;
    /**
     * Per-server tool-call timeout in milliseconds. Overrides the MCP_TOOL_TIMEOUT environment variable for this server. Hard wall-clock limit per call; progress notifications do not extend it. Values below 1000ms are ignored (falls through to MCP_TOOL_TIMEOUT or the default). Applies when the server is first registered; changing it for an already-registered server has no effect until it is removed and re-added.
     */
    timeout?: number;
};
// McpSdkServerConfigWithInstance — 0.3.259 package/sdk.d.ts:1099-1106
/**
 * MCP SDK server config with an actual McpServer instance.
 * Not serializable - contains a live McpServer object.
 */
export declare type McpSdkServerConfigWithInstance = McpSdkServerConfig & {
    instance: McpServer;
};

// McpServerConfig — 0.3.259 package/sdk.d.ts:1108-1112
 * Union of all MCP server config types, including those with non-serializable instances.
 */
export declare type McpServerConfig = McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfigWithInstance;

export declare type McpServerConfigForProcessTransport = McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig;
// McpServerStatus — 0.3.259 package/sdk.d.ts:1114-1158
/**
 * Status information for an MCP server connection.
 */
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
    }[];

};
// McpServerStatusConfig — 0.3.259 package/sdk.d.ts:1160-1160
export declare type McpServerStatusConfig = McpServerConfigForProcessTransport | McpClaudeAIProxyServerConfig;
// Options.mcpServers — 0.3.259 package/sdk.d.ts:1788-1802
    /**
     * MCP (Model Context Protocol) server configurations.
     * Keys are server names, values are server configurations.
     *
     * @example
     * ```typescript
     * mcpServers: {
     *   'my-server': {
     *     command: 'node',
     *     args: ['./my-mcp-server.js']
     *   }
     * }
     * ```
     */
    mcpServers?: Record<string, McpServerConfig>;
// Options.strictMcpConfig — 0.3.259 package/sdk.d.ts:2107-2119
    /**
     * Callback for stderr output from the Claude Code process.
     * Useful for debugging and logging.
     */
    stderr?: (data: string) => void;
    /**
     * Only use MCP servers passed via the `mcpServers` option (and servers
     * declared by explicitly-passed agent definitions in `agents`), ignoring
     * all other MCP configurations: project `.mcp.json`, user settings,
     * plugins, and on-disk agent frontmatter — including subagent frontmatter
     * MCP. Maps to the CLI `--strict-mcp-config` flag.
     */
    strictMcpConfig?: boolean;
// Query.mcpServerStatus — 0.3.259 package/sdk.d.ts:2745-2750
    /**
     * Get the current status of all configured MCP servers.
     *
     * @returns Array of MCP server statuses (connected, failed, needs-auth, pending)
     */
    mcpServerStatus(): Promise<McpServerStatus[]>;
// SdkMcpToolDefinition — 0.3.259 package/sdk.d.ts:4574-4586
/**
 * MCP tool definition for SDK servers.
 * Contains a handler function, so not serializable.
 * Supports both Zod 3 and Zod 4 schemas.
 */
export declare type SdkMcpToolDefinition<Schema extends AnyZodRawShape = AnyZodRawShape> = {
    name: string;
    description: string;
    inputSchema: Schema;
    annotations?: ToolAnnotations;
    _meta?: Record<string, unknown>;
    handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>;
};
// tool — 0.3.259 package/sdk.d.ts:8508-8512
export declare function tool<Schema extends AnyZodRawShape>(_name: string, _description: string, _inputSchema: Schema, _handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>, _extras?: {
    annotations?: ToolAnnotations;
    searchHint?: string;
    alwaysLoad?: boolean;
}): SdkMcpToolDefinition<Schema>;
// createSdkMcpServer — 0.3.259 package/sdk.d.ts:505-511
 * This allows SDK users to define custom tools that run in the same process.
 *
 * Tool calls are bounded by the MCP tool-call timeout — `options.timeout`
 * (ms) for this server, else the MCP_TOOL_TIMEOUT env var, effectively
 * unbounded by default.
 */
export declare function createSdkMcpServer(_options: CreateSdkMcpServerOptions): McpSdkServerConfigWithInstance;
