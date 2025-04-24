use std::path::Path;
use std::sync::Arc;
use std::collections::HashMap;
use mlua::{Lua, Result as LuaResult, Function, Table, Value as LuaValue};
use anyhow::{Result, Context};
use crate::plugin::{PluginRuntime, PluginMetadata, PluginMessage, ResourceLimits, ResourceUsage, PluginApi};

pub struct LuaRuntime {
    lua: Lua,
    metadata: PluginMetadata,
    script_path: String,
    apis: Arc<Mutex<HashMap<String, Box<dyn PluginApi>>>>,
    resource_limits: Arc<Mutex<ResourceLimits>>,
    message_handler: Option<Function<'static>>,
}

impl LuaRuntime {
    pub fn new(metadata: PluginMetadata, script_path: String) -> Result<Self> {
        let lua = Lua::new();
        Self::setup_sandbox(&lua)?;
        
        Ok(Self {
            lua,
            metadata,
            script_path,
            apis: Arc::new(Mutex::new(HashMap::new())),
            resource_limits: Arc::new(Mutex::new(ResourceLimits {
                max_memory: Some(50 * 1024 * 1024), // 默认 50MB
                max_cpu_time: Some(1000),           // 默认 1 秒
                max_tasks: Some(10),                // 默认最多 10 个并发任务
                max_api_calls: Some(1000),          // 默认每分钟 1000 次 API 调用
            })),
            message_handler: None,
        })
    }

    fn setup_sandbox(lua: &Lua) -> LuaResult<()> {
        // 创建一个受限制的环境
        let globals = lua.globals();
        
        // 创建一个新的环境表
        let env = lua.create_table()?;
        
        // 只允许访问安全的标准库函数
        let string = lua.create_table()?;
        string.set("len", globals.get::<_, Function>("string.len")?)?;
        string.set("sub", globals.get::<_, Function>("string.sub")?)?;
        string.set("upper", globals.get::<_, Function>("string.upper")?)?;
        string.set("lower", globals.get::<_, Function>("string.lower")?)?;
        env.set("string", string)?;
        
        let math = lua.create_table()?;
        math.set("abs", globals.get::<_, Function>("math.abs")?)?;
        math.set("ceil", globals.get::<_, Function>("math.ceil")?)?;
        math.set("floor", globals.get::<_, Function>("math.floor")?)?;
        math.set("max", globals.get::<_, Function>("math.max")?)?;
        math.set("min", globals.get::<_, Function>("math.min")?)?;
        env.set("math", math)?;
        
        // 添加 print 函数的安全版本
        let print = lua.create_function(|_, msg: String| {
            println!("[Lua Plugin]: {}", msg);
            Ok(())
        })?;
        env.set("print", print)?;
        
        // 设置为默认环境
        lua.set_named_registry_value("plugin_env", env)?;
        
        Ok(())
    }

    fn setup_api_bindings(&self) -> Result<()> {
        let env: Table = self.lua.named_registry_value("plugin_env")?;
        
        // 创建 API 访问表
        let api_table = self.lua.create_table()?;
        
        // 遍历所有注册的 API
        for (name, api) in self.apis.lock().unwrap().iter() {
            let api_clone = api.clone();
            let func = self.lua.create_function(move |_, (method, params): (String, LuaValue)| {
                let params_json = serde_json::to_value(params)?;
                let result = api_clone.call(&method, params_json)?;
                Ok(LuaValue::String(self.lua.create_string(&serde_json::to_string(&result)?)?))
            })?;
            api_table.set(name.clone(), func)?;
        }
        
        env.set("api", api_table)?;
        
        Ok(())
    }

    fn load_script(&self) -> Result<()> {
        let script = std::fs::read_to_string(&self.script_path)
            .context("Failed to read Lua script")?;
            
        // 获取沙箱环境
        let env: Table = self.lua.named_registry_value("plugin_env")?;
        
        // 在沙箱环境中加载并执行脚本
        self.lua.load(&script)
            .set_environment(env)?
            .exec()
            .context("Failed to execute Lua script")?;
            
        // 保存消息处理函数（如果存在）
        if let Ok(handler) = env.get::<_, Function>("on_message") {
            self.message_handler = Some(handler);
        }
            
        Ok(())
    }

    fn call_function(&self, name: &str) -> Result<()> {
        let env: Table = self.lua.named_registry_value("plugin_env")?;
        
        if let Ok(func) = env.get::<_, Function>(name) {
            func.call(()).context(format!("Failed to call Lua function '{}'", name))?;
        }
        
        Ok(())
    }
}

impl PluginRuntime for LuaRuntime {
    fn init(&self) -> Result<()> {
        self.setup_api_bindings()?;
        self.load_script()?;
        self.call_function("init")
    }

    fn start(&self) -> Result<()> {
        self.call_function("start")
    }

    fn stop(&self) -> Result<()> {
        self.call_function("stop")
    }

    fn unload(&self) -> Result<()> {
        self.call_function("unload")
    }
    
    fn send_message(&self, message: PluginMessage) -> Result<()> {
        if let Some(handler) = &self.message_handler {
            let message_table = self.lua.create_table()?;
            message_table.set("source", message.source)?;
            message_table.set("target", message.target)?;
            message_table.set("message_type", message.message_type)?;
            message_table.set("payload", serde_json::to_string(&message.payload)?)?;
            message_table.set("timestamp", message.timestamp.to_rfc3339())?;
            
            handler.call(message_table)
                .context("Failed to call message handler")?;
        }
        Ok(())
    }
    
    fn get_resource_usage(&self) -> Result<ResourceUsage> {
        // TODO: 实现资源使用统计
        Ok(ResourceUsage {
            memory_used: 0,
            cpu_time: 0,
            active_tasks: 0,
            api_calls: 0,
        })
    }
    
    fn set_resource_limits(&self, limits: ResourceLimits) -> Result<()> {
        *self.resource_limits.lock().unwrap() = limits;
        Ok(())
    }
    
    fn register_api(&self, name: &str, api: Box<dyn PluginApi>) -> Result<()> {
        self.apis.lock().unwrap().insert(name.to_string(), api);
        self.setup_api_bindings()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn create_test_plugin() -> (LuaRuntime, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let script_path = dir.path().join("test.lua");
        
        let script = r#"
            function init()
                print("Plugin initialized")
            end

            function start()
                print("Plugin started")
            end

            function stop()
                print("Plugin stopped")
            end

            function unload()
                print("Plugin unloaded")
            end
            
            function on_message(message)
                print("Received message: " .. message.message_type)
                -- 处理消息
                return true
            end
        "#;
        
        fs::write(&script_path, script).unwrap();
        
        let metadata = PluginMetadata {
            name: "test_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Test Author".to_string()),
            description: Some("Test Plugin".to_string()),
            homepage_url: None,
            repository_url: None,
            license: Some("MIT".to_string()),
            language: crate::plugin::PluginLanguage::Lua,
            main_file: "test.lua".to_string(),
            permissions: vec!["task.read".to_string()],
            dependencies: None,
        };
        
        let runtime = LuaRuntime::new(
            metadata,
            script_path.to_str().unwrap().to_string()
        ).unwrap();
        
        (runtime, dir)
    }

    #[test]
    fn test_lua_runtime_lifecycle() {
        let (runtime, _dir) = create_test_plugin();
        
        assert!(runtime.init().is_ok());
        assert!(runtime.start().is_ok());
        assert!(runtime.stop().is_ok());
        assert!(runtime.unload().is_ok());
    }

    #[test]
    fn test_sandbox_security() {
        let (runtime, dir) = create_test_plugin();
        let script_path = dir.path().join("malicious.lua");
        
        // 尝试访问文件系统（这应该被阻止）
        let malicious_script = r#"
            function init()
                -- 尝试访问 io 库（应该失败）
                local file = io.open("sensitive.txt", "w")
                if file then
                    file:write("Malicious content")
                    file:close()
                end
            end
        "#;
        
        fs::write(&script_path, malicious_script).unwrap();
        
        let metadata = PluginMetadata {
            name: "malicious_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Test Author".to_string()),
            description: Some("Malicious Plugin".to_string()),
            homepage_url: None,
            repository_url: None,
            license: Some("MIT".to_string()),
            language: crate::plugin::PluginLanguage::Lua,
            main_file: "malicious.lua".to_string(),
            permissions: vec![],
            dependencies: None,
        };
        
        let malicious_runtime = LuaRuntime::new(
            metadata,
            script_path.to_str().unwrap().to_string()
        ).unwrap();
        
        // 初始化应该成功，但恶意操作应该被阻止
        assert!(malicious_runtime.init().is_ok());
        
        // 验证文件没有被创建
        assert!(!dir.path().join("sensitive.txt").exists());
    }
    
    #[test]
    fn test_api_registration() {
        let (runtime, _dir) = create_test_plugin();
        
        // 创建测试 API
        struct TestApi;
        impl PluginApi for TestApi {
            fn call(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value> {
                Ok(serde_json::json!({
                    "method": method,
                    "params": params,
                }))
            }
            
            fn get_permissions(&self) -> Vec<String> {
                vec!["test.api".to_string()]
            }
        }
        
        // 注册 API
        assert!(runtime.register_api("test", Box::new(TestApi)).is_ok());
        
        // 验证 API 是否可用
        let script = r#"
            function test_api()
                local result = api.test("hello", {message = "world"})
                print(result)
                return result
            end
        "#;
        
        runtime.lua.load(script)
            .set_environment(runtime.lua.named_registry_value::<Table>("plugin_env").unwrap())
            .exec()
            .unwrap();
            
        assert!(runtime.call_function("test_api").is_ok());
    }
} 