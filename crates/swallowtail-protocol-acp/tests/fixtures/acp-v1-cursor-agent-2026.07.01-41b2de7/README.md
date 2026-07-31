# Cursor Agent `2026.07.01-41b2de7` ACP Evidence

Captured 2026-07-31 from the exact installed Cursor Agent executable already
authenticated by the operator.

The probe sent one ACP `initialize` request with read-only filesystem client
capabilities, then closed stdin and joined the process. It sent no
`authenticate`, `session/new`, prompt, model request, tool request, login,
update, or installation request. No provider session or workspace mutation was
created.

The model catalogue was observed through the executable's dedicated `models`
command. Only its normalized count and digest are retained here. The account
identity is omitted. Catalogue membership is auth-aware discovery evidence,
not proof that any listed model can execute.

Headless shapes come from Cursor's official output-format documentation, the
exact installed help surface, and the installed source identified in
`headless-source.json`. `headless-derived.jsonl` freezes the bounded event and
correlation shapes used by the structured driver. It is not a live prompt
capture. Provider tool arguments and results are deliberately private.

The interactive corpus is source-derived, not a live prompt capture. It was
normalized from the exact installed bundle identified in
`interactive-source.json`. The corpus freezes only methods, update shapes,
correlation fields, and stop reasons implemented by that artifact. Provider
payloads and account data are absent.
