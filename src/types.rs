use std::{fmt, io, path::Path};

//
// Trait to
//
pub trait Lookup {
    type Error: fmt::Debug + Sized;
    type Item: Sized;

    fn lookup<P>(root: P) -> Result<Self::Item, Self::Error>
    where
        P: AsRef<Path> + Copy;
}

//
// Trait for parsing help generated by [command] --help/-h
//
pub trait HelpParser {
    type Error;
    type Item;

    fn parse<R>(reader: R) -> Result<Self::Item, Self::Err>
    where
        R: io::Read;
}

pub trait CompletionGenerator {
    type Item;

    fn write<W>(&self, item: Item, writer: &mut W) -> io::Result
    where
        W: io::Write;
}