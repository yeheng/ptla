# 桌面应用程序系统设计文档

## 目录

1. [系统概述](modules/SYSTEM_OVERVIEW.md)
2. [技术栈说明](modules/TECH_STACK.md)
3. [系统架构设计](modules/ARCHITECTURE.md)
4. [数据流设计](modules/DATA_FLOW.md)
5. [前端模块设计](modules/FRONTEND_MODULES.md)
6. [后端模块设计](modules/BACKEND_MODULES.md)
7. [数据库设计](modules/DATABASE_DESIGN.md)
   - [用户表设计](modules/database/USER_TABLES.md)
   - [会话表设计](modules/database/SESSION_TABLES.md)
   - [任务表设计](modules/database/TASK_TABLES.md)
   - [任务清单表设计](modules/database/TASK_LIST_TABLES.md)
   - [任务分类表设计](modules/database/TASK_CATEGORY_TABLES.md)
   - [通知设置表设计](modules/database/NOTIFICATION_TABLES.md)
   - [通知模板表设计](modules/database/NOTIFICATION_TEMPLATE_TABLES.md)
   - [通知规则表设计](modules/database/NOTIFICATION_RULE_TABLES.md)
   - [系统配置表设计](modules/database/SYSTEM_CONFIG_TABLES.md)
   - [日志表设计](modules/database/LOG_TABLES.md)
   - [定时任务表设计](modules/database/SCHEDULED_JOB_TABLES.md)

## 文档说明

本文档已按模块拆分为多个独立的markdown文件，以提供更好的阅读和维护体验。请点击上方链接查看详细设计内容。

每个模块文档都包含该模块的详细设计信息，并提供返回主文档的链接，方便导航。如需修改或更新某个模块的设计，请直接编辑对应的模块文档。

## 项目结构

```
project/
├── src-tauri/           # Tauri后端代码
│   ├── src/            # Rust源代码
│   ├── Cargo.toml      # Rust依赖配置
│   └── tauri.conf.json # Tauri配置
├── src/                # Vue前端代码
│   ├── components/     # Vue组件
│   ├── views/         # 页面视图
│   ├── stores/        # 状态管理
│   └── assets/        # 静态资源
├── public/            # 公共资源
├── package.json       # 前端依赖配置
└── vite.config.ts    # Vite配置
```

## 开发规范

1. **代码规范**
   - Rust: rustfmt
   - TypeScript: ESLint + Prettier
   - Vue: Vue Style Guide

2. **Git工作流**
   - 分支管理策略
   - 提交信息规范
   - 代码审查流程

3. **文档规范**
   - API文档
   - 组件文档
   - 部署文档

## 1. 系统概述

本系统是一个基于现代技术栈的桌面应用程序，采用前后端分离架构，使用Tauri作为桌面应用框架，Vue.js作为前端框架，ShadcnUI作为UI组件库，Actix-web作为后端Web框架，SQLx作为数据库查询构建器，SQLite作为数据库。

## 2. 技术栈

### 2.1 前端技术栈

- **Tauri**: 用于构建跨平台桌面应用程序的框架
- **Vue 3**: 前端框架，使用Composition API
- **TypeScript**: 类型安全的JavaScript超集
- **ShadcnUI**: 基于Tailwind CSS的UI组件库
- **Vite**: 前端构建工具

### 2.2 后端技术栈

- **Rust**: 系统编程语言
- **Actix-web**: 高性能Web框架
- **SQLx**: 异步SQL查询构建器
- **SQLite**: 轻量级关系型数据库

## 3. 系统架构

### 3.1 整体架构

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│                 │     │                 │     │                 │
│   Vue Frontend  │◄───►│  Tauri Bridge  │◄───►│  Rust Backend   │
│                 │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                         │
                                                         ▼
                                                 ┌─────────────────┐
                                                 │                 │
                                                 │    SQLite DB    │
                                                 │                 │
                                                 └─────────────────┘
```

### 3.2 组件职责

1. **前端层 (Vue)**
   - 用户界面渲染
   - 状态管理
   - 用户交互处理
   - 与Tauri桥接层通信

2. **Tauri桥接层**
   - 提供系统API访问
   - 处理前端与后端的通信
   - 管理窗口和系统资源

3. **后端层 (Actix-web)**
   - 业务逻辑处理
   - 数据持久化
   - API端点提供
   - 数据验证和安全性

4. **数据层 (SQLx + SQLite)**
   - 数据存储
   - 数据查询和操作
   - 数据迁移管理

## 4. 数据流

1. 用户操作 → Vue组件
2. Vue组件 → Tauri命令
3. Tauri命令 → Actix-web API
4. Actix-web → SQLx查询
5. SQLite数据库操作
6. 响应数据反向传递

## 5. 关键功能模块

### 5.1 前端模块

- 用户认证界面
- 主应用界面
- 设置界面
- 错误处理界面
- 加载状态管理

### 5.2 后端模块

- 用户认证服务
- 数据管理服务
- 文件系统服务
- 系统配置服务
- 日志服务

### 5.3 数据库设计

#### 5.3.1 用户表 (users)

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

#### 5.3.2 应用数据表 (app_data)

```sql
CREATE TABLE app_data (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER NOT NULL,
    data_type       TEXT NOT NULL,
    title           TEXT NOT NULL,
    content         TEXT NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags            TEXT, -- JSON格式存储标签
    metadata        TEXT, -- JSON格式存储元数据
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_app_data_user_id ON app_data(user_id);
CREATE INDEX idx_app_data_data_type ON app_data(data_type);
CREATE INDEX idx_app_data_created_at ON app_data(created_at);
```

#### 5.3.3 系统配置表 (system_configs)

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

#### 5.3.4 日志表 (logs)

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

#### 5.3.5 用户会话表 (sessions)

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

#### 5.3.6 任务表 (tasks)

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

#### 5.3.7 任务清单表 (task_lists)

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

#### 5.3.8 任务与清单关联表 (task_list_items)

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

#### 5.3.9 任务分类表 (task_categories)

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

#### 5.3.10 任务分类关联表 (task_category_items)

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

#### 5.3.11 通知设置表 (notification_settings)

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

#### 5.3.12 通知模板表 (notification_templates)

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

#### 5.3.13 通知记录表 (notifications)

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

#### 5.3.14 通知触发规则表 (notification_rules)

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

#### 5.3.15 定时任务表 (scheduled_jobs)

```sql
CREATE TABLE scheduled_jobs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    job_type        TEXT NOT NULL, -- notification_check, task_reminder, email_digest, etc.
    status          TEXT NOT NULL DEFAULT 'pending', -- pending, running, completed, failed, cancelled
    schedule_type   TEXT NOT NULL, -- once, recurring
    cron_expression TEXT, -- 用于recurring类型的定时任务
    scheduled_at    TIMESTAMP NOT NULL, -- 用于one-time类型的定时任务
    last_run_at     TIMESTAMP,
    next_run_at     TIMESTAMP,
    retry_count     INTEGER NOT NULL DEFAULT 0,
    max_retries     INTEGER NOT NULL DEFAULT 3,
    parameters      TEXT, -- JSON格式存储任务参数
    result          TEXT, -- JSON格式存储执行结果
    error_message   TEXT,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_scheduled_jobs_status ON scheduled_jobs(status);
CREATE INDEX idx_scheduled_jobs_job_type ON scheduled_jobs(job_type);
CREATE INDEX idx_scheduled_jobs_next_run_at ON scheduled_jobs(next_run_at);
```

#### 5.3.16 定时任务执行记录表 (job_execution_logs)

```sql
CREATE TABLE job_execution_logs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id          INTEGER NOT NULL,
    execution_start TIMESTAMP NOT NULL,
    execution_end   TIMESTAMP,
    status          TEXT NOT NULL, -- started, completed, failed
    affected_items  INTEGER, -- 受影响的项目数量
    error_message   TEXT,
    performance_metrics TEXT, -- JSON格式存储性能指标
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (job_id) REFERENCES scheduled_jobs(id) ON DELETE CASCADE
);

CREATE INDEX idx_job_execution_logs_job_id ON job_execution_logs(job_id);
CREATE INDEX idx_job_execution_logs_execution_start ON job_execution_logs(execution_start);
CREATE INDEX idx_job_execution_logs_status ON job_execution_logs(status);
```

#### 5.3.17 系统任务配置表 (job_configurations)

```sql
CREATE TABLE job_configurations (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    job_type        TEXT NOT NULL UNIQUE,
    is_enabled      BOOLEAN NOT NULL DEFAULT TRUE,
    default_schedule TEXT NOT NULL, -- 默认调度配置（cron表达式）
    retry_strategy  TEXT NOT NULL DEFAULT 'exponential', -- none, linear, exponential
    max_retries     INTEGER NOT NULL DEFAULT 3,
    timeout_seconds INTEGER NOT NULL DEFAULT 300,
    concurrency     INTEGER NOT NULL DEFAULT 1, -- 允许同时运行的实例数
    parameters      TEXT, -- JSON格式存储默认参数
    description     TEXT,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_job_configurations_job_type ON job_configurations(job_type);
```

#### 5.3.18 表关系图

```
users
  ↑
  ├──── app_data
  ├──── sessions
  ├──── logs
  ├──── tasks ─────────┐
  ├──── task_lists     │
  ├──── task_categories│
  ├──── notification_settings
  ├──── notification_rules
  └──── notifications
                       │
task_list_items ───────┤
task_category_items ───┘

system_configs (独立表)
notification_templates (独立表)
scheduled_jobs ─── job_execution_logs
job_configurations (独立表)
```

#### 5.3.19 数据库设计说明

1. **字段类型选择**
   - 使用 INTEGER 作为主键类型（SQLite 中会被优化）
   - 使用 TEXT 存储可变长度字符串
   - 使用 TIMESTAMP 存储时间信息
   - 使用 BOOLEAN 存储布尔值
   - 复杂数据结构使用 JSON 格式存储在 TEXT 字段中

2. **索引策略**
   - 为所有外键创建索引
   - 为经常查询的字段创建索引
   - 为唯一约束字段创建唯一索引
   - 为时间范围查询字段创建索引

3. **安全考虑**
   - 密码只存储哈希值
   - 敏感配置支持加密存储
   - 使用外键约束保证数据完整性
   - 支持软删除机制

4. **性能优化**
   - 合理使用索引
   - 适当的字段类型选择
   - JSON 字段用于灵活存储
   - 分表策略预留（app_data表）

5. **可扩展性**
   - 预留元数据字段
   - 模块化的表结构设计
   - 支持未来功能扩展

#### 5.3.20 插件表 (plugins)

```sql
CREATE TABLE plugins (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT NOT NULL UNIQUE,
    version         TEXT NOT NULL,
    description     TEXT,
    author          TEXT,
    homepage_url    TEXT,
    repository_url  TEXT,
    license         TEXT,
    language        TEXT NOT NULL, -- 'lua' or 'typescript'
    main_file       TEXT NOT NULL, -- 入口文件路径
    config          TEXT, -- JSON格式存储插件配置
    permissions     TEXT NOT NULL, -- JSON格式存储权限列表
    dependencies    TEXT, -- JSON格式存储依赖信息
    enabled         BOOLEAN NOT NULL DEFAULT FALSE,
    installed_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_run_at     TIMESTAMP,
    status          TEXT NOT NULL DEFAULT 'installed', -- installed, active, error, disabled
    error_message   TEXT
);

CREATE INDEX idx_plugins_name ON plugins(name);
CREATE INDEX idx_plugins_status ON plugins(status);
```

#### 5.3.21 插件资源使用表 (plugin_resources)

```sql
CREATE TABLE plugin_resources (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id       INTEGER NOT NULL,
    cpu_usage       REAL, -- CPU使用率
    memory_usage    INTEGER, -- 内存使用量（字节）
    disk_usage      INTEGER, -- 磁盘使用量（字节）
    api_calls       INTEGER, -- API调用次数
    last_updated    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE
);

CREATE INDEX idx_plugin_resources_plugin_id ON plugin_resources(plugin_id);
```

#### 5.3.22 插件事件日志表 (plugin_events)

```sql
CREATE TABLE plugin_events (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id       INTEGER NOT NULL,
    event_type      TEXT NOT NULL, -- install, uninstall, enable, disable, error, etc.
    message         TEXT NOT NULL,
    details         TEXT, -- JSON格式存储详细信息
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE
);

CREATE INDEX idx_plugin_events_plugin_id ON plugin_events(plugin_id);
CREATE INDEX idx_plugin_events_created_at ON plugin_events(created_at);
```

## 6. 安全性考虑

1. **数据安全**
   - SQL注入防护
   - 参数化查询
   - 数据加密

2. **通信安全**
   - HTTPS/TLS
   - 请求验证
   - 跨域控制

3. **系统安全**
   - 文件系统访问控制
   - 系统API权限管理
   - 用户认证和授权

## 7. 性能优化

1. **前端优化**
   - 组件懒加载
   - 状态管理优化
   - 资源压缩

2. **后端优化**
   - 异步处理
   - 连接池管理
   - 缓存策略

3. **数据库优化**
   - 索引优化
   - 查询优化
   - 连接管理

## 8. 开发环境设置

### 8.1 前端开发环境

```bash
# 安装依赖
npm install

# 开发服务器
npm run dev

# 构建
npm run build
```

### 8.2 后端开发环境

```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装依赖
cargo build

# 运行开发服务器
cargo run
```

## 9. 部署策略

1. **开发环境**
   - 本地开发服务器
   - 热重载支持
   - 调试工具集成

2. **生产环境**
   - 应用打包
   - 自动更新支持
   - 错误报告系统

## 10. 监控和日志

1. **前端监控**
   - 性能监控
   - 错误追踪
   - 用户行为分析

2. **后端监控**
   - 系统资源监控
   - API性能监控
   - 错误日志记录

## 11. 未来扩展性

1. **功能扩展**
   - 插件系统
   - 主题定制
   - 多语言支持

2. **技术扩展**
   - 云同步支持
   - 多设备支持
   - 离线功能增强

## 12. 项目结构

```
project/
├── src-tauri/           # Tauri后端代码
│   ├── src/            # Rust源代码
│   ├── Cargo.toml      # Rust依赖配置
│   └── tauri.conf.json # Tauri配置
├── src/                # Vue前端代码
│   ├── components/     # Vue组件
│   ├── views/         # 页面视图
│   ├── stores/        # 状态管理
│   └── assets/        # 静态资源
├── public/            # 公共资源
├── package.json       # 前端依赖配置
└── vite.config.ts    # Vite配置
```

## 13. 开发规范

1. **代码规范**
   - Rust: rustfmt
   - TypeScript: ESLint + Prettier
   - Vue: Vue Style Guide

2. **Git工作流**
   - 分支管理策略
   - 提交信息规范
   - 代码审查流程

3. **文档规范**
   - API文档
   - 组件文档
   - 部署文档

## 14. 定时任务与通知系统实现

### 14.1 定时任务系统

#### 14.1.1 架构设计

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│                 │     │                 │     │                 │
│  Job Scheduler  │────►│   Job Queue    │────►│  Job Executor   │
│                 │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
        │                                               │
        │                                               │
        ▼                                               ▼
┌─────────────────┐                           ┌─────────────────┐
│                 │                           │                 │
│  Job Registry   │                           │   Job Logger    │
│                 │                           │                 │
└─────────────────┘                           └─────────────────┘
```

#### 14.1.2 核心组件

1. **Job Scheduler（任务调度器）**

   ```rust
   pub struct JobScheduler {
       registry: Arc<JobRegistry>,
       queue: Arc<JobQueue>,
       executor: Arc<JobExecutor>,
   }
   
   impl JobScheduler {
       // 初始化调度器
       pub async fn new() -> Self { ... }
       
       // 添加定时任务
       pub async fn schedule_job(&self, job: Job) -> Result<JobId> { ... }
       
       // 启动调度器
       pub async fn start(&self) -> Result<()> { ... }
       
       // 停止调度器
       pub async fn stop(&self) -> Result<()> { ... }
   }
   ```

2. **Job Registry（任务注册表）**

   ```rust
   pub struct JobRegistry {
       jobs: RwLock<HashMap<JobType, Box<dyn JobHandler>>>,
   }
   
   impl JobRegistry {
       // 注册任务处理器
       pub fn register<H: JobHandler>(&self, job_type: JobType, handler: H) { ... }
       
       // 获取任务处理器
       pub fn get_handler(&self, job_type: &JobType) -> Option<Box<dyn JobHandler>> { ... }
   }
   ```

3. **Job Queue（任务队列）**

   ```rust
   pub struct JobQueue {
       pending: Queue<Job>,
       running: HashMap<JobId, JobHandle>,
   }
   
   impl JobQueue {
       // 添加任务到队列
       pub async fn push(&self, job: Job) -> Result<JobId> { ... }
       
       // 获取下一个待执行的任务
       pub async fn next(&self) -> Option<Job> { ... }
       
       // 更新任务状态
       pub async fn update_status(&self, job_id: JobId, status: JobStatus) { ... }
   }
   ```