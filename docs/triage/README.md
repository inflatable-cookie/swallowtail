# Triage

Temporary capture buffer for observations, ideas, plans, and questions that are
useful but not yet settled into the Swallowtail planning spine.

## What Belongs Here

Triage notes are deliberately lightweight Markdown. There is no required
frontmatter or body shape. Capture the useful thought before following one
branch of a conversation deeply.

A note may contain several related threads, tentative interpretations, links,
or a possible canonical destination. Triage is an intake buffer, not execution
authority: do not treat an idea here as an approved contract, roadmap item, or
implementation request.

The directory holds unresolved current meaning, not history. Its disposition
rules live in `docs/contracts/001-working-rules.md` and the Northstar
chatterbox doctrine; this file is a stable anchor, not an index of the notes
beside it.

## Naming

One unique note per issue, named `YYYYMMDD-HHMMSS-<slug>.md` or dated by issue.
Use a short lowercase kebab-case slug. If the same timestamp and slug already
exists, append `-2`, `-3`, and so on. The filename records when the note was
captured; do not rename it because the note was edited later.

Update a note in place when later conversation corrects or develops the same
issue. Never create a correction, addendum, or deprecation note solely to
supersede a line in another open note.

## Lifecycle

Every note needs a disposition:

1. promote or rework useful content into its canonical home;
2. delete a fully promoted note in the same coherent promotion commit;
3. edit a partially promoted note down to only its unresolved remainder;
4. merge duplicates and remove notes that are implemented, superseded, or no
   longer useful.

Keeping a note open is an interim state, not a permanent home. Git history and
`docs/logs/` preserve provenance after a note is corrected or removed, so
promoted gate records, audit findings, and census material belong in
`docs/logs/` rather than here.

## Chatterbox Shared-Checkout Rule

Chatterboxes share the working checkout. For triage capture they isolate
commits to the exact note paths, verify a clean pre-stage index, and **do not
modify this `README.md`**. If the destination, ownership, or removal decision
for a note is unclear, ask the operator before changing or deleting it.
