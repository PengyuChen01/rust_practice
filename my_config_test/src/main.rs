use clap::Parser;
use figment::{Figment, providers::{Env, Format, Serialized, Toml}, value::{Map, Value}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    name: String,
    comfy: bool,
    foo: i64,
}

// 这是第一层：硬编码的默认值
impl Default for MyConfig {
    fn default() -> Self {
        Self {
            name: String::from("default_user"),
            comfy: true,
            foo: 42,
        }
    }
}

/// CLI 参数定义 — 字段用 Option<T>，没传的就是 None，不会覆盖下层配置
#[derive(Debug, Parser, Serialize)]
#[command(name = "my_config_test", about = "Config layering demo")]
struct CliArgs {
    /// User name
    #[arg(long)]
    name: Option<String>,

    /// Comfy mode
    #[arg(long)]
    comfy: Option<bool>,

    /// Foo value
    #[arg(long)]
    foo: Option<i64>,
}

/// 只把 CLI 里用户实际传了的参数（Some 值）放进 map，None 的跳过
fn cli_to_figment(cli: &CliArgs) -> Map<String, Value> {
    let mut map = Map::new();
    if let Some(ref name) = cli.name {
        map.insert("name".into(), Value::from(name.clone()));
    }
    if let Some(comfy) = cli.comfy {
        map.insert("comfy".into(), Value::from(comfy));
    }
    if let Some(foo) = cli.foo {
        map.insert("foo".into(), Value::from(foo));
    }
    map
}

// 配置优先级: 默认值 < config.toml < 环境变量(MYAPP_前缀) < 命令行参数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CliArgs::parse();

    let cfg: MyConfig = Figment::new()
        .merge(Serialized::defaults(MyConfig::default()))  // 第一层：默认值
        .merge(Toml::file("config.toml"))                  // 第二层：配置文件
        .merge(Env::prefixed("MYAPP_"))                    // 第三层：环境变量
        .merge(Serialized::defaults(cli_to_figment(&cli))) // 第四层：CLI 参数（最高优先级）
        .extract()?;

    println!("My config: {cfg:?}");
    Ok(())
}
