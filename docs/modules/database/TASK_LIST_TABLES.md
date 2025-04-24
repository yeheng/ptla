# 任务清单表设计

## 任务清单表 (task_lists)

```sql
CREATE TABLE task_lists (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    title           TEXT NOT NULL,
    description     TEXT,
    color          TEXT, -- 用于UI显示的颜色
    icon           TEXT, -- 图标标识
    position        INTEGER NOT NULL DEFAULT 0, -- 用于列表排序
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_task_lists_user_id ON task_lists(user_id);
CREATE INDEX idx_task_lists_position ON task_lists(position);
```

## 任务与清单关联表 (task_list_items)

```sql
CREATE TABLE task_list_items (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id         INTEGER NOT NULL,
    list_id         INTEGER NOT NULL,
    position        INTEGER NOT NULL DEFAULT 0, -- 任务在特定列表中的排序位置
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (list_id) REFERENCES task_lists(id) ON DELETE CASCADE,
    UNIQUE(task_id, list_id) -- 确保任务在同一个列表中只出现一次
);

CREATE INDEX idx_task_list_items_task_id ON task_list_items(task_id);
CREATE INDEX idx_task_list_items_list_id ON task_list_items(list_id);
CREATE INDEX idx_task_list_items_position ON task_list_items(position);
```

## 字段说明

### task_lists 表

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| user_id | INTEGER | 用户ID，外键 |
| title | TEXT | 清单标题 |
| description | TEXT | 清单描述 |
| color | TEXT | UI显示颜色 |
| icon | TEXT | 图标标识 |
| position | INTEGER | 排序位置 |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |
| is_deleted | BOOLEAN | 是否删除 |

### task_list_items 表

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| task_id | INTEGER | 任务ID，外键 |
| list_id | INTEGER | 清单ID，外键 |
| position | INTEGER | 排序位置 |
| created_at | TIMESTAMP | 创建时间 |

[返回数据库设计](../DATABASE_DESIGN.md)
