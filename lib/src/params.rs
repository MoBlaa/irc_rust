pub trait Parameterized<'a> {
    fn param(&self, index: usize) -> Option<&'a str>;
    /// Returns the trailing parameter which is separated from the
    /// other parameters with ' :'.
    fn trailing(&self) -> Option<&'a str>;
}
