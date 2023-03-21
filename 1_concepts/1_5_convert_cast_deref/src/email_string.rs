use std::{str::FromStr, fmt::Display, borrow::Borrow};

#[derive(Clone)]
pub struct EmailString {
    inner: String
}

impl EmailString {
    pub fn new(str_val: &str) -> Result<Self, ()> {
        // just confirm a valid(ish) email
        let mut parts = str_val.split("@");
        
        let _name = parts.next().ok_or(())?;
        let domain = parts.next().ok_or(())?;

        let tld_split_position = domain.as_bytes().iter().rposition(|&byte| byte == '.' as u8).ok_or(())?;

        let (domain_rest, tld) = domain.split_at(tld_split_position);
        if domain_rest.len() == 0 || tld.len() == 0 {
            return Err(())
        }

        Ok(
            Self {
                inner: str_val.to_string()
            }
        )
    }
}

impl FromStr for EmailString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Self::new(s)
    }
}

impl Display for EmailString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Borrow<str> for EmailString {
    fn borrow(&self) -> &str {
        &self.inner
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl TryFrom<&str> for EmailString {
    type Error = <EmailString as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<EmailString> for String {
    fn from(value: EmailString) -> Self {
        value.inner
    }
}