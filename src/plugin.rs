use std::io;

//
// Trait for plugin.
//
// Plugin should be able to handle forwarded arguments & could lookup
// passed environment variables.
//
trait Plugin: Sized {
    fn handle(&self, envs: Vec<(String, String)>, argv: Vec<String>) -> io::Result;
}

//
// Trait for plugin discovery in local filesystem.
//
trait PluginDiscovery {
    type Error: fmt::Debug;
    type Item: Plugin;

    fn discover(self, name: &'a str) -> Result<Self::Item, Self::Error>;
    fn list(self, pattern: &'a str) -> Iterator<Self::Item>;
}
