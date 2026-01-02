// crates/a2a-server/src/store.rs
use std::collections::HashMap;
use std::future::Future;
use std::sync::RwLock;

/// Filter for listing tasks.
#[derive(Debug, Clone, Default)]
pub struct TaskFilter {
    pub context_id: Option<String>,
    pub status: Option<String>,
}

/// Task storage trait.
pub trait TaskStore: Send + Sync {
    /// Error type for this store.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Get a task by ID.
    fn get(
        &self,
        task_id: &str,
    ) -> impl Future<Output = Result<Option<serde_json::Value>, Self::Error>> + Send;

    /// Save a task.
    fn save(
        &self,
        task: &serde_json::Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// List tasks matching the filter.
    fn list(
        &self,
        filter: TaskFilter,
    ) -> impl Future<Output = Result<Vec<serde_json::Value>, Self::Error>> + Send;

    /// Delete a task.
    fn delete(
        &self,
        task_id: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

/// In-memory task store.
#[derive(Debug, Default)]
pub struct InMemoryTaskStore {
    tasks: RwLock<HashMap<String, serde_json::Value>>,
}

impl InMemoryTaskStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TaskStore for InMemoryTaskStore {
    type Error = std::convert::Infallible;

    async fn get(&self, task_id: &str) -> Result<Option<serde_json::Value>, Self::Error> {
        let tasks = self.tasks.read().unwrap();
        Ok(tasks.get(task_id).cloned())
    }

    async fn save(&self, task: &serde_json::Value) -> Result<(), Self::Error> {
        if let Some(id) = task.get("id").and_then(|v| v.as_str()) {
            let mut tasks = self.tasks.write().unwrap();
            tasks.insert(id.to_string(), task.clone());
        }
        Ok(())
    }

    async fn list(&self, filter: TaskFilter) -> Result<Vec<serde_json::Value>, Self::Error> {
        let tasks = self.tasks.read().unwrap();
        let result: Vec<_> = tasks
            .values()
            .filter(|task| {
                if let Some(ref ctx) = filter.context_id
                    && task.get("context_id").and_then(|v| v.as_str()) != Some(ctx)
                {
                    return false;
                }
                if let Some(ref status) = filter.status
                    && task.get("status").and_then(|v| v.get("state")).and_then(|v| v.as_str()) != Some(status)
                {
                    return false;
                }
                true
            })
            .cloned()
            .collect();
        Ok(result)
    }

    async fn delete(&self, task_id: &str) -> Result<(), Self::Error> {
        let mut tasks = self.tasks.write().unwrap();
        tasks.remove(task_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_in_memory_store() {
        let store = InMemoryTaskStore::new();

        let task = json!({
            "id": "task-1",
            "context_id": "ctx-1",
            "status": { "state": "submitted" }
        });

        // Save
        store.save(&task).await.unwrap();

        // Get
        let retrieved = store.get("task-1").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap()["id"], "task-1");

        // List
        let all = store.list(TaskFilter::default()).await.unwrap();
        assert_eq!(all.len(), 1);

        // Delete
        store.delete("task-1").await.unwrap();
        let deleted = store.get("task-1").await.unwrap();
        assert!(deleted.is_none());
    }
}
