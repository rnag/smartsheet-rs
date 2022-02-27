use serde::{Deserialize, Serialize};

/// Represents a [Contact Option] for a cell for a `CONTACT` or a `MULTI_CONTACT`
/// column in a smartsheet.
///
/// Mainly used for deserialization purposes with the `Cell::contacts` method,
/// for example to more easily retrieve the contact details (such as a list of
/// emails) from a `CONTACT` or `MULTI_CONTACT` cell.
///
/// [Contact Option]: https://smartsheet-platform.github.io/api-docs/#contactoption-object
///
#[derive(Debug, Deserialize)]
#[serde(tag = "objectType", rename = "CONTACT")]
pub struct ContactOwned {
    /// A parsable email address.
    pub email: String,
    /// Can be a user's name, display name, or free text, such as a job class or TBD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Represents a [Contact Option] for a `CONTACT` or a `MULTI_CONTACT` column in
/// a smartsheet.
///
/// [Contact Option]: https://smartsheet-platform.github.io/api-docs/#contactoption-object
///
#[derive(Serialize)]
#[serde(tag = "objectType", rename = "CONTACT")]
pub struct Contact<'a> {
    /// A parsable email address.
    pub email: &'a str,
    /// Can be a user's name, display name, or free text, such as a job class or TBD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

impl<'a> From<&'a str> for Contact<'a> {
    /// Create a new `Contact` from an *email address*
    fn from(email: &'a str) -> Self {
        Self { email, name: None }
    }
}

/// Helper trait to retrieve names and emails from a `ContactOwned` object.
pub trait ContactEmailAddrs {
    /// Returns a list of `addr` values, for instance one such as
    /// *john@example.com*.
    fn addrs(&self) -> Vec<String>;
    /// Returns a comma-separated list of `addr` values, for example like
    /// *john1@example.com, john2@example.com*.
    fn addrs_str(&self) -> String;
    /// Returns a list of [`name-addr`] values, which as indicated in the RFC
    /// will be in the format `[display-name] angle-addr` -- that is, for
    /// example, *John Doe <john@example.com>*.
    ///
    /// In the case of a missing contact name, the value will instead be the
    /// same as `addr`, so for instance like *john@example.com*.
    ///
    /// [`name-addr`]: https://www.rfc-editor.org/rfc/rfc5322#section-3.4
    fn name_addrs(&self) -> Vec<String>;
}

impl ContactEmailAddrs for Vec<ContactOwned> {
    fn addrs(&self) -> Vec<String> {
        self.iter()
            .map(|contact| contact.email.to_owned())
            .collect()
    }

    fn addrs_str(&self) -> String {
        let emails: Vec<_> = self.iter().map(|contact| contact.email.as_str()).collect();
        emails.join(", ")
    }

    fn name_addrs(&self) -> Vec<String> {
        self.iter()
            .map(|contact| match &contact.name {
                Some(name) => {
                    let mut s = String::with_capacity(name.len() + 3 + contact.email.len());
                    s.push_str(&name);
                    s.push(' ');
                    s.push('<');
                    s.push_str(&contact.email);
                    s.push('>');
                    s
                }
                None => contact.email.to_owned(),
            })
            .collect()
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
    use super::*;

    use indoc::indoc;
    use serde_json::{from_str, to_string_pretty};

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

    #[test]
    fn test_deserialize_with_name_address() {
        let input = r#"
        [
            {
              "objectType": "CONTACT",
              "email": "john1@email.com",
              "name": "My Name"
            },
            {
              "objectType": "CONTACT",
              "email": "john2@email.com",
              "name": "Other Name"
            }        
        ]
        "#;

        let contacts: Vec<ContactOwned> = from_str(input).unwrap();
        println!("Contacts: {:#?}", contacts);

        let name_addrs = contacts.name_addrs();

        assert_eq!(
            name_addrs,
            ["My Name <john1@email.com>", "Other Name <john2@email.com>"]
        );
    }
}
