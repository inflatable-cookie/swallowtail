use futures_executor::block_on;
use std::fs;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicU64, Ordering};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    ResourceAccess, ResourceRepresentation, ScopeId, WorkingResourceIoService,
    WorkingResourceLocator, WorkingResourceReadRequest, WorkingResourceRef, WorkingResourceService,
    WorkingResourceText, WorkingResourceWriteRequest,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(1);

#[test]
fn reads_stay_bounded_by_the_approved_canonical_root() {
    let fixture = fixture_root();
    let root = fixture.join("workspace");
    let outside = fixture.join("outside.txt");
    fs::create_dir_all(root.join("src")).expect("fixture directory is created");
    fs::write(root.join("src/lib.rs"), "one\ntwo\nthree\n").expect("fixture source is written");
    fs::write(&outside, "private outside text").expect("outside fixture is written");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, root.join("escaped-link"))
        .expect("fixture symlink is created");

    let reference = WorkingResourceRef::new("workspace.fixture").expect("valid reference");
    let scope = ScopeId::new("working-resource-io-fixture").expect("valid scope");
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_working_resource(reference.clone(), &root)
        .build();
    let lease = block_on(host.resolve(
        scope,
        reference,
        ResourceAccess::Read,
        ResourceRepresentation::Filesystem,
    ))
    .expect("approved lease resolves");

    let read = WorkingResourceReadRequest::new(
        WorkingResourceLocator::new("src/lib.rs").expect("valid locator"),
        NonZeroUsize::new(64).expect("non-zero"),
    )
    .with_lines(Some(2), Some(1));
    let content = block_on(host.read_text(&lease, read)).expect("bounded read succeeds");
    assert_eq!(content.as_driver_value(), "two\n");

    let traversal = WorkingResourceReadRequest::new(
        WorkingResourceLocator::new("../outside.txt").expect("valid locator"),
        NonZeroUsize::new(64).expect("non-zero"),
    );
    assert_eq!(
        block_on(host.read_text(&lease, traversal))
            .expect_err("traversal fails")
            .diagnostic()
            .code(),
        "swallowtail.local_resource_io.boundary_rejected"
    );

    #[cfg(unix)]
    {
        let symlink = WorkingResourceReadRequest::new(
            WorkingResourceLocator::new("escaped-link").expect("valid locator"),
            NonZeroUsize::new(64).expect("non-zero"),
        );
        assert_eq!(
            block_on(host.read_text(&lease, symlink))
                .expect_err("symlink escape fails")
                .diagnostic()
                .code(),
            "swallowtail.local_resource_io.boundary_rejected"
        );
    }

    fs::remove_dir_all(&fixture).expect("fixture cleanup succeeds");
}

#[test]
fn writes_replace_or_create_regular_files_inside_the_approved_root() {
    let fixture = fixture_root();
    let root = fixture.join("workspace");
    fs::create_dir_all(root.join("src")).expect("fixture directory is created");
    fs::write(root.join("src/existing.rs"), "old").expect("fixture source is written");
    let outside = fixture.join("outside.txt");
    fs::write(&outside, "outside").expect("outside fixture is written");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, root.join("src/escaped-link"))
        .expect("fixture symlink is created");

    let reference = WorkingResourceRef::new("workspace.write-fixture").expect("valid reference");
    let scope = ScopeId::new("working-resource-write-fixture").expect("valid scope");
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_working_resource(reference.clone(), &root)
        .build();
    let lease = block_on(host.resolve(
        scope,
        reference,
        ResourceAccess::ReadWrite,
        ResourceRepresentation::Filesystem,
    ))
    .expect("approved lease resolves");
    let maximum = NonZeroUsize::new(64).expect("non-zero");

    for (locator, content) in [
        ("src/existing.rs", "replacement"),
        ("src/new.rs", "new file"),
    ] {
        let request = WorkingResourceWriteRequest::new(
            WorkingResourceLocator::new(locator).expect("valid locator"),
            WorkingResourceText::new(content.to_owned(), maximum).expect("bounded content"),
        );
        block_on(host.write_text(&lease, request)).expect("bounded write succeeds");
        assert_eq!(
            fs::read_to_string(root.join(locator)).expect("file is readable"),
            content
        );
    }

    let traversal = WorkingResourceWriteRequest::new(
        WorkingResourceLocator::new("../outside.txt").expect("valid locator"),
        WorkingResourceText::new("changed".to_owned(), maximum).expect("bounded content"),
    );
    assert_eq!(
        block_on(host.write_text(&lease, traversal))
            .expect_err("traversal fails")
            .diagnostic()
            .code(),
        "swallowtail.local_resource_io.boundary_rejected"
    );
    assert_eq!(
        fs::read_to_string(&outside).expect("outside is readable"),
        "outside"
    );

    #[cfg(unix)]
    {
        let symlink = WorkingResourceWriteRequest::new(
            WorkingResourceLocator::new("src/escaped-link").expect("valid locator"),
            WorkingResourceText::new("changed".to_owned(), maximum).expect("bounded content"),
        );
        assert_eq!(
            block_on(host.write_text(&lease, symlink))
                .expect_err("symlink fails")
                .diagnostic()
                .code(),
            "swallowtail.local_resource_io.boundary_rejected"
        );
    }

    fs::remove_dir_all(&fixture).expect("fixture cleanup succeeds");
}

fn fixture_root() -> std::path::PathBuf {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "swallowtail-working-resource-io-{}-{sequence}",
        std::process::id()
    ))
}
