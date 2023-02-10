pub type EhResult<T> = Result<T, ParseError>;

pub trait Parser: Sized {
    fn parse(doc: &str) -> EhResult<Self>;
}

// error

pub type BoxDynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum ParseError {
    RegexMatchFailed,
    OutOfRange,
    SignInRequired,
    AttributeNotFound(&'static str),
    DomNotFound(&'static str),
    FromServer(String),
    Invalid,
    Other(BoxDynError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::RegexMatchFailed => write!(f, "regular expression matching failed"),
            ParseError::OutOfRange => write!(f, "input is out of range"),
            ParseError::SignInRequired => write!(f, "this page requires you to log on"),
            ParseError::AttributeNotFound(s) => write!(f, "attribute `{}` cannot be found", s),
            ParseError::DomNotFound(s) => write!(f, "dom `{}` cannot be found", s),
            ParseError::FromServer(s) => write!(f, "error from server: {}", s),
            ParseError::Invalid => write!(f, "input contains invalid characters"),
            ParseError::Other(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<BoxDynError> for ParseError {
    fn from(value: BoxDynError) -> Self {
        ParseError::Other(value)
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(value: std::num::ParseIntError) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<std::num::ParseFloatError> for ParseError {
    fn from(value: std::num::ParseFloatError) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(value: chrono::ParseError) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<String> for ParseError {
    fn from(value: String) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        ParseError::Other(value.into())
    }
}
