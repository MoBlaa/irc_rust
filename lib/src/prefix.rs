pub trait Prefixed<'a>: Sized {
    fn name(&self) -> Option<&'a str>;
    fn user(&self) -> Option<&'a str>;
    fn host(&self) -> Option<&'a str>;
    fn as_parts(&self) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>) {
        (self.name(), self.user(), self.host())
    }
}

pub type Prefix<'a> = (&'a str, Option<&'a str>, Option<&'a str>);
