mod app_server;
mod exec;

pub(super) fn app_server_activity_profile(
    prepared: &crate::CodexPreparedIntegration,
) -> Result<swallowtail_core::ObservableActivityProfile, swallowtail_runtime::PreparationFailure> {
    app_server::app_server_activity_profile(prepared)
}

pub(super) fn exec_activity_profile(
    prepared: &crate::CodexPreparedIntegration,
) -> Result<swallowtail_core::ObservableActivityProfile, swallowtail_runtime::PreparationFailure> {
    exec::exec_activity_profile(prepared)
}
