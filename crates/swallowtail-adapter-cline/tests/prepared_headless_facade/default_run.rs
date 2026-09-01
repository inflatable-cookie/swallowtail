#[test]
fn prepared_run_names_cline_headless_and_package_then_drains_one_print() {
    let host_id = ExecutionHostId::new("fixture.prepared.headless.local").expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = FixtureHost::scripted([SUCCESS]);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let prepared = block_on(prepare_cline_headless(
        preparation_input(host_id.clone()),
        probe(),
        services,
    ))
    .expect("Cline headless prepares");
    assert_eq!(
        discovery
            .observed_process()
            .expect("version probe ran")
            .arguments,
        ["--version"]
    );
    assert_eq!(
        prepared.observation().version().axis().as_str(),
        CLINE_PACKAGE_AXIS
    );
    assert_eq!(
        prepared.observation().version().version().as_str(),
        CLINE_PACKAGE_VERSION
    );
    assert_eq!(
        prepared.access_profile().endpoint_audience().as_str(),
        CLINE_LOCAL_ACCOUNT_AUDIENCE
    );
    assert!(prepared.access_profile().credential_reference().is_none());
    assert_eq!(
        prepared.instance().driver_id().as_str(),
        "swallowtail.cline.headless"
    );
    assert!(
        prepared
            .instance()
            .capabilities()
            .iter()
            .all(|(capability, _)| capability != Capability::InteractiveSession)
    );
    assert!(
        prepared.instance().capabilities().supports_constraint(
            Capability::HarnessModeSelection,
            &CapabilityConstraint::HarnessMode(HarnessMode::Plan),
        ),
        "qualified Cline headless advertises portable Plan"
    );

    let run = prepared
        .prepare_run(run_input("prompt"))
        .expect("run prepares");
    assert_prepared_operation_evidence_matches_plan(run.evidence(), run.plan());
    assert_eq!(run.harness_mode(), None);
    assert_eq!(run.request().policy().harness_mode(), None);
    assert!(
        run.plan()
            .requirements()
            .capabilities()
            .all(|required| required.capability() != Capability::HarnessModeSelection),
        "omitted prepared run must not require Plan"
    );
    assert_eq!(
        run.plan().driver_identity().id().as_str(),
        "swallowtail.cline.headless"
    );
    let versions: Vec<_> = run
        .plan()
        .interface_versions()
        .map(|binding| {
            (
                binding.axis().as_str().to_owned(),
                binding.version().as_str().to_owned(),
            )
        })
        .collect();
    assert_eq!(
        versions,
        [(
            CLINE_PACKAGE_AXIS.to_owned(),
            CLINE_PACKAGE_VERSION.to_owned()
        )]
    );
    assert!(run.plan().credential_reference().is_none());
    assert!(run.plan().model_id().is_none());
    assert!(run.plan().model_route_id().is_none());
    assert!(run.request().deadline().is_some());

    let mut handle = block_on(run.start_run(operation.services(host_id))).expect("run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal
            .output()
            .map(swallowtail_runtime::OperationContent::as_str),
        Some("Cline display text.")
    );
    assert!(!format!("{events:?}").contains("private fixture prompt"));
    assert!(!format!("{terminal:?}").contains("Cline display text"));
    let observed = operation.observed();
    assert_eq!(
        observed.arguments,
        [
            "--json",
            "--auto-approve",
            "false",
            "-c",
            FIXTURE_CWD,
            "private fixture prompt"
        ]
    );
    for forbidden in ["--acp", "--id", "--yolo", "--plan"] {
        assert!(
            !observed
                .arguments
                .iter()
                .any(|argument| argument == forbidden)
        );
    }
    assert!(
        !observed
            .arguments
            .windows(2)
            .any(|pair| pair == ["--auto-approve", "true"])
    );
    assert!(operation.stdin().is_empty());
    assert!(operation.stdin_closed());
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(operation.joined());
}
