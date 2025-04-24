# 通知规则表设计

## 通知触发规则表 (notification_rules)

```sql
CREATE TABLE notification_rules (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    event_type      TEXT NOT NULL, -- task_created, task_due, status_changed, etc.
    conditions      TEXT NOT NULL, -- JSON格式存储触发条件
    platforms       TEXT NOT NULL, -- JSON格式存储通知平台列表
    advance_notice  INTEGER, -- 提前通知时间（分钟）
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_notification_rules_user_id ON notification_rules(user_id);
CREATE INDEX idx_notification_rules_event_type ON notification_rules(event_type);
```

## 字段说明

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| user_id | INTEGER | 用户ID，外键 |
| event_type | TEXT | 事件类型 |
| conditions | TEXT | 触发条件（JSON） |
| platforms | TEXT | 通知平台列表（JSON） |
| advance_notice | INTEGER | 提前通知时间 |
| is_active | BOOLEAN | 是否激活 |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |

## 条件格式说明

conditions字段的JSON格式示例：

```json
{
    "operator": "AND",
    "conditions": [
        {
            "field": "priority",
            "operator": ">=",
            "value": 2
        },
        {
            "field": "due_date",
            "operator": "<=",
            "value": "24h"
        }
    ]
}
```

## 平台列表格式

platforms字段的JSON格式示例：

```json
{
    "platforms": ["desktop", "email"],
    "desktop": {
        "priority": "high"
    },
    "email": {
        "template": "urgent_task"
    }
}
```

[返回数据库设计](../DATABASE_DESIGN.md)
