use crate::output::failure;
use std::collections::{BTreeMap, BTreeSet};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_runtime::RuntimeFailure;

const DESCENDANT_SCAN_INTERVAL: Duration = Duration::from_millis(25);
const DESCENDANT_STATE_COMMAND: &str = "pid=,ppid=,pgid=,stat=";

#[derive(Clone, Copy)]
struct ProcessRow {
    parent_id: u32,
    group_id: u32,
    zombie: bool,
}

/// Observes descendants while the supervised root still preserves the parent
/// chain, then verifies that every observed descendant has stopped.
///
/// The host never signals an observed descendant by numeric PID. An escaped
/// descendant is therefore a fail-closed cleanup result: the host refuses to
/// report clean process truth when it cannot safely contain that descendant.
pub(crate) struct DescendantTracker {
    root_process_id: u32,
    owner_group_id: u32,
    seen: BTreeMap<u32, bool>,
}

impl DescendantTracker {
    pub(crate) fn new(root_process_id: u32, owner_group_id: u32) -> Self {
        Self {
            root_process_id,
            owner_group_id,
            seen: BTreeMap::new(),
        }
    }

    pub(crate) fn observe(&mut self) -> Result<(), RuntimeFailure> {
        let rows = process_rows()?;
        let mut children = BTreeMap::<u32, Vec<(u32, ProcessRow)>>::new();
        for (process_id, row) in rows {
            children
                .entry(row.parent_id)
                .or_default()
                .push((process_id, row));
        }

        let mut pending = vec![self.root_process_id];
        let mut discovered = BTreeSet::new();
        while let Some(parent_id) = pending.pop() {
            for &(process_id, row) in children.get(&parent_id).into_iter().flatten() {
                if !discovered.insert(process_id) {
                    continue;
                }
                let escaped = row.group_id != self.owner_group_id;
                self.seen
                    .entry(process_id)
                    .and_modify(|was_escaped| *was_escaped |= escaped)
                    .or_insert(escaped);
                pending.push(process_id);
            }
        }
        Ok(())
    }

    pub(crate) fn verify_stopped(&self, bound: Duration) -> Result<(), RuntimeFailure> {
        let deadline = Instant::now()
            .checked_add(bound)
            .expect("descendant verification deadline is representable");
        loop {
            let rows = process_rows()?;
            let remaining = self
                .seen
                .iter()
                .filter_map(|(process_id, escaped)| {
                    rows.get(process_id)
                        .filter(|row| !row.zombie)
                        .map(|_| (*process_id, *escaped))
                })
                .collect::<Vec<_>>();
            if remaining.iter().any(|(_, escaped)| *escaped) {
                return Err(failure(
                    "swallowtail.local_process.descendant_escape_detected",
                    "Local process cleanup observed a descendant outside its owned process group",
                ));
            }
            if remaining.is_empty() {
                return Ok(());
            }
            let now = Instant::now();
            if now >= deadline {
                return Err(failure(
                    "swallowtail.local_process.descendant_join_failed",
                    "Local process descendants could not be joined",
                ));
            }
            thread::sleep((deadline - now).min(DESCENDANT_SCAN_INTERVAL));
        }
    }
}

fn process_rows() -> Result<BTreeMap<u32, ProcessRow>, RuntimeFailure> {
    let output = Command::new("/bin/ps")
        .args(["-axo", DESCENDANT_STATE_COMMAND])
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| observation_failure())?;
    if !output.status.success() {
        return Err(observation_failure());
    }
    let text = std::str::from_utf8(&output.stdout).map_err(|_| observation_failure())?;
    let mut rows = BTreeMap::new();
    for line in text.lines() {
        let mut fields = line.split_whitespace();
        let process_id = fields
            .next()
            .and_then(|value| value.parse::<u32>().ok())
            .ok_or_else(observation_failure)?;
        let parent_id = fields
            .next()
            .and_then(|value| value.parse::<u32>().ok())
            .ok_or_else(observation_failure)?;
        let group_id = fields
            .next()
            .and_then(|value| value.parse::<u32>().ok())
            .ok_or_else(observation_failure)?;
        let state = fields.next().ok_or_else(observation_failure)?;
        rows.insert(
            process_id,
            ProcessRow {
                parent_id,
                group_id,
                zombie: state.starts_with('Z'),
            },
        );
    }
    Ok(rows)
}

fn observation_failure() -> RuntimeFailure {
    failure(
        "swallowtail.local_process.descendant_observation_failed",
        "Local process descendants could not be observed",
    )
}
