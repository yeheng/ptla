# PLTA system design

## 目录

1. [系统概述](modules/SYSTEM_OVERVIEW.md)
2. [技术栈说明](modules/TECH_STACK.md)
3. [系统架构设计](modules/ARCHITECTURE.md)
4. [数据流设计](modules/DATA_FLOW.md)
5. [前端模块设计](modules/FRONTEND_MODULES.md)
6. [后端模块设计](modules/BACKEND_MODULES.md)
7. [插件系统设计](modules/plugin-system-design.md)
8. [数据库设计](modules/DATABASE_DESIGN.md)
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
│   │   ├── plugins/   # 插件系统实现
│   │   └── runtime/   # 插件运行时
│   ├── Cargo.toml      # Rust依赖配置
│   └── tauri.conf.json # Tauri配置
├── src/                # Vue前端代码
│   ├── components/     # Vue组件
│   ├── views/         # 页面视图
│   ├── stores/        # 状态管理
│   └── assets/        # 静态资源
├── plugins/           # 插件目录
│   ├── official/     # 官方插件
│   └── community/    # 社区插件
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