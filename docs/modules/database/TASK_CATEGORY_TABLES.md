# 任务分类表设计

## 任务分类表 (task_categories)

```sql
CREATE TABLE task_categories (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT,
    color          TEXT, -- 用于UI显示的颜色
    icon           TEXT, -- 图标标识
    parent_id       INTEGER, -- 支持分类层级
    position        INTEGER NOT NULL DEFAULT 0, -- 用于分类排序
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES task_categories(id) ON DELETE CASCADE
);

CREATE INDEX idx_task_categories_user_id ON task_categories(user_id);
CREATE INDEX idx_task_categories_parent_id ON task_categories(parent_id);
CREATE INDEX idx_task_categories_position ON task_categories(position);
```

## 任务分类关联表 (task_category_items)

```sql
CREATE TABLE task_category_items (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id         INTEGER NOT NULL,
    category_id     INTEGER NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES task_categories(id) ON DELETE CASCADE,
    UNIQUE(task_id, category_id) -- 确保任务在同一个分类中只出现一次
);

CREATE INDEX idx_task_category_items_task_id ON task_category_items(task_id);
CREATE INDEX idx_task_category_items_category_id ON task_category_items(category_id);
```

## 字段说明

### task_categories 表

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| user_id | INTEGER | 用户ID，外键 |
| name | TEXT | 分类名称 |
| description | TEXT | 分类描述 |
| color | TEXT | UI显示颜色 |
| icon | TEXT | 图标标识 |
| parent_id | INTEGER | 父分类ID |
| position | INTEGER | 排序位置 |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |
| is_deleted | BOOLEAN | 是否删除 |

### task_category_items 表

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| task_id | INTEGER | 任务ID，外键 |
| category_id | INTEGER | 分类ID，外键 |
| created_at | TIMESTAMP | 创建时间 |

[返回数据库设计](../DATABASE_DESIGN.md)
