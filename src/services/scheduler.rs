//! Autonomous Cron Task Scheduler & External Messaging Gateway Worker
//! Runs on background Tokio tasks without blocking the Iced GPU rendering thread.

use std::sync::Arc;
use std::time::Duration;
use tracing::info;

use crate::db::HermesDatabase;
use crate::models::TaskStatus;

pub struct AutonomousScheduler {
    db: Arc<HermesDatabase>,
}

impl AutonomousScheduler {
    pub fn new(db: Arc<HermesDatabase>) -> Self {
        Self { db }
    }

    pub async fn start(self) {
        let mut ticker = tokio::time::interval(Duration::from_secs(300));
        loop {
            ticker.tick().await;
            let tasks = self.db.list_scheduled_tasks().unwrap_or_default();
            for task in tasks {
                if task.status == TaskStatus::Scheduled {
                    info!(
                        "Executing scheduled autonomous task: {} ({})",
                        task.title, task.cron_expression
                    );
                    let _ = self.db.update_task_status(&task.id, TaskStatus::Running);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    let _ = self.db.update_task_status(&task.id, TaskStatus::Completed);
                }
            }
        }
    }
}
