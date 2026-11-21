//! Attachment input for the Pi RPC driver. The mechanics are shared in
//! `crate::attachment_input`; this file only binds the RPC namespace.

use crate::attachment_input::{
    AttachmentFailureNamespace, PiAttachmentInput, prepare_attachment as prepare_with_namespace,
};
use swallowtail_runtime::{AttachmentDescriptor, HostServices, RuntimeFailure, ScopeId};

pub(super) use swallowtail_runtime::SharedAttachmentMaterialization;

pub(super) async fn prepare_attachment(
    descriptor: Option<AttachmentDescriptor>,
    services: &HostServices,
    scope: ScopeId,
) -> Result<Option<PiAttachmentInput>, RuntimeFailure> {
    prepare_with_namespace(descriptor, services, scope, AttachmentFailureNamespace::RPC).await
}
