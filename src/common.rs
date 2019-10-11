#[derive(Debug)]
pub enum Argument<'a> {
    PositionalArgument {
        idx: usize,
        name: &'a str,
        optional: bool,
    },
}
