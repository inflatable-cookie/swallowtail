/// Host-supplied ZCode app-server session mode. Swallowtail does not default `yolo`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZcodeAppServerMode(String);

impl ZcodeAppServerMode {
    /// Accepts only host-supplied `plan` or `build`.
    #[must_use]
    pub fn new(value: &str) -> Option<Self> {
        match value {
            "plan" | "build" => Some(Self(value.to_owned())),
            _ => None,
        }
    }

    #[must_use]
    /// Returns the read-only planning mode.
    pub fn plan() -> Self {
        Self::new("plan").expect("plan is an admitted mode")
    }

    #[must_use]
    /// Returns the host-supplied build mode.
    pub fn build() -> Self {
        Self::new("build").expect("build is an admitted mode")
    }

    #[must_use]
    /// Returns the exact admitted mode text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ZcodeAppServerMode;

    #[test]
    fn yolo_and_unknown_modes_are_not_admitted() {
        assert_eq!(ZcodeAppServerMode::plan().as_str(), "plan");
        assert_eq!(ZcodeAppServerMode::build().as_str(), "build");
        for rejected in ["", "yolo", "Yolo", "plan ", "ask", "default"] {
            assert!(ZcodeAppServerMode::new(rejected).is_none(), "{rejected}");
        }
    }
}
