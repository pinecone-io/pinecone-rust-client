/// The namespace of an index
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Namespace {
    /// The name of the namespace
    pub name: String,
}

impl From<String> for Namespace {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl From<&str> for Namespace {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
