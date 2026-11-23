The checked-revocation test in `acp_guard_prepared_child.rs` now counts only the
Node process and group owner that its own fixture started, instead of every
child of the test process. Sibling tests in the same libtest binary spawn
children too, so the old count failed under default parallel threads. The
observation no longer depends on test scheduling, and the revocation and
child-joining assertions are unchanged.
