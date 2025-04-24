use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono;
use tokio::sync::mpsc;

pub mod api;
pub mod runtime;
pub mod watcher;
pub mod lua;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    pub license: Option<String>,
    pub language: PluginLanguage,
    pub main_file: String,
    pub permissions: Vec<String>,
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginLanguage {
    Lua,
    TypeScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginStatus {
    Installed,
    Active,
    Error,
    Disabled,
}

pub struct PluginManager {
    plugins: Arc<Mutex<HashMap<String, Box<dyn PluginRuntime>>>>,
    watcher: Option<watcher::PluginWatcher>,
    watcher_rx: Option<mpsc::Receiver<watcher::PluginWatchEvent>>,
}

#[derive(Clone)]
pub struct Plugin {
    metadata: PluginMetadata,
    status: PluginStatus,
    runtime: Option<Arc<dyn PluginRuntime>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMessage {
    pub source: String,
    pub target: Option<String>,
    pub message_type: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory: Option<usize>,      // 最大内存使用量（字节）
    pub max_cpu_time: Option<u64>,      // 最大 CPU 时间（毫秒）
    pub max_tasks: Option<usize>,       // 最大并发任务数
    pub max_api_calls: Option<usize>,   // API 调用频率限制
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub memory_used: usize,
    pub cpu_time: u64,
    pub active_tasks: usize,
    pub api_calls: usize,
}

pub trait PluginRuntime: Send + Sync {
    // 基本生命周期管理
    fn init(&self) -> Result<()>;
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
    fn unload(&self) -> Result<()>;
    
    // 插件间通信
    fn send_message(&self, message: PluginMessage) -> Result<()> {
        Ok(()) // 默认实现：不处理消息
    }
    
    fn receive_message(&self, message: PluginMessage) -> Result<()> {
        Ok(()) // 默认实现：不处理消息
    }
    
    // 资源管理
    fn get_resource_usage(&self) -> Result<ResourceUsage> {
        Ok(ResourceUsage {
            memory_used: 0,
            cpu_time: 0,
            active_tasks: 0,
            api_calls: 0,
        })
    }
    
    fn set_resource_limits(&self, limits: ResourceLimits) -> Result<()> {
        Ok(()) // 默认实现：不限制资源
    }
    
    // 热重载支持
    fn reload(&self) -> Result<()> {
        self.stop()?;
        self.init()?;
        self.start()
    }
    
    // API 注册
    fn register_api(&self, name: &str, api: Box<dyn PluginApi>) -> Result<()> {
        Ok(()) // 默认实现：不支持 API 注册
    }
}

pub trait PluginApi: Send + Sync {
    fn call(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value>;
    fn get_permissions(&self) -> Vec<String>;
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(Mutex::new(HashMap::new())),
            watcher: None,
            watcher_rx: None,
        }
    }

    pub async fn init_with_watch(&mut self, plugin_dir: impl AsRef<std::path::Path>) -> Result<()> {
        let (tx, rx) = mpsc::channel(32);
        let watcher = watcher::PluginWatcher::new(plugin_dir, tx)?;
        
        self.watcher = Some(watcher);
        self.watcher_rx = Some(rx);
        
        // 启动文件监控处理循环
        let plugins = Arc::clone(&self.plugins);
        tokio::spawn(async move {
            if let Some(mut rx) = self.watcher_rx.take() {
                while let Some(event) = rx.recv().await {
                    let mut plugins = plugins.lock().await;
                    match event.event_type {
                        watcher::PluginWatchEventType::Modified => {
                            if let Some(plugin_id) = event.path.file_stem() {
                                if let Some(plugin) = plugins.get_mut(plugin_id.to_str().unwrap()) {
                                    let _ = plugin.reload().await;
                                }
                            }
                        }
                        watcher::PluginWatchEventType::Created => {
                            // 处理新插件创建
                            if let Some(plugin_id) = event.path.file_stem() {
                                if !plugins.contains_key(plugin_id.to_str().unwrap()) {
                                    // TODO: 加载新插件
                                }
                            }
                        }
                        watcher::PluginWatchEventType::Deleted => {
                            // 处理插件删除
                            if let Some(plugin_id) = event.path.file_stem() {
                                plugins.remove(plugin_id.to_str().unwrap());
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    pub fn load_plugin(&self, name: &str) -> Result<()> {
        // TODO: 实现插件加载逻辑
        Ok(())
    }

    pub fn unload_plugin(&self, name: &str) -> Result<()> {
        // TODO: 实现插件卸载逻辑
        Ok(())
    }

    pub fn enable_plugin(&self, name: &str) -> Result<()> {
        // TODO: 实现插件启用逻辑
        Ok(())
    }

    pub fn disable_plugin(&self, name: &str) -> Result<()> {
        // TODO: 实现插件禁用逻辑
        Ok(())
    }

    pub fn get_plugin(&self, name: &str) -> Option<Box<dyn PluginRuntime>> {
        self.plugins.lock().unwrap().get(name).cloned()
    }

    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        self.plugins
            .lock()
            .unwrap()
            .values()
            .map(|p| p.metadata.clone())
            .collect()
    }
}

impl Plugin {
    pub fn new(metadata: PluginMetadata) -> Self {
        Self {
            metadata,
            status: PluginStatus::Installed,
            runtime: None,
        }
    }

    pub fn status(&self) -> PluginStatus {
        self.status.clone()
    }

    pub fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let plugin_dir = PathBuf::from("plugins");
        let manager = PluginManager::new();
        assert!(manager.plugins.lock().unwrap().is_empty());
    }

    #[test]
    fn test_plugin_metadata_serialization() {
        let metadata = PluginMetadata {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test plugin".to_string()),
            author: Some("Test Author".to_string()),
            homepage_url: None,
            repository_url: None,
            license: Some("MIT".to_string()),
            language: PluginLanguage::Lua,
            main_file: "main.lua".to_string(),
            permissions: vec!["fs.read".to_string()],
            dependencies: None,
        };

        let serialized = serde_json::to_string(&metadata).unwrap();
        let deserialized: PluginMetadata = serde_json::from_str(&serialized).unwrap();
        assert_eq!(metadata.name, deserialized.name);
    }
} 