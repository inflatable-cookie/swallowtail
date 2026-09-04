#[test]
fn parallel_hosts_do_not_reuse_a_released_fixture_path() {
    let host = swallowtail_core::ExecutionHostId::new("fixture.host.local").expect("host");
    let first = local_watcher_host(host.clone());
    let second = local_watcher_host(host);
    let first_resource = block_on(
        first
            .services()
            .working_resource()
            .expect("working resource")
            .create_temporary(
                swallowtail_runtime::ScopeId::new("fixture.scope.first").expect("scope"),
                swallowtail_core::ResourceAccess::ReadWrite,
                swallowtail_core::ResourceRepresentation::Filesystem,
            ),
    )
    .expect("first resource");
    let first_path = first_resource
        .filesystem()
        .expect("filesystem path")
        .as_driver_value()
        .to_owned();
    assert!(std::path::Path::new(&first_path).exists());
    assert_eq!(
        block_on(
            first
                .services()
                .working_resource()
                .expect("working resource")
                .release(first_resource)
        ),
        CleanupOutcome::Clean
    );
    assert!(!std::path::Path::new(&first_path).exists());

    let second_resource = block_on(
        second
            .services()
            .working_resource()
            .expect("working resource")
            .create_temporary(
                swallowtail_runtime::ScopeId::new("fixture.scope.second").expect("scope"),
                swallowtail_core::ResourceAccess::ReadWrite,
                swallowtail_core::ResourceRepresentation::Filesystem,
            ),
    )
    .expect("second resource");
    let second_path = second_resource
        .filesystem()
        .expect("filesystem path")
        .as_driver_value()
        .to_owned();
    assert_ne!(first_path, second_path);
    assert!(std::path::Path::new(&second_path).exists());
    assert_eq!(
        block_on(
            second
                .services()
                .working_resource()
                .expect("working resource")
                .release(second_resource)
        ),
        CleanupOutcome::Clean
    );
    assert!(!std::path::Path::new(&second_path).exists());
}
