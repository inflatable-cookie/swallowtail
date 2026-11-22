#[derive(Clone, Copy)]
enum FakeMode {
    Success,
    WaitForCancellation,
    EmptyDeltaThenFlood,
    EmptyOutput,
    OversizedOutput,
}

struct FakeExecutor {
    mode: FakeMode,
    calls: Arc<AtomicUsize>,
}

impl SdkExecutor for FakeExecutor {
    fn execute(
        &self,
        invocation: SdkInvocation,
        mut updates: mpsc::Sender<Result<StreamUpdate, RuntimeFailure>>,
        mut cancelled: watch::Receiver<bool>,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let mode = self.mode;
        let calls = Arc::clone(&self.calls);
        Box::pin(async move {
            calls.fetch_add(1, Ordering::SeqCst);
            assert_eq!(invocation.endpoint, "https://bedrock-runtime.fixture");
            assert_eq!(invocation.region.as_str(), "eu-west-2");
            assert_eq!(invocation.model, "anthropic.claude-fixture-v1:0");
            assert_eq!(invocation.prompt, "fixture prompt");
            assert_eq!(invocation.maximum_output_tokens, 64);
            match mode {
                FakeMode::Success => {
                    for update in [
                        StreamUpdate::MessageStarted,
                        StreamUpdate::TextDelta("Hello ".to_owned()),
                        StreamUpdate::TextDelta("Bedrock".to_owned()),
                        StreamUpdate::ContentBlockStopped,
                        StreamUpdate::MessageStopped(crate::StopKind::EndTurn),
                        StreamUpdate::Usage(crate::TokenUsage { input: 2, output: 2, total: 4 }),
                    ] {
                        updates.send(Ok(update)).await.map_err(|_| failure("fixture.receiver_closed", "Fixture receiver closed"))?;
                    }
                }
                FakeMode::WaitForCancellation => {
                    while !*cancelled.borrow() {
                        if cancelled.changed().await.is_err() { break; }
                    }
                }
                FakeMode::EmptyDeltaThenFlood => {
                    let _ = updates.send(Ok(StreamUpdate::MessageStarted)).await;
                    let _ = updates.send(Ok(StreamUpdate::TextDelta(String::new()))).await;
                    // Flood past EVENT_CAPACITY (64) without checking `cancelled`;
                    // `pump_run` must drop its receiver before awaiting this future.
                    for _ in 0..128 {
                        if updates.send(Ok(StreamUpdate::TextDelta("x".to_owned()))).await.is_err() {
                            break;
                        }
                    }
                }
                FakeMode::EmptyOutput => {
                    let _ = updates.send(Ok(StreamUpdate::MessageStarted)).await;
                    let _ = updates
                        .send(Ok(StreamUpdate::MessageStopped(crate::StopKind::EndTurn)))
                        .await;
                }
                FakeMode::OversizedOutput => {
                    let _ = updates.send(Ok(StreamUpdate::MessageStarted)).await;
                    let chunk = "a".repeat(1024 * 1024);
                    for _ in 0..5 {
                        if updates.send(Ok(StreamUpdate::TextDelta(chunk.clone()))).await.is_err() {
                            break;
                        }
                    }
                }
            }
            Ok(())
        })
    }
}
