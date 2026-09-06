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
