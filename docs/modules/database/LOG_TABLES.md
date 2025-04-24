# 日志表设计

## 日志表 (logs)

```sql
CREATE TABLE logs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER,
    log_level       TEXT NOT NULL,
    event_type      TEXT NOT NULL,
    message         TEXT NOT NULL,
    details         TEXT, -- JSON格式存储详细信息
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    source          TEXT NOT NULL, -- 日志来源（前端/后端/系统）
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX idx_logs_user_id ON logs(user_id);
CREATE INDEX idx_logs_created_at ON logs(created_at);
CREATE INDEX idx_logs_log_level ON logs(log_level);
CREATE INDEX idx_logs_event_type ON logs(event_type);
```

## 字段说明

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| user_id | INTEGER | 用户ID，外键 |
| log_level | TEXT | 日志级别 |
| event_type | TEXT | 事件类型 |
| message | TEXT | 日志消息 |
| details | TEXT | 详细信息（JSON） |
| created_at | TIMESTAMP | 创建时间 |
| source | TEXT | 日志来源 |

## 日志级别说明

- DEBUG: 调试信息
- INFO: 一般信息
- WARNING: 警告信息
- ERROR: 错误信息
- CRITICAL: 严重错误

## 事件类型示例

- USER_LOGIN: 用户登录
- USER_LOGOUT: 用户登出
- TASK_CREATE: 创建任务
- TASK_UPDATE: 更新任务
- TASK_DELETE: 删除任务
- SYSTEM_ERROR: 系统错误
- API_ERROR: API错误

## 日志示例

```sql
-- 用户登录日志
INSERT INTO logs (user_id, log_level, event_type, message, source, details) VALUES
(1, 'INFO', 'USER_LOGIN', '用户登录成功', 'backend', 
 '{"ip": "192.168.1.1", "device": "Chrome/Windows", "login_method": "password"}');

-- 系统错误日志
INSERT INTO logs (log_level, event_type, message, source, details) VALUES
('ERROR', 'SYSTEM_ERROR', '数据库连接失败', 'system',
 '{"error_code": "DB_001", "connection_string": "localhost:5432", "retry_count": 3}');

-- 任务操作日志
INSERT INTO logs (user_id, log_level, event_type, message, source, details) VALUES
(1, 'INFO', 'TASK_CREATE', '创建新任务', 'backend',
 '{"task_id": 123, "task_title": "完成项目报告", "due_date": "2024-03-20"}');
```

[返回数据库设计](../DATABASE_DESIGN.md)
