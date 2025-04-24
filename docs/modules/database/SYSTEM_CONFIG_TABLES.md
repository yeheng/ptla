# 系统配置表设计

## 系统配置表 (system_configs)

```sql
CREATE TABLE system_configs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    config_key      TEXT NOT NULL UNIQUE,
    config_value    TEXT NOT NULL,
    description     TEXT,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_encrypted    BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE UNIQUE INDEX idx_system_configs_key ON system_configs(config_key);
```

## 字段说明

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 主键，自增 |
| config_key | TEXT | 配置键名，唯一 |
| config_value | TEXT | 配置值 |
| description | TEXT | 配置说明 |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |
| is_encrypted | BOOLEAN | 是否加密存储 |

## 系统配置示例

```sql
-- 基本系统配置
INSERT INTO system_configs (config_key, config_value, description) VALUES
('app_name', 'Todo App', '应用名称'),
('app_version', '1.0.0', '应用版本'),
('max_tasks_per_list', '1000', '每个列表最大任务数'),
('max_lists_per_user', '50', '每个用户最大列表数');

-- 邮件服务配置
INSERT INTO system_configs (config_key, config_value, description, is_encrypted) VALUES
('smtp_host', 'smtp.example.com', 'SMTP服务器地址', FALSE),
('smtp_port', '587', 'SMTP服务器端口', FALSE),
('smtp_username', 'noreply@example.com', 'SMTP用户名', FALSE),
('smtp_password', 'encrypted_password', 'SMTP密码', TRUE);

-- 通知配置
INSERT INTO system_configs (config_key, config_value, description) VALUES
('notification_check_interval', '300', '通知检查间隔（秒）'),
('notification_batch_size', '100', '每批处理的通知数量');
```

[返回数据库设计](../DATABASE_DESIGN.md)
