# 用户表设计

## 用户表 (users)

```sql
CREATE TABLE users (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    username        TEXT NOT NULL UNIQUE,
    password_hash   TEXT NOT NULL,
    email           TEXT UNIQUE,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login      TIMESTAMP,
    status          TEXT NOT NULL DEFAULT 'active',
    role            TEXT NOT NULL DEFAULT 'user',
    settings        TEXT -- JSON格式存储用户设置
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
```

## 字段说明

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| username | TEXT | 用户名，唯一 |
| password_hash | TEXT | 密码哈希值 |
| email | TEXT | 邮箱，唯一 |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |
| last_login | TIMESTAMP | 最后登录时间 |
| status | TEXT | 用户状态 |
| role | TEXT | 用户角色 |
| settings | TEXT | 用户设置（JSON） |

[返回数据库设计](../DATABASE_DESIGN.md)
