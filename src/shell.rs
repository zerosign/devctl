use std::{ convert, env };

#[derive(Debug)]
pub enum ShellType {
    Zsh,
    Bash,
    Fish,
}

#[derive(Debug)]
pub enum ShellError {
    UnsupportedShell
}

impl convert::TryFrom<String> ShellType {
    type Error = ShellError;

    #[inline]
    pub fn from(p: String) -> Result<Self, Self::Error> {
        p.split("/").last().and_then(move |p| match p {
            "fish" => Ok(ShellType::Fish),
            "bash" => Ok(ShellType::Bash),
            "zsh" => Ok(ShellType::Zsh),
            _ => ShellError::UnsupportedShell,
        })
    }
}

impl ShellType {
    #[inline]
    pub fn current() -> Option<ShellType> {
        env::var(&"SHELL").and_then(ShellType::from)
    }
}
