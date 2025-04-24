use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use notify::{Watcher, RecursiveMode, Event};
use anyhow::Result;

pub struct PluginWatcher {
    watcher: notify::RecommendedWatcher,
    watch_path: PathBuf,
    _tx: mpsc::Sender<PluginWatchEvent>,
}

#[derive(Debug, Clone)]
pub struct PluginWatchEvent {
    pub path: PathBuf,
    pub event_type: PluginWatchEventType,
}

#[derive(Debug, Clone)]
pub enum PluginWatchEventType {
    Created,
    Modified,
    Deleted,
}

impl PluginWatcher {
    pub fn new(
        watch_path: impl AsRef<Path>,
        tx: mpsc::Sender<PluginWatchEvent>
    ) -> Result<Self> {
        let watch_path = watch_path.as_ref().to_path_buf();
        let tx_clone = tx.clone();

        // 创建文件系统监控器
        let watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let event_type = match event.kind {
                    notify::EventKind::Create(_) => Some(PluginWatchEventType::Created),
                    notify::EventKind::Modify(_) => Some(PluginWatchEventType::Modified),
                    notify::EventKind::Remove(_) => Some(PluginWatchEventType::Deleted),
                    _ => None,
                };

                if let Some(event_type) = event_type {
                    for path in event.paths {
                        if path.extension().map_or(false, |ext| ext == "lua") {
                            let watch_event = PluginWatchEvent {
                                path,
                                event_type: event_type.clone(),
                            };
                            let _ = tx_clone.blocking_send(watch_event);
                        }
                    }
                }
            }
        })?;

        // 开始监控插件目录
        watcher.watch(&watch_path, RecursiveMode::Recursive)?;

        Ok(Self {
            watcher,
            watch_path,
            _tx: tx,
        })
    }

    pub fn watch_path(&self) -> &Path {
        &self.watch_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    use tokio::sync::mpsc;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_plugin_watcher() {
        let dir = tempdir().unwrap();
        let (tx, mut rx) = mpsc::channel(32);
        
        let _watcher = PluginWatcher::new(dir.path(), tx).unwrap();
        
        // 创建测试文件
        let test_file = dir.path().join("test.lua");
        fs::write(&test_file, "print('Hello')").unwrap();
        
        // 等待文件创建事件
        if let Some(event) = rx.recv().await {
            assert_eq!(event.path, test_file);
            assert!(matches!(event.event_type, PluginWatchEventType::Created));
        }
        
        // 修改文件
        sleep(Duration::from_millis(100)).await;
        fs::write(&test_file, "print('Updated')").unwrap();
        
        // 等待文件修改事件
        if let Some(event) = rx.recv().await {
            assert_eq!(event.path, test_file);
            assert!(matches!(event.event_type, PluginWatchEventType::Modified));
        }
        
        // 删除文件
        sleep(Duration::from_millis(100)).await;
        fs::remove_file(&test_file).unwrap();
        
        // 等待文件删除事件
        if let Some(event) = rx.recv().await {
            assert_eq!(event.path, test_file);
            assert!(matches!(event.event_type, PluginWatchEventType::Deleted));
        }
    }
} 