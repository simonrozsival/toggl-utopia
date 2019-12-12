#[derive(Debug)]
pub enum Error {
    ApiError(u16, String),
    NetworkError(reqwest::Error),
}

impl Error {
    pub fn code(&self) -> u16 {
        match self {
            Error::ApiError(status, _) => *status,
            Error::NetworkError(_) => 1,
        }
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        println!("From reqwest {:?}", err);
        Error::NetworkError(err)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NetworkError(ref inner) => Some(inner),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ApiError(code, msg) => write!(f, "API error {}: {}", code, msg),
            Error::NetworkError(inner) => inner.fmt(f),
        }
    }
}
