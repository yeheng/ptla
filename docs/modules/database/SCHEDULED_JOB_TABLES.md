# 定时任务表设计

## 定时任务表 (scheduled_jobs)

```sql
CREATE TABLE scheduled_jobs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    job_type        TEXT NOT NULL, -- notification_check, task_reminder, email_digest, etc.
    status          TEXT NOT NULL DEFAULT 'pending', -- pending, running, completed, failed, cancelled
    schedule_type   TEXT NOT NULL, -- once, recurring
    cron_expression TEXT, -- 用于recurring类型的定时任务
    scheduled_at    TIMESTAMP NOT NULL, -- 用于one-time类型的定时任务
    last_run_at     TIMESTAMP,
    next_run_at     TIMESTAMP,
    retry_count     INTEGER NOT NULL DEFAULT 0,
    max_retries     INTEGER NOT NULL DEFAULT 3,
    error_message   TEXT,
    metadata        TEXT, -- JSON格式存储任务相关数据
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_active       BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX idx_scheduled_jobs_status ON scheduled_jobs(status);
CREATE INDEX idx_scheduled_jobs_next_run_at ON scheduled_jobs(next_run_at);
CREATE INDEX idx_scheduled_jobs_job_type ON scheduled_jobs(job_type);
```

## 字段说明

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| job_type | TEXT | 任务类型 |
| status | TEXT | 任务状态 |
| schedule_type | TEXT | 调度类型 |
| cron_expression | TEXT | Cron表达式 |
| scheduled_at | TIMESTAMP | 计划执行时间 |
| last_run_at | TIMESTAMP | 上次执行时间 |
| next_run_at | TIMESTAMP | 下次执行时间 |
| retry_count | INTEGER | 重试次数 |
| max_retries | INTEGER | 最大重试次数 |
| error_message | TEXT | 错误信息 |
| metadata | TEXT | 元数据（JSON） |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |
| is_active | BOOLEAN | 是否激活 |

## 任务类型示例

- notification_check: 通知检查
- task_reminder: 任务提醒
- email_digest: 邮件摘要
- data_cleanup: 数据清理
- backup: 数据备份

## 示例任务

```sql
-- 每日邮件摘要任务
INSERT INTO scheduled_jobs (
    job_type, 
    schedule_type, 
    cron_expression, 
    metadata
) VALUES (
    'email_digest',
    'recurring',
    '0 0 * * *', -- 每天午夜执行
    '{"template": "daily_digest", "user_filter": "active_users"}'
);

-- 定时数据备份任务
INSERT INTO scheduled_jobs (
    job_type,
    schedule_type,
    cron_expression,
    metadata
) VALUES (
    'backup',
    'recurring',
    '0 0 * * 0', -- 每周日午夜执行
    '{"backup_type": "full", "retention_days": 30}'
);

-- 一次性任务提醒
INSERT INTO scheduled_jobs (
    job_type,
    schedule_type,
    scheduled_at,
    metadata
) VALUES (
    'task_reminder',
    'once',
    '2024-03-20 10:00:00',
    '{"task_id": 123, "user_id": 1, "reminder_type": "due_soon"}'
);
```

[返回数据库设计](../DATABASE_DESIGN.md)
