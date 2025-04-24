use std::path::Path;
use anyhow::Result;
use mlua::Lua;
use async_trait::async_trait;

#[async_trait]
pub trait PluginRuntime: Send + Sync {
    /// 获取插件ID
    fn id(&self) -> &str;
    
    /// 获取插件版本
    fn version(&self) -> &str;
    
    /// 获取插件描述
    fn description(&self) -> &str;
    
    /// 获取插件作者
    fn author(&self) -> &str;
    
    /// 初始化插件
    async fn init(&mut self) -> Result<()>;
    
    /// 重新加载插件
    async fn reload(&mut self) -> Result<()>;
    
    /// 卸载插件
    async fn unload(&mut self) -> Result<()>;
    
    /// 执行插件中的函数
    async fn call_function(&self, name: &str, args: Vec<mlua::Value>) -> Result<mlua::Value>;
}

pub struct LuaPluginRuntime {
    id: String,
    version: String,
    description: String,
    author: String,
    lua: Lua,
    script_path: std::path::PathBuf,
}

impl LuaPluginRuntime {
    pub fn new(script_path: impl AsRef<Path>) -> Result<Self> {
        let lua = Lua::new();
        let script_path = script_path.as_ref().to_path_buf();
        
        // 创建一个基本的插件实例
        let runtime = Self {
            id: script_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
            version: "0.1.0".to_string(),
            description: "A Lua plugin".to_string(),
            author: "Unknown".to_string(),
            lua,
            script_path,
        };
        
        Ok(runtime)
    }
    
    fn load_script(&self) -> Result<()> {
        let script = std::fs::read_to_string(&self.script_path)?;
        self.lua.load(&script).exec()?;
        Ok(())
    }
    
    fn update_metadata(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        
        if let Ok(version) = globals.get::<_, String>("PLUGIN_VERSION") {
            self.version = version;
        }
        
        if let Ok(description) = globals.get::<_, String>("PLUGIN_DESCRIPTION") {
            self.description = description;
        }
        
        if let Ok(author) = globals.get::<_, String>("PLUGIN_AUTHOR") {
            self.author = author;
        }
        
        Ok(())
    }
}

#[async_trait]
impl PluginRuntime for LuaPluginRuntime {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn author(&self) -> &str {
        &self.author
    }
    
    async fn init(&mut self) -> Result<()> {
        self.load_script()?;
        self.update_metadata()?;
        Ok(())
    }
    
    async fn reload(&mut self) -> Result<()> {
        self.lua = Lua::new();
        self.init().await
    }
    
    async fn unload(&mut self) -> Result<()> {
        // Lua 会自动清理资源，这里暂时不需要特殊处理
        Ok(())
    }
    
    async fn call_function(&self, name: &str, args: Vec<mlua::Value>) -> Result<mlua::Value> {
        let func: mlua::Function = self.lua.globals().get(name)?;
        Ok(func.call(args)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    #[tokio::test]
    async fn test_lua_plugin_runtime() -> Result<()> {
        // 创建一个临时的 Lua 脚本文件
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, r#"
            PLUGIN_VERSION = "1.0.0"
            PLUGIN_DESCRIPTION = "Test Plugin"
            PLUGIN_AUTHOR = "Test Author"
            
            function test_function(x, y)
                return x + y
            end
        "#)?;
        
        // 创建运行时实例
        let mut runtime = LuaPluginRuntime::new(temp_file.path())?;
        
        // 初始化插件
        runtime.init().await?;
        
        // 验证元数据
        assert_eq!(runtime.version(), "1.0.0");
        assert_eq!(runtime.description(), "Test Plugin");
        assert_eq!(runtime.author(), "Test Author");
        
        // 测试函数调用
        let result = runtime.call_function(
            "test_function",
            vec![mlua::Value::Number(5.0), mlua::Value::Number(3.0)]
        ).await?;
        
        assert_eq!(result, mlua::Value::Number(8.0));
        
        Ok(())
    }
} 