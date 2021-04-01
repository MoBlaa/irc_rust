/// Trait for everything which provides access to tags.
pub trait Taggable<'a> {
    fn tag(&self, key: &str) -> Option<&'a str>;
}
