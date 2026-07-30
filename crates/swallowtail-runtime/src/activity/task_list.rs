use super::InvalidActivityRecord;
use crate::OperationContent;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TaskListItemStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TaskListItemPriority {
    High,
    Medium,
    Low,
}

#[derive(Clone, Eq, PartialEq)]
pub struct TaskListItem {
    content: OperationContent,
    status: TaskListItemStatus,
    priority: Option<TaskListItemPriority>,
}

impl TaskListItem {
    #[must_use]
    pub const fn new(content: OperationContent, status: TaskListItemStatus) -> Self {
        Self {
            content,
            status,
            priority: None,
        }
    }

    #[must_use]
    pub const fn with_priority(mut self, priority: TaskListItemPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    #[must_use]
    pub const fn content(&self) -> &OperationContent {
        &self.content
    }

    #[must_use]
    pub const fn status(&self) -> TaskListItemStatus {
        self.status
    }

    #[must_use]
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

#[derive(Clone, Eq, PartialEq)]
pub struct TaskListSnapshot {
    items: Vec<TaskListItem>,
}

impl TaskListSnapshot {
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

    pub fn items(&self) -> impl ExactSizeIterator<Item = &TaskListItem> {
        self.items.iter()
    }

    #[must_use]
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
