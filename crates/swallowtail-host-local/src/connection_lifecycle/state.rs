use std::collections::BTreeMap;
use swallowtail_core::{AdmittedInstanceRecord, ConfiguredInstanceId, OverlayMarker};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct StoreState {
    pub instances: BTreeMap<String, AdmittedInstanceRecord>,
    pub overlays: BTreeMap<(String, String, String), OverlayMarker>,
}

impl StoreState {
    pub(super) fn put_instance(&mut self, record: AdmittedInstanceRecord) {
        self.instances
            .insert(record.id().as_str().to_owned(), record);
    }

    pub(super) fn get_instance(&self, id: &ConfiguredInstanceId) -> Option<AdmittedInstanceRecord> {
        self.instances.get(id.as_str()).cloned()
    }

    pub(super) fn list_instances(&self) -> Vec<AdmittedInstanceRecord> {
        self.instances.values().cloned().collect()
    }

    pub(super) fn put_overlay_marker(&mut self, marker: OverlayMarker) {
        self.overlays.insert(overlay_key(&marker), marker);
    }

    pub(super) fn list_overlay_markers(&self) -> Vec<OverlayMarker> {
        self.overlays.values().cloned().collect()
    }
}

pub(super) fn overlay_key(marker: &OverlayMarker) -> (String, String, String) {
    (
        marker.instance_id().as_str().to_owned(),
        marker.provider_id().as_str().to_owned(),
        marker.model_id().as_str().to_owned(),
    )
}
