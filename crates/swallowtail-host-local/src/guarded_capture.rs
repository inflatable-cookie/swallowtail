//! Local capture for one adapter-frozen guarded graph.
//!
//! An adapter freezes the manifest, loader and identity digests for one hop;
//! this module turns those bytes into a captured recipe under the local host's
//! custody. It reads files, so it cannot live in the adapter, and it must not
//! know which provider froze them, so the frozen inputs arrive as the
//! provider-neutral runtime descriptor.

use crate::captured_source::{LocalCapturedGraph, LocalCapturedNodeRecipe};
use crate::trusted_runtime::LocalTrustedRuntime;
use std::io;
use std::path::Path;
use swallowtail_runtime::FrozenGuardedNodeCapture;

/// Captures one frozen guarded graph and derives its candidate Node recipe.
///
/// `root` holds the exact manifest-declared package trees. The caller keeps
/// custody of the Node runtime, the native loader dependency and, when the
/// frozen manifest requires it, the process-helper runtime. Unknown or
/// substituted bytes refuse; nothing here authorizes a launch, and the caller
/// must still bind the recipe into its checked-process service.
pub fn capture_frozen_guarded_recipe(
    root: &Path,
    frozen: FrozenGuardedNodeCapture<'_>,
    node: &LocalTrustedRuntime,
    native: &LocalTrustedRuntime,
    ps: Option<&LocalTrustedRuntime>,
) -> io::Result<LocalCapturedNodeRecipe> {
    node.verify()?;
    let graph = LocalCapturedGraph::capture_manifest(
        root,
        frozen.manifest(),
        frozen.platform(),
        native,
        ps,
    )?;
    LocalCapturedNodeRecipe::for_trusted_runtime(
        &graph,
        frozen.loader(),
        frozen.vendor_sha256(),
        node,
        frozen.policy_sha256(),
    )
}
