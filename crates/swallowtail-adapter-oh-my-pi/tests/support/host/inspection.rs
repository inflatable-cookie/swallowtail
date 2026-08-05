use super::{CleanupEvent, FixtureHost};
use serde_json::Value;

impl FixtureHost {
    pub fn wait_for_input(&self, kind: &str) {
        let mut state = self
            .shared
            .process
            .lock()
            .expect("OhMyPi fixture state lock poisoned");
        while !state
            .input
            .iter()
            .any(|value| value.get("type").and_then(Value::as_str) == Some(kind))
        {
            state = self
                .shared
                .changed
                .wait(state)
                .expect("OhMyPi fixture wait lock poisoned");
        }
    }

    #[allow(dead_code)]
    pub fn process_started(&self) -> bool {
        self.shared
            .process_request
            .lock()
            .expect("OhMyPi fixture process lock poisoned")
            .is_some()
    }

    pub fn process_arguments(&self) -> Vec<String> {
        self.shared
            .process_request
            .lock()
            .expect("OhMyPi fixture process lock poisoned")
            .as_ref()
            .expect("OhMyPi fixture process started")
            .arguments()
            .map(str::to_owned)
            .collect()
    }

    pub fn process_environment(&self) -> Vec<String> {
        self.shared
            .process_request
            .lock()
            .expect("OhMyPi fixture process lock poisoned")
            .as_ref()
            .expect("OhMyPi fixture process started")
            .environment()
            .map(|value| value.as_host_value().to_owned())
            .collect()
    }

    pub fn process_executable(&self) -> String {
        self.shared
            .process_request
            .lock()
            .expect("OhMyPi fixture process lock poisoned")
            .as_ref()
            .expect("OhMyPi fixture process started")
            .executable()
            .as_host_value()
            .to_owned()
    }

    pub fn process_working_resource(&self) -> String {
        self.shared
            .process_request
            .lock()
            .expect("OhMyPi fixture process lock poisoned")
            .as_ref()
            .expect("OhMyPi fixture process started")
            .working_resource()
            .expect("OhMyPi fixture working resource")
            .as_host_value()
            .to_owned()
    }

    pub fn inputs(&self) -> Vec<Value> {
        self.shared
            .process
            .lock()
            .expect("OhMyPi fixture state lock poisoned")
            .input
            .clone()
    }

    pub fn cleanup_events(&self) -> Vec<CleanupEvent> {
        self.shared
            .cleanup
            .lock()
            .expect("OhMyPi fixture cleanup lock poisoned")
            .clone()
    }
}
