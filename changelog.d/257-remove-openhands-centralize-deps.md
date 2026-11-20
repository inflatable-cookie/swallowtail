Remove `swallowtail-adapter-openhands`. It contributed no production route:
`start_run` failed closed and live HTTP/WebSocket conversation never arrived, so
the package added release, metadata, API and QA weight for a route the matrix
does not carry. The workspace drops to 39 packages and 89 internal dependency
edges; all 51 production routes stay intact, and every tag that released the
package keeps its inventory.

Centralize the shared external crate requirements (`base64`, `curl`,
`futures-channel`, `futures-core`, `futures-executor`, `futures-util`,
`semver`, `serde_json`, `sha2`, `tokio`, `tungstenite`, `url`, `zeroize`) in
root `[workspace.dependencies]` and inherit them with `workspace = true`, so
`sha2 = "0.10.9"` versus `"0.10"` and `url = "2.5"` versus `"2"` drift cannot
recur. Crate-local feature selections stay local; no locked version moves.
