#[derive(Clone, Copy)]
enum FakeMode {
    Success,
    ProviderFailure,
    WaitForCancellation,
}

struct FakeExecutor {
    mode: FakeMode,
    calls: Arc<AtomicUsize>,
    cancellations: Arc<AtomicUsize>,
    completions: Arc<AtomicUsize>,
    endpoints: Arc<Mutex<Vec<String>>>,
}

impl FakeExecutor {
    fn new(mode: FakeMode) -> Self {
        Self {
            mode,
            calls: Arc::new(AtomicUsize::new(0)),
            cancellations: Arc::new(AtomicUsize::new(0)),
            completions: Arc::new(AtomicUsize::new(0)),
            endpoints: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl CatalogueSdkExecutor for FakeExecutor {
    fn execute(
        &self,
        invocation: CatalogueInvocation,
        mut cancelled: watch::Receiver<bool>,
    ) -> BoxFuture<'static, Result<ListFoundationModelsOutput, RuntimeFailure>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.endpoints
            .lock()
            .expect("endpoint lock is available")
            .push(invocation.endpoint);
        assert_eq!(invocation.region.as_str(), "eu-west-2");
        assert_eq!(
            format!("{:?}", invocation.provider),
            "BedrockCredentialProvider(<opaque>)"
        );
        let mode = self.mode;
        let cancellations = Arc::clone(&self.cancellations);
        let completions = Arc::clone(&self.completions);
        Box::pin(async move {
            let result = match mode {
                FakeMode::Success => Ok(ListFoundationModelsOutput::builder()
                    .model_summaries(
                        aws_sdk_bedrock::types::FoundationModelSummary::builder()
                            .model_arn("private-resource")
                            .model_id("provider.model-v1")
                            .model_name("Model One")
                            .build()
                            .expect("summary builds"),
                    )
                    .build()),
                FakeMode::ProviderFailure => Err(failure(
                    "swallowtail.bedrock.catalogue_unavailable",
                    "Bedrock catalogue was unavailable",
                )),
                FakeMode::WaitForCancellation => {
                    let _ = cancelled.changed().await;
                    cancellations.fetch_add(1, Ordering::SeqCst);
                    Err(failure(
                        "swallowtail.bedrock.catalogue_cancelled",
                        "Bedrock catalogue work was cancelled",
                    ))
                }
            };
            completions.fetch_add(1, Ordering::SeqCst);
            result
        })
    }
}
