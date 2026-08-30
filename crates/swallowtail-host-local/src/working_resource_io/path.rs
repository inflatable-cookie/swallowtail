fn reject_parent_components(path: &Path) -> Result<(), RuntimeFailure> {
    if path
        .components()
        .any(|component| component == std::path::Component::ParentDir)
    {
        Err(failure(
            "swallowtail.local_resource_io.boundary_rejected",
            "Working-resource write locator contains traversal",
        ))
    } else {
        Ok(())
    }
}

fn write_parent_within_root(
    root: &Path,
    locator: &Path,
    private: bool,
) -> Result<PathBuf, RuntimeFailure> {
    if locator.is_absolute() {
        let candidate = locator.to_path_buf();
        let parent = candidate.parent().ok_or_else(write_parent_rejected)?;
        let parent = parent
            .canonicalize()
            .map_err(|_| write_parent_unavailable())?;
        if !parent.starts_with(root) || !parent.is_dir() {
            return Err(write_boundary_rejected());
        }
        return Ok(parent);
    }
    if locator.file_name().is_none() {
        return Err(write_parent_rejected());
    }
    let mut parent = root.to_path_buf();
    if let Some(relative_parent) = locator.parent() {
        for component in relative_parent.components() {
            match component {
                std::path::Component::CurDir => {}
                std::path::Component::Normal(name) => {
                    parent.push(name);
                    if !parent.exists() {
                        fs::create_dir(&parent).map_err(|_| write_parent_unavailable())?;
                        if private {
                            crate::materialization::restrict_directory(&parent)?;
                        }
                    }
                    parent = parent
                        .canonicalize()
                        .map_err(|_| write_parent_unavailable())?;
                    if !parent.starts_with(root) || !parent.is_dir() {
                        return Err(write_boundary_rejected());
                    }
                }
                _ => return Err(write_boundary_rejected()),
            }
        }
    }
    Ok(parent)
}

fn write_parent_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.local_resource_io.boundary_rejected",
        "Working-resource write has no approved parent directory",
    )
}

fn write_parent_unavailable() -> RuntimeFailure {
    failure(
        "swallowtail.local_resource_io.file_unavailable",
        "Working-resource write parent is unavailable",
    )
}

fn write_boundary_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.local_resource_io.boundary_rejected",
        "Working-resource write escaped the approved filesystem boundary",
    )
}

fn canonical_directory(path: &Path) -> Result<PathBuf, RuntimeFailure> {
    let root = path.canonicalize().map_err(|_| {
        failure(
            "swallowtail.local_resource_io.resource_unavailable",
            "Approved working-resource root is unavailable",
        )
    })?;
    if root.is_dir() {
        Ok(root)
    } else {
        Err(failure(
            "swallowtail.local_resource_io.resource_unavailable",
            "Approved working-resource root is not a directory",
        ))
    }
}

fn select_lines(content: &str, line: Option<usize>, limit: Option<usize>) -> String {
    if line.is_none() && limit.is_none() {
        return content.to_owned();
    }
    content
        .split_inclusive('\n')
        .skip(line.unwrap_or(1).saturating_sub(1))
        .take(limit.unwrap_or(usize::MAX))
        .collect()
}
