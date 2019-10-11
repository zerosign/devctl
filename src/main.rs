use std::{
    collections::HashMap,
    env, fmt,
    fs::DirEntry,
    io,
    path::{Path, PathBuf},
    process::Command,
};


fn main() {

    println!("shell: {:?}", ShellType::current());
    println!("devctl path: {:?}", Devctl::discover(&"/home/zerosign"));

    let devctl = Devctl::discover(&"/home/zerosign").or_else(|e| {
        e match {
            EmptyDir(path) => Devctl::create(path),
            e => Err(e)
        }
    });

    devctl.discover()
}
