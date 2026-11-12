Affected-package proof allocates its temporary workspace beneath the configured
scratch root, including on BSD `mktemp`, and keeps cleanup bounded to that
workspace.
