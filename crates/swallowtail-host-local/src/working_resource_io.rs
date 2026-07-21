use crate::host::LocalProcessHost;
use crate::output::failure;
use std::fs;
use std::path::{Path, PathBuf};
use swallowtail_runtime::{
    BoxFuture, ResourceAccess, ResourceLease, ResourceRepresentation, RuntimeFailure,
    WorkingResourceIoService, WorkingResourceReadRequest, WorkingResourceText,
    WorkingResourceWriteRequest,
};

const MAXIMUM_READ_BYTES: usize = 1024 * 1024;
const MAXIMUM_WRITE_BYTES: usize = 1024 * 1024;

impl WorkingResourceIoService for LocalProcessHost {
    fn read_text(
        &self,
        lease: &ResourceLease,
        request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>> {
        let result = self.read_working_resource_text(lease, &request);
        Box::pin(async move { result })
    }

    fn write_text(
        &self,
        lease: &ResourceLease,
        request: WorkingResourceWriteRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let result = self.write_working_resource_text(lease, &request);
        Box::pin(async move { result })
    }
}

impl LocalProcessHost {
    fn read_working_resource_text(
        &self,
        lease: &ResourceLease,
        request: &WorkingResourceReadRequest,
    ) -> Result<WorkingResourceText, RuntimeFailure> {
        if !matches!(
            lease.access(),
            ResourceAccess::Read | ResourceAccess::ReadWrite
        ) || lease.representation() != ResourceRepresentation::Filesystem
        {
            return Err(failure(
                "swallowtail.local_resource_io.lease_rejected",
                "Working-resource read requires a filesystem read lease",
            ));
        }
        if request.maximum_bytes().get() > MAXIMUM_READ_BYTES {
            return Err(failure(
                "swallowtail.local_resource_io.byte_limit_rejected",
                "Working-resource read exceeds the local host byte limit",
            ));
        }
        if request.line() == Some(0) || request.limit() == Some(0) {
            return Err(failure(
                "swallowtail.local_resource_io.line_range_rejected",
                "Working-resource line ranges are one-based and non-zero",
            ));
        }
        let root = self
            .approvals
            .working_resources
            .get(lease.reference())
            .cloned()
            .or_else(|| {
                self.materialization
                    .working_resource_path(lease.scope(), lease.reference())
            })
            .ok_or_else(|| {
                failure(
                    "swallowtail.local_resource_io.resource_not_approved",
                    "Working-resource read lease is not approved for this scope",
                )
            })?;
        let root = canonical_directory(&root)?;
        let locator = Path::new(request.locator().as_host_value());
        let candidate = if locator.is_absolute() {
            locator.to_path_buf()
        } else {
            root.join(locator)
        };
        let target = candidate.canonicalize().map_err(|_| {
            failure(
                "swallowtail.local_resource_io.file_unavailable",
                "Working-resource file is unavailable",
            )
        })?;
        if !target.starts_with(&root) || !target.is_file() {
            return Err(failure(
                "swallowtail.local_resource_io.boundary_rejected",
                "Working-resource read escaped the approved filesystem boundary",
            ));
        }
        let metadata = fs::metadata(&target).map_err(|_| {
            failure(
                "swallowtail.local_resource_io.file_unavailable",
                "Working-resource file is unavailable",
            )
        })?;
        if metadata.len() > request.maximum_bytes().get() as u64 {
            return Err(failure(
                "swallowtail.local_resource_io.byte_limit_exceeded",
                "Working-resource file exceeds the approved read limit",
            ));
        }
        let content = fs::read_to_string(target).map_err(|_| {
            failure(
                "swallowtail.local_resource_io.text_unavailable",
                "Working-resource file is not available as UTF-8 text",
            )
        })?;
        let content = select_lines(&content, request.line(), request.limit());
        WorkingResourceText::new(content, request.maximum_bytes()).map_err(|_| {
            failure(
                "swallowtail.local_resource_io.byte_limit_exceeded",
                "Working-resource result exceeds the approved read limit",
            )
        })
    }

    fn write_working_resource_text(
        &self,
        lease: &ResourceLease,
        request: &WorkingResourceWriteRequest,
    ) -> Result<(), RuntimeFailure> {
        if lease.access() != ResourceAccess::ReadWrite
            || lease.representation() != ResourceRepresentation::Filesystem
        {
            return Err(failure(
                "swallowtail.local_resource_io.lease_rejected",
                "Working-resource write requires a filesystem read-write lease",
            ));
        }
        if request.content().byte_len() > MAXIMUM_WRITE_BYTES {
            return Err(failure(
                "swallowtail.local_resource_io.byte_limit_rejected",
                "Working-resource write exceeds the local host byte limit",
            ));
        }
        let root = self.approved_root(lease)?;
        let locator = Path::new(request.locator().as_host_value());
        reject_parent_components(locator)?;
        let candidate = if locator.is_absolute() {
            locator.to_path_buf()
        } else {
            root.join(locator)
        };
        let parent = candidate.parent().ok_or_else(|| {
            failure(
                "swallowtail.local_resource_io.boundary_rejected",
                "Working-resource write has no approved parent directory",
            )
        })?;
        let parent = parent.canonicalize().map_err(|_| {
            failure(
                "swallowtail.local_resource_io.file_unavailable",
                "Working-resource write parent is unavailable",
            )
        })?;
        if !parent.starts_with(&root) || !parent.is_dir() {
            return Err(failure(
                "swallowtail.local_resource_io.boundary_rejected",
                "Working-resource write escaped the approved filesystem boundary",
            ));
        }
        let file_name = candidate.file_name().ok_or_else(|| {
            failure(
                "swallowtail.local_resource_io.boundary_rejected",
                "Working-resource write target is invalid",
            )
        })?;
        let target = parent.join(file_name);
        match fs::symlink_metadata(&target) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
                return Err(failure(
                    "swallowtail.local_resource_io.boundary_rejected",
                    "Working-resource write target is not a regular file",
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => {
                return Err(failure(
                    "swallowtail.local_resource_io.file_unavailable",
                    "Working-resource write target is unavailable",
                ));
            }
        }
        fs::write(target, request.content().as_driver_value()).map_err(|_| {
            failure(
                "swallowtail.local_resource_io.write_failed",
                "Working-resource text replacement failed",
            )
        })
    }

    fn approved_root(&self, lease: &ResourceLease) -> Result<PathBuf, RuntimeFailure> {
        let root = self
            .approvals
            .working_resources
            .get(lease.reference())
            .cloned()
            .or_else(|| {
                self.materialization
                    .working_resource_path(lease.scope(), lease.reference())
            })
            .ok_or_else(|| {
                failure(
                    "swallowtail.local_resource_io.resource_not_approved",
                    "Working-resource write lease is not approved for this scope",
                )
            })?;
        canonical_directory(&root)
    }
}

include!("working_resource_io/path.rs");
