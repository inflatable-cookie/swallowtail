use super::InvalidActivityRecord;
use crate::OperationContent;
use std::fmt;

/// Portable status of one provider task-list item.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TaskListItemStatus {
    /// Not yet started.
    Pending,
    /// Currently in progress.
    InProgress,
    /// Finished.
    Completed,
}

/// Optional provider-reported task priority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TaskListItemPriority {
    /// High priority.
    High,
    /// Medium priority.
    Medium,
    /// Low priority.
    Low,
}

/// One ordered task in a provider-authoritative replacement snapshot.
#[derive(Clone, Eq, PartialEq)]
pub struct TaskListItem {
    content: OperationContent,
    status: TaskListItemStatus,
    priority: Option<TaskListItemPriority>,
}

impl TaskListItem {
    /// Creates a task with content and status but no priority.
    #[must_use]
    pub const fn new(content: OperationContent, status: TaskListItemStatus) -> Self {
        Self {
            content,
            status,
            priority: None,
        }
    }

    #[must_use]
    /// Adds the provider-reported priority.
    pub const fn with_priority(mut self, priority: TaskListItemPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    #[must_use]
    /// Returns the potentially sensitive task content.
    pub const fn content(&self) -> &OperationContent {
        &self.content
    }

    #[must_use]
    /// Returns the task status.
    pub const fn status(&self) -> TaskListItemStatus {
        self.status
    }

    #[must_use]
    /// Returns the provider priority when supplied.
    pub const fn priority(&self) -> Option<TaskListItemPriority> {
        self.priority
    }
}

impl fmt::Debug for TaskListItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TaskListItem")
            .field(
                "content",
                &format_args!("<redacted:{} bytes>", self.content.byte_len()),
            )
            .field("status", &self.status)
            .field("priority", &self.priority)
            .finish()
    }
}

/// Bounded full replacement of the visible provider task list.
///
/// An empty snapshot explicitly clears the list. Items have no portable
/// durable identity beyond their position in this observation.
#[derive(Clone, Eq, PartialEq)]
pub struct TaskListSnapshot {
    items: Vec<TaskListItem>,
}

impl TaskListSnapshot {
    /// Validates item count and aggregate content bounds before construction.
    pub fn new(
        items: impl IntoIterator<Item = TaskListItem>,
        maximum_items: usize,
        maximum_content_bytes: usize,
    ) -> Result<Self, InvalidActivityRecord> {
        let items = items.into_iter().collect::<Vec<_>>();
        if items.len() > maximum_items {
            return Err(InvalidActivityRecord::new(
                "Task-list snapshot exceeds its item-count bound",
            ));
        }
        let content_bytes = items
            .iter()
            .try_fold(0_usize, |total, item| {
                total.checked_add(item.content.byte_len())
            })
            .ok_or_else(|| {
                InvalidActivityRecord::new("Task-list snapshot content size overflowed")
            })?;
        if content_bytes > maximum_content_bytes {
            return Err(InvalidActivityRecord::new(
                "Task-list snapshot exceeds its content bound",
            ));
        }
        Ok(Self { items })
    }

    /// Iterates over tasks in provider order.
    pub fn items(&self) -> impl ExactSizeIterator<Item = &TaskListItem> {
        self.items.iter()
    }

    #[must_use]
    /// Returns whether this snapshot explicitly clears the task list.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl fmt::Debug for TaskListSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TaskListSnapshot")
            .field(
                "items",
                &format_args!("<{} redacted items>", self.items.len()),
            )
            .finish()
    }
}
