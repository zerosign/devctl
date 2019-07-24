use std::path::PathBuf;

#[derive(Debug)]
pub enum DevctlErr {
    EmptyDir(PathBuf),
}
