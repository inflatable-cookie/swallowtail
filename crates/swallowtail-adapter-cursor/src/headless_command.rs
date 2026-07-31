use swallowtail_core::{ModelId, ResourceAccess};

pub(crate) fn arguments(model: &ModelId, access: ResourceAccess) -> Vec<String> {
    let mut arguments = [
        "--print",
        "--output-format",
        "stream-json",
        "--model",
        model.as_str(),
        "--trust",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if access == ResourceAccess::Read {
        arguments.extend(["--mode".to_owned(), "plan".to_owned()]);
    }
    arguments
}

#[cfg(test)]
mod tests {
    use super::arguments;
    use swallowtail_core::{ModelId, ResourceAccess};

    #[test]
    fn read_and_write_profiles_never_select_force_or_implicit_sandboxing() {
        let model = ModelId::new("fixture-model").expect("model");
        let read = arguments(&model, ResourceAccess::Read);
        let write = arguments(&model, ResourceAccess::ReadWrite);
        assert!(read.ends_with(&["--mode".to_owned(), "plan".to_owned()]));
        assert!(!write.iter().any(|value| value == "--mode"));
        for arguments in [&read, &write] {
            assert!(arguments.iter().any(|value| value == "--trust"));
            for rejected in ["--force", "--yolo", "--sandbox", "--stream-partial-output"] {
                assert!(!arguments.iter().any(|value| value == rejected));
            }
        }
    }
}
