/// Configured Claude authentication a watcher command still admits.
#[derive(Debug, Eq, PartialEq)]
enum FixtureAuthenticationAdmission {
    /// No mode flag narrows credentials, so the host's configured path applies.
    HostConfigured,
    /// `--bare` never reads OAuth or keychain state.
    ApiKeyOrHelperOnly,
}

/// One named ambient authority axis from the card 029 invariant.
///
/// Fail closed: an axis is `Excluded` only where exact `2.1.251` help states the
/// exclusion. Anything help leaves unstated stays `Admitted`.
#[derive(Debug, Eq, PartialEq)]
enum FixtureAmbientAdmission {
    Excluded,
    Admitted,
}

/// One operation-private watcher part the invariant requires the command to keep.
#[derive(Debug, Eq, PartialEq)]
enum FixtureCompositionAdmission {
    Preserved,
    Unstated,
    Disabled,
}

#[derive(Debug, Eq, PartialEq)]
struct FixtureAmbientAuthority {
    settings: FixtureAmbientAdmission,
    hooks: FixtureAmbientAdmission,
    skills: FixtureAmbientAdmission,
    mcp_servers: FixtureAmbientAdmission,
    memory_claude_md: FixtureAmbientAdmission,
    plugins: FixtureAmbientAdmission,
}

impl FixtureAmbientAuthority {
    fn excludes_every_named_axis(&self) -> bool {
        [
            &self.settings,
            &self.hooks,
            &self.skills,
            &self.mcp_servers,
            &self.memory_claude_md,
            &self.plugins,
        ]
        .into_iter()
        .all(|axis| *axis == FixtureAmbientAdmission::Excluded)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct FixturePrivateComposition {
    private_mcp: FixtureCompositionAdmission,
    stop_hook: FixtureCompositionAdmission,
    injected_skill: FixtureCompositionAdmission,
}

impl FixturePrivateComposition {
    fn is_wholly_preserved(&self) -> bool {
        [&self.private_mcp, &self.stop_hook, &self.injected_skill]
            .into_iter()
            .all(|part| *part == FixtureCompositionAdmission::Preserved)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct FixtureIsolation {
    authentication: FixtureAuthenticationAdmission,
    ambient: FixtureAmbientAuthority,
    composition: FixturePrivateComposition,
}

impl FixtureIsolation {
    /// The card 029 invariant: one configured authentication path, the whole
    /// private composition, and none of the named ambient authority.
    fn satisfies_the_review_oracle(&self) -> bool {
        self.authentication == FixtureAuthenticationAdmission::HostConfigured
            && self.ambient.excludes_every_named_axis()
            && self.composition.is_wholly_preserved()
    }
}

fn admission(excluded: bool) -> FixtureAmbientAdmission {
    if excluded {
        FixtureAmbientAdmission::Excluded
    } else {
        FixtureAmbientAdmission::Admitted
    }
}

fn part(present: bool, unstated: bool, disabled: bool) -> FixtureCompositionAdmission {
    if disabled || !present {
        FixtureCompositionAdmission::Disabled
    } else if unstated {
        FixtureCompositionAdmission::Unstated
    } else {
        FixtureCompositionAdmission::Preserved
    }
}

/// Classify one exact argv against the invariant using only help-stated exclusions.
fn fixture_isolation(arguments: &[String]) -> FixtureIsolation {
    let names = |flag: &str| arguments.iter().any(|argument| argument == flag);
    let value = |flag: &str| {
        arguments
            .iter()
            .position(|argument| argument == flag)
            .and_then(|index| arguments.get(index + 1))
            .map(String::as_str)
    };
    let bare = names("--bare");
    let restricted = names("--restricted");
    let safe_mode = names("--safe-mode");
    let no_slash_commands = names("--disable-slash-commands");
    let empty_sources = value("--setting-sources") == Some("");
    // Fail closed: help never says a mode overrides an explicit `--setting-sources`,
    // so naming ambient sources re-admits them whatever else the argv carries.
    let ambient_sources = value("--setting-sources").is_some_and(|sources| !sources.is_empty());
    let private_mcp = value("--mcp-config").is_some_and(|config| config != r#"{"mcpServers":{}}"#);

    FixtureIsolation {
        authentication: if bare {
            FixtureAuthenticationAdmission::ApiKeyOrHelperOnly
        } else {
            FixtureAuthenticationAdmission::HostConfigured
        },
        ambient: FixtureAmbientAuthority {
            settings: admission(!ambient_sources && (restricted || safe_mode || empty_sources)),
            hooks: admission(!ambient_sources && (bare || restricted || safe_mode || empty_sources)),
            skills: admission(safe_mode || no_slash_commands),
            mcp_servers: admission(names("--strict-mcp-config") || safe_mode),
            memory_claude_md: admission(bare || safe_mode),
            plugins: admission(bare || safe_mode),
        },
        composition: FixturePrivateComposition {
            private_mcp: part(private_mcp, false, safe_mode),
            stop_hook: part(names("--settings"), bare, safe_mode),
            injected_skill: part(names("--add-dir"), false, safe_mode || no_slash_commands),
        },
    }
}

/// Rebuild one argv with `flag` swapped for `replacement`, to model a candidate.
fn replacing(arguments: &[String], flag: &str, replacement: &[&str]) -> Vec<String> {
    let index = arguments
        .iter()
        .position(|argument| argument == flag)
        .unwrap_or_else(|| panic!("{flag} is present"));
    let mut variant = arguments[..index].to_vec();
    variant.extend(replacement.iter().map(|argument| (*argument).to_owned()));
    variant.extend_from_slice(&arguments[index + 1..]);
    variant
}
