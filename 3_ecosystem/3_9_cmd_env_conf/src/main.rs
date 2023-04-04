mod config_structs;

use std::path::PathBuf;

use clap::Parser;
use config::{ConfigError, Config, File, Environment};
use config_structs::MyConfig;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Enables debug mode
    #[arg(short, long)]
    debug: bool,

    /// Path to configuration file [env: CONF_FILE=]
    #[arg(short, long, value_name = "conf", default_value = "config.toml")]
    config: PathBuf,
}

#[derive(Deserialize)]
struct EnvConf {
    conf_file: PathBuf
}

fn main() -> Result<(), ConfigError> {
    let mut args = Args::parse();

    if let Ok(env_conf) = envy::from_env::<EnvConf>() {
        args.config = env_conf.conf_file;
    }

    let config = Config::builder()
        .add_source(MyConfig::default())
        .add_source(File::with_name(&args.config.to_string_lossy()))
        .add_source(
            Environment::with_prefix("CONF")
        )
        .build()?;

    let res_config: MyConfig = config.try_deserialize().expect("Config to deserialize");
    println!("{:?}", res_config);

    Ok(())
}
