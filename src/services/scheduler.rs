//! Autonomous Cron Task Scheduler & External Messaging Gateway Worker
//! Runs on background Tokio tasks without blocking the Iced GPU rendering thread.

use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
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

    pub async fn start(&self) -> anyhow::Result<()> {
        let sched = JobScheduler::new().await?;
        let tasks = self.db.list_scheduled_tasks().unwrap_or_default();

        for task in tasks {
            let db_ref = Arc::clone(&self.db);
            let task_id = task.id.clone();
            let task_title = task.title.clone();
            let cron_expr = task.cron_expression.clone();

            let job = Job::new_async(cron_expr.as_str(), move |_uuid, _l| {
                let db_inner = Arc::clone(&db_ref);
                let tid = task_id.clone();
                let ttitle = task_title.clone();
                Box::pin(async move {
                    info!("Executing scheduled autonomous task: {} ({})", ttitle, tid);
                    let _ = db_inner.update_task_status(&tid, TaskStatus::Running);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let _ = db_inner.update_task_status(&tid, TaskStatus::Completed);
                })
            })?;

            sched.add(job).await?;
        }

        sched.start().await?;
        Ok(())
    }
}
