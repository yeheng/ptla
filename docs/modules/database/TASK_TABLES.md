# 任务表设计

## 任务表 (tasks)

```sql
CREATE TABLE tasks (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    title           TEXT NOT NULL,
    description     TEXT,
    status          TEXT NOT NULL DEFAULT 'pending', -- pending, in_progress, completed, cancelled
    priority        INTEGER NOT NULL DEFAULT 0, -- 0: 低优先级, 1: 中优先级, 2: 高优先级
    due_date        TIMESTAMP,
    completed_at    TIMESTAMP,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    parent_task_id  INTEGER, -- 用于子任务功能
    position        INTEGER NOT NULL DEFAULT 0, -- 用于任务排序
    tags            TEXT, -- JSON格式存储标签
    metadata        TEXT, -- JSON格式存储额外数据
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
CREATE INDEX idx_tasks_parent_task_id ON tasks(parent_task_id);
CREATE INDEX idx_tasks_position ON tasks(position);
```

## 字段说明

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| user_id | INTEGER | 用户ID，外键 |
| title | TEXT | 任务标题 |
| description | TEXT | 任务描述 |
| status | TEXT | 任务状态 |
| priority | INTEGER | 优先级 |
| due_date | TIMESTAMP | 截止日期 |
| completed_at | TIMESTAMP | 完成时间 |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |
| parent_task_id | INTEGER | 父任务ID |
| position | INTEGER | 排序位置 |
| tags | TEXT | 标签（JSON） |
| metadata | TEXT | 元数据（JSON） |
| is_deleted | BOOLEAN | 是否删除 |

[返回数据库设计](../DATABASE_DESIGN.md)
