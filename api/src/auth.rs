#[derive(Debug, PartialEq)]
pub enum Credentials {
    UsernamePassword(String, String),
    Token(String),
}

use Credentials::{Token, UsernamePassword};

impl Credentials {
    pub fn into_basic(self: Self) -> (String, String) {
        match self {
            UsernamePassword(username, password) => (username, password),
            Token(token) => (token, "api_token".to_string()),
        }
    }

    fn decode_username_and_password(encoded_data: &str) -> Option<Credentials> {
        let decoded = base64::decode(encoded_data)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())?;

        match decoded.split(':').collect::<Vec<_>>().as_slice() {
            [username, password] => {
                Some(UsernamePassword(username.to_string(), password.to_string()))
            }
            _ => None,
        }
    }

    pub fn decode(auth_header: &str) -> Option<Credentials> {
        if auth_header.starts_with("Basic ") {
            let encoded_data = &auth_header["Basic ".len()..];
            return Credentials::decode_username_and_password(&encoded_data);
        }

        if auth_header.starts_with("Bearer ") {
            let token = &auth_header["Bearer ".len()..];
            return Some(Token(token.to_string()));
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
        let header = "Bearer some_token";
        let credentials = Credentials::decode(&header);
        assert_eq!(credentials, Some(Token(String::from("some_token"))));
    }
}
