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
