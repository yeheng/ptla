# 通知模板表设计

## 通知模板表 (notification_templates)

```sql
CREATE TABLE notification_templates (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    type            TEXT NOT NULL, -- task_due, task_completed, task_assigned, etc.
    title_template  TEXT NOT NULL,
    body_template   TEXT NOT NULL,
    platform        TEXT NOT NULL, -- desktop, email
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_notification_templates_type_platform ON notification_templates(type, platform);
```

## 字段说明

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| type | TEXT | 通知类型 |
| title_template | TEXT | 标题模板 |
| body_template | TEXT | 内容模板 |
| platform | TEXT | 通知平台 |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |

## 模板变量说明

通知模板支持以下变量：

### 通用变量

- `{{user_name}}` - 用户名称
- `{{current_time}}` - 当前时间
- `{{app_name}}` - 应用名称

### 任务相关变量

- `{{task_title}}` - 任务标题
- `{{task_description}}` - 任务描述
- `{{task_due_date}}` - 任务截止日期
- `{{task_status}}` - 任务状态
- `{{task_priority}}` - 任务优先级

### 示例模板

```sql
-- 任务到期提醒
INSERT INTO notification_templates (type, title_template, body_template, platform) VALUES
('task_due', 
 '任务即将到期: {{task_title}}',
 '您的任务"{{task_title}}"将在{{task_due_date}}到期，请及时处理。\n\n任务描述：{{task_description}}',
 'desktop');

-- 任务完成通知
INSERT INTO notification_templates (type, title_template, body_template, platform) VALUES
('task_completed',
 '任务已完成: {{task_title}}',
 '恭喜！您已完成任务"{{task_title}}"。\n\n完成时间：{{current_time}}',
 'email');
```

[返回数据库设计](../DATABASE_DESIGN.md)
