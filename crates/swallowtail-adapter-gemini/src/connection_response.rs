fn optional_usize(params: &Value, field: &str) -> Result<Option<usize>, RuntimeFailure> {
    params
        .get(field)
        .map(|value| {
            value
                .as_u64()
                .and_then(|value| usize::try_from(value).ok())
                .ok_or_else(malformed)
        })
        .transpose()
}

struct ResponseState {
    result: Option<Result<Value, RuntimeFailure>>,
    waiter: Option<Waker>,
}

struct ResponseSender(Arc<Mutex<ResponseState>>);

impl ResponseSender {
    fn complete(self, result: Result<Value, RuntimeFailure>) {
        let mut state = self.0.lock().expect("ACP response lock poisoned");
        state.result = Some(result);
        if let Some(waiter) = state.waiter.take() {
            waiter.wake();
        }
    }
}

struct ResponseFuture(Arc<Mutex<ResponseState>>);

impl Future for ResponseFuture {
    type Output = Result<Value, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("ACP response lock poisoned");
        if let Some(result) = state.result.take() {
            Poll::Ready(result)
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

pub(crate) struct PendingResponse(ResponseFuture);

impl Future for PendingResponse {
    type Output = Result<Value, RuntimeFailure>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(context)
    }
}

fn response_channel() -> (ResponseSender, ResponseFuture) {
    let state = Arc::new(Mutex::new(ResponseState {
        result: None,
        waiter: None,
    }));
    (ResponseSender(Arc::clone(&state)), ResponseFuture(state))
}
