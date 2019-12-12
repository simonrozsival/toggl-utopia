use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "API error {}: {}", _0, _1)]
    ApiError(u16, String),

    #[fail(display = "Network error: {:?}", _0)]
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
