use std::{path::{Path, PathBuf}, fs::DirEntry, process::Command};


#[derive(Debug)]
pub struct Devctl {
    root: PathBuf,
}

impl Lookup for Devctl {
    type Err = DevctlErr;
    type Item = Self;

    fn lookup<P>(root: P) -> Result<Self::Item, Self::Err>
    where
        P: AsRef<Path> + Copy,
    {
        let required = &[".devops", ".devops/bin", ".devops/conf.d"];
        let flag = required
            .iter()
            .all(move |p| Path::join(root.as_ref(), p).exists());

        if flag {
            Ok(Devctl {
                root: root.as_ref().to_path_buf(),
            })
        } else {
            Err(DevctlErr::EmptyDir(root.as_ref().to_path_buf()))
        }
    }
}


impl Devctl {

    pub fn create<P>(root: P) -> Result<Self::Item, Self::Err>
    where
        P: AsRef<Path> + Copy,
    {
        // create $HOME/.devops/{bin,conf.d}
        fs::create_dir(Path::join(root, ".devops"))?;
        fs::create_dir(Path::join(root, ".devops/bin"))?;
        fs::create_dir(Path::join(root, ".devops/conf.d"))?;

        // get current pwd and process file
        // copy self into $HOME/.devops/bin
        fs::copy(.., ...);

        // init conf.d/default.toml

        undefined!()
    }
}
