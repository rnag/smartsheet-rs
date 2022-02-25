use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(tag = "objectType", rename = "CONTACT")]
pub struct Contact<'a> {
    pub email: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

impl<'a> From<&'a str> for Contact<'a> {
    /// Create a new `Contact` from an *email address*
    fn from(email: &'a str) -> Self {
        Self { email, name: None }
    }
}

impl<'a> Contact<'a> {
    /// Fluent setter for the `email` attribute
    ///
    /// # Note
    ///
    /// Prefer to use the `from()` method instead for creating a new `Contact`
    /// with an email.
    pub fn email(mut self, email: &'a str) -> Self {
        self.email = email;
        self
    }

    /// Fluent setter for the `name` attribute
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Contact;
    use indoc::indoc;
    use serde_json::to_string_pretty;

    #[test]
    fn test_serialize_with_empty_email() {
        let c = Contact::from("");

        let result = to_string_pretty(&c).unwrap();

        println!("{}", result);
        assert_eq!(
            result,
            indoc!(
                r#"
        {
          "objectType": "CONTACT",
          "email": ""
        }
                "#
            )
            .trim()
        )
    }

    #[test]
    fn test_serialize() {
        let c = Contact::from("my@email.com").name("My Name");

        let result = to_string_pretty(&c).unwrap();

        println!("{}", result);
        assert_eq!(
            result,
            indoc!(
                r#"
        {
          "objectType": "CONTACT",
          "email": "my@email.com",
          "name": "My Name"
        }
                "#
            )
            .trim()
        )
    }
}
