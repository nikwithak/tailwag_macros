use std::ops::Deref;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Identifier {
    value: String,
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        return self.value.fmt(f);
    }
}

impl Identifier {
    pub fn new(value: String) -> Result<Self, &'static str> {
        if value.chars().all(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
            _ => false,
        }) {
            Ok(Self {
                value,
            })
        } else {
            Err("Contains invalid characters. [a-zA-Z0-9_] are only allowed values.")
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
