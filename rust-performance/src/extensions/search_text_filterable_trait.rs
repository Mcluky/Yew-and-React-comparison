pub trait SearchTextFilterable {
    /// Returns true if the text is contained in the object.
    /// The text is searched in the object's name, description, etc.
    /// The search is case insensitive.
    fn contains(&self, text: &str) -> bool;
}