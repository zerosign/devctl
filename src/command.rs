use std::process::Command;

#[derive(Debug)]
pub struct CommandPlugin<'a> {
    inner: &'a str
}

impl<'a> Plugin for CommandPlugin<'a> {
    fn handle(&self, envs: Vec<(String, String)>, argv: Vec<String>) -> io::Result {

        let mut (stdout, stderr) = Command::new(inner)
            .args(argv.clone())
            .envs(envs.clone())
            .output()
    }
}
