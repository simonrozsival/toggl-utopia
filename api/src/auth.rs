#[derive(Debug, PartialEq)]
pub enum Credentials {
    UsernamePassword(String, String),
    Token(String),
}

impl Credentials {
    pub fn into_basic(self: &Self) -> (&str, &str) {
        match self {
            Credentials::UsernamePassword(username, password) => (&username, &password),
            Credentials::Token(token) => (&token, "api_token"),
        }
    }

    fn decode_base64(encoded_data: &str) -> Option<String> {
        base64::decode(encoded_data)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
    }

    fn decode_username_and_password(encoded_data: &str) -> Option<Credentials> {
        let decoded = Credentials::decode_base64(encoded_data)?;
        match decoded.split(':').collect::<Vec<_>>().as_slice() {
            [username, password] => Some(Credentials::UsernamePassword(
                username.to_string(),
                password.to_string(),
            )),
            _ => None,
        }
    }

    fn decode_token(encoded_data: &str) -> Option<Credentials> {
        let decoded = Credentials::decode_base64(encoded_data)?;
        Some(Credentials::Token(decoded))
    }

    pub fn decode(auth_header: &str) -> Option<Credentials> {
        if auth_header.starts_with("Basic ") {
            let encoded_data = &auth_header["Basic ".len()..];
            return Credentials::decode_username_and_password(&encoded_data);
        }

        if auth_header.starts_with("Bearer ") {
            let encoded_data = &auth_header["Bearer ".len()..];
            return Credentials::decode_token(&encoded_data);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::Credentials;
    use super::Credentials::{Token, UsernamePassword};

    #[test]
    fn rejects_incorrectly_formatted_geader() {
        let credentials = Credentials::decode("garbage");
        assert_eq!(credentials, None);
    }

    #[test]
    fn extracts_correct_username_and_password() {
        let header = format!("Basic {}", base64::encode("some@username.com:pass123"));
        let credentials = Credentials::decode(&header);
        assert_eq!(
            credentials,
            Some(UsernamePassword(
                String::from("some@username.com"),
                String::from("pass123")
            ))
        );
    }

    #[test]
    fn extracts_correct_token() {
        let header = format!("Bearer {}", base64::encode("some_token"));
        let credentials = Credentials::decode(&header);
        assert_eq!(credentials, Some(Token(String::from("some_token"))));
    }
}
