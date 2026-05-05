use serde::Deserialize;


#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Contact {
    
    pub nb: String,

    pub name: String,
}

impl Contact {
    /// TEST
    pub fn new(nb: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            nb: nb.into(),
            name: name.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_contact_with_given_values() {
        let c = Contact::new("0612345678", "Alice");
        assert_eq!(c.nb, "0612345678");
        assert_eq!(c.name, "Alice");
    }
}
