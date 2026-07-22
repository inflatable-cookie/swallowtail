use zeroize::Zeroize;

pub(crate) struct ProviderSessionHandle(String);

impl ProviderSessionHandle {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub(crate) fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for ProviderSessionHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ProviderSessionHandle(<redacted>)")
    }
}

impl Drop for ProviderSessionHandle {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[derive(Default)]
pub(super) struct ResumptionState {
    latest: Option<ProviderSessionHandle>,
}

#[derive(Default)]
pub(crate) struct RolloverState {
    resumption: ResumptionState,
    pending: bool,
    warning: Option<ProviderTimeLeft>,
    completed: u32,
}

struct ProviderTimeLeft(String);

impl RolloverState {
    pub(crate) fn update(&mut self, resumable: bool, handle: Option<ProviderSessionHandle>) {
        self.resumption.update(resumable, handle);
    }

    pub(crate) fn warn(&mut self, time_left: String) {
        self.pending = true;
        self.warning = Some(ProviderTimeLeft(time_left));
    }

    pub(crate) const fn pending(&self) -> bool {
        self.pending
    }

    pub(crate) const fn exhausted(&self) -> bool {
        self.completed >= 1
    }

    pub(crate) fn handle(&self) -> Option<&ProviderSessionHandle> {
        self.resumption.latest()
    }

    pub(crate) fn complete(&mut self) {
        self.completed += 1;
        self.pending = false;
        self.warning = None;
    }

    pub(crate) fn clear(&mut self) {
        self.resumption.clear();
        self.pending = false;
        self.warning = None;
    }

    #[cfg(test)]
    pub(super) fn warning_debug(&self) -> Option<String> {
        self.warning.as_ref().map(|warning| format!("{warning:?}"))
    }
}

impl std::fmt::Debug for ProviderTimeLeft {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ProviderTimeLeft(<redacted>)")
    }
}

impl Drop for ProviderTimeLeft {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl ResumptionState {
    pub(super) fn update(&mut self, resumable: bool, handle: Option<ProviderSessionHandle>) {
        self.latest = resumable.then_some(handle).flatten();
    }

    pub(super) fn latest(&self) -> Option<&ProviderSessionHandle> {
        self.latest.as_ref()
    }

    pub(super) fn clear(&mut self) {
        self.latest = None;
    }
}
