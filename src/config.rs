use serde_derive;

pub type EnvTuple = (String, String);

#[derive(Debug, Deserialize)]
pub struct CommandConf {
    name: String,
    alias: Option<String>,
    parser: Option<String>,
    envs: Vec<EnvTuple>,,
}

#[derive(Debug, Deserialize)]
pub struct PluginConf {
    commands: Vec<CommandConf>,
}
