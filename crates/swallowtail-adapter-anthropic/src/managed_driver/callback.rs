const CALLBACK_CAPACITY: usize = 64;

struct PendingCallback {
    operation_id: swallowtail_runtime::CallbackOperationId,
    result: Option<CallbackResult>,
}

struct CallbackState {
    requests: VecDeque<CallbackRequest>,
    pending: BTreeMap<CallbackId, PendingCallback>,
    by_provider_event: BTreeMap<String, CallbackId>,
    closed: bool,
    waiter: Option<Waker>,
}

#[derive(Clone)]
pub(super) struct ManagedCallbackHub {
    state: Arc<Mutex<CallbackState>>,
}

impl ManagedCallbackHub {
    pub(super) fn new() -> (Self, CallbackExchange) {
        let state = Arc::new(Mutex::new(CallbackState {
            requests: VecDeque::new(),
            pending: BTreeMap::new(),
            by_provider_event: BTreeMap::new(),
            closed: false,
            waiter: None,
        }));
        let requests: BoxCallbackStream = Box::pin(CallbackRequestStream {
            state: Arc::clone(&state),
        });
        let responder: Arc<dyn CallbackResponder> = Arc::new(ManagedCallbackResponder {
            state: Arc::clone(&state),
        });
        (Self { state }, CallbackExchange::new(requests, responder))
    }

    pub(super) fn enqueue(
        &self,
        provider_event_id: String,
        request: CallbackRequest,
    ) -> Result<(), RuntimeFailure> {
        let mut state = self.state.lock().expect("managed callback lock poisoned");
        if state.closed {
            return Err(closed());
        }
        if state.pending.len() >= CALLBACK_CAPACITY || state.requests.len() >= CALLBACK_CAPACITY {
            return Err(callback_failure(
                "capacity_exceeded",
                "Anthropic Managed Agents exceeded the bounded callback capacity",
            ));
        }
        if state.by_provider_event.contains_key(&provider_event_id)
            || state.pending.contains_key(request.callback_id())
        {
            return Err(callback_failure(
                "identity_reused",
                "Anthropic Managed Agents reused a callback identity",
            ));
        }
        state
            .by_provider_event
            .insert(provider_event_id.clone(), request.callback_id().clone());
        state.pending.insert(
            request.callback_id().clone(),
            PendingCallback {
                operation_id: request.operation_id().clone(),
                result: None,
            },
        );
        state.requests.push_back(request);
        wake(&mut state);
        Ok(())
    }

    pub(super) async fn wait_for(
        &self,
        provider_event_ids: &[String],
    ) -> Result<Vec<(String, CallbackResult)>, RuntimeFailure> {
        poll_fn(|context| {
            let mut state = self.state.lock().expect("managed callback lock poisoned");
            if state.closed {
                return Poll::Ready(Err(closed()));
            }
            let mut ready = Vec::with_capacity(provider_event_ids.len());
            for provider_event_id in provider_event_ids {
                let Some(callback_id) = state.by_provider_event.get(provider_event_id) else {
                    return Poll::Ready(Err(callback_failure(
                        "unknown_provider_event",
                        "Anthropic Managed Agents requested an unknown callback",
                    )));
                };
                let pending = state.pending.get(callback_id).expect("callback index agrees");
                let Some(result) = pending.result.clone() else {
                    state.waiter = Some(context.waker().clone());
                    return Poll::Pending;
                };
                ready.push((provider_event_id.clone(), result));
            }
            for provider_event_id in provider_event_ids {
                let callback_id = state
                    .by_provider_event
                    .remove(provider_event_id)
                    .expect("ready callback remains indexed");
                state.pending.remove(&callback_id);
            }
            Poll::Ready(Ok(ready))
        })
        .await
    }

    pub(super) fn abandon(&self, _reason: CallbackAbandonment) {
        let mut state = self.state.lock().expect("managed callback lock poisoned");
        state.closed = true;
        state.requests.clear();
        state.pending.clear();
        state.by_provider_event.clear();
        wake(&mut state);
    }
}

struct CallbackRequestStream {
    state: Arc<Mutex<CallbackState>>,
}

impl Stream for CallbackRequestStream {
    type Item = Result<CallbackRequest, RuntimeFailure>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self.state.lock().expect("managed callback lock poisoned");
        if let Some(request) = state.requests.pop_front() {
            Poll::Ready(Some(Ok(request)))
        } else if state.closed {
            Poll::Ready(None)
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

struct ManagedCallbackResponder {
    state: Arc<Mutex<CallbackState>>,
}

impl CallbackResponder for ManagedCallbackResponder {
    fn respond(&self, response: CallbackResponse) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let mut state = state.lock().expect("managed callback lock poisoned");
            if state.closed {
                return Err(closed());
            }
            let pending = state.pending.get_mut(response.callback_id()).ok_or_else(|| {
                callback_failure(
                    "unknown_or_duplicate",
                    "Anthropic Managed Agents callback response was unknown or already used",
                )
            })?;
            if &pending.operation_id != response.operation_id() {
                return Err(callback_failure(
                    "operation_mismatch",
                    "Anthropic Managed Agents callback response belongs to another operation",
                ));
            }
            if pending.result.is_some() {
                return Err(callback_failure(
                    "unknown_or_duplicate",
                    "Anthropic Managed Agents callback response was unknown or already used",
                ));
            }
            pending.result = Some(response.result().clone());
            wake(&mut state);
            Ok(())
        })
    }
}

fn closed() -> RuntimeFailure {
    callback_failure(
        "closed",
        "Anthropic Managed Agents callback exchange is closed",
    )
}

fn callback_failure(suffix: &str, message: &str) -> RuntimeFailure {
    let code = match suffix {
        "capacity_exceeded" => "swallowtail.anthropic.managed.callback_capacity_exceeded",
        "identity_reused" => "swallowtail.anthropic.managed.callback_identity_reused",
        "unknown_provider_event" => {
            "swallowtail.anthropic.managed.callback_unknown_provider_event"
        }
        "unknown_or_duplicate" => {
            "swallowtail.anthropic.managed.callback_unknown_or_duplicate"
        }
        "operation_mismatch" => "swallowtail.anthropic.managed.callback_operation_mismatch",
        _ => "swallowtail.anthropic.managed.callback_closed",
    };
    failure(code, message)
}

fn wake(state: &mut CallbackState) {
    if let Some(waiter) = state.waiter.take() {
        waiter.wake();
    }
}
