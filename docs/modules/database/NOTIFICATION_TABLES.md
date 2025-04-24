# 通知设置表设计

## 通知设置表 (notification_settings)

```sql
CREATE TABLE notification_settings (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    desktop_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    email_enabled   BOOLEAN NOT NULL DEFAULT TRUE,
    email_frequency TEXT NOT NULL DEFAULT 'instant', -- instant, daily, weekly
    quiet_hours_start TIME,
    quiet_hours_end TIME,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_notification_settings_user_id ON notification_settings(user_id);
```

## 通知记录表 (notifications)

```sql
CREATE TABLE notifications (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    task_id         INTEGER,
    type            TEXT NOT NULL, -- task_due, task_completed, task_assigned, etc.
    title           TEXT NOT NULL,
    content         TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'pending', -- pending, sent, failed, read
    platform        TEXT NOT NULL, -- desktop, email
    scheduled_for   TIMESTAMP NOT NULL,
    sent_at         TIMESTAMP,
    read_at         TIMESTAMP,
    error           TEXT,
    metadata        TEXT, -- JSON格式存储额外数据
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE SET NULL
);

CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_task_id ON notifications(task_id);
CREATE INDEX idx_notifications_status ON notifications(status);
CREATE INDEX idx_notifications_scheduled_for ON notifications(scheduled_for);
CREATE INDEX idx_notifications_type ON notifications(type);
```

## 字段说明

### notification_settings 表

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| user_id | INTEGER | 用户ID，外键 |
| desktop_enabled | BOOLEAN | 是否启用桌面通知 |
| email_enabled | BOOLEAN | 是否启用邮件通知 |
| email_frequency | TEXT | 邮件通知频率 |
| quiet_hours_start | TIME | 免打扰开始时间 |
| quiet_hours_end | TIME | 免打扰结束时间 |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |

### notifications 表

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| user_id | INTEGER | 用户ID，外键 |
| task_id | INTEGER | 任务ID，外键 |
| type | TEXT | 通知类型 |
| title | TEXT | 通知标题 |
| content | TEXT | 通知内容 |
| status | TEXT | 通知状态 |
| platform | TEXT | 通知平台 |
| scheduled_for | TIMESTAMP | 计划发送时间 |
| sent_at | TIMESTAMP | 实际发送时间 |
| read_at | TIMESTAMP | 阅读时间 |
| error | TEXT | 错误信息 |
| metadata | TEXT | 元数据（JSON） |
| created_at | TIMESTAMP | 创建时间 |

[返回数据库设计](../DATABASE_DESIGN.md)
