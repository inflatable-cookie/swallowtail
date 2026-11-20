//! Byte bounds on provider output the adapter buffers.
//!
//! A child process controls the size of every line and delta it writes, so
//! the host caps each buffer that grows from that output. A breach ends the
//! stream with a bounded failure rather than growing without limit.

/// Largest stdout line the adapter buffers while it waits for a newline.
pub(crate) const MAX_STDOUT_LINE_BYTES: usize = 16 * 1024 * 1024;

/// Largest assistant output one turn may accumulate from streamed deltas.
pub(crate) const MAX_TURN_OUTPUT_BYTES: usize = 16 * 1024 * 1024;
