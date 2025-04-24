# 会话表设计

## 用户会话表 (sessions)

```sql
CREATE TABLE sessions (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    token           TEXT NOT NULL UNIQUE,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at      TIMESTAMP NOT NULL,
    last_activity   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    device_info     TEXT, -- JSON格式存储设备信息
    is_valid        BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_sessions_token ON sessions(token);
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
```

## 字段说明

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| user_id | INTEGER | 用户ID，外键 |
| token | TEXT | 会话令牌，唯一 |
| created_at | TIMESTAMP | 创建时间 |
| expires_at | TIMESTAMP | 过期时间 |
| last_activity | TIMESTAMP | 最后活动时间 |
| device_info | TEXT | 设备信息（JSON） |
| is_valid | BOOLEAN | 会话是否有效 |

[返回数据库设计](../DATABASE_DESIGN.md)
