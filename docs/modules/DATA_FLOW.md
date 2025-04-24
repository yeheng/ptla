# 数据流设计

## 1. 数据流程

1. 用户操作 → Vue组件
2. Vue组件 → Tauri命令
3. Tauri命令 → Actix-web API
4. Actix-web → SQLx查询
5. SQLite数据库操作
6. 响应数据反向传递

## 2. 数据流程图

```
用户操作 → Vue组件 → Tauri命令 → Actix-web API → SQLx查询 → SQLite
                                                            ↓
响应数据 ← Vue组件 ← Tauri命令 ← Actix-web API ← SQLx查询 ← SQLite
```

[返回系统设计文档](../../README.md)
